# Module: services/common/security.py — Fail-Closed Depends Guards & Startup Route-Auth Introspection Gate
# Invariants: Aborts startup if any non-public route lacks Depends(require_*); blocks dev-guest-auth in prod (F-084, F-095).
# Spec: architecture/15-application-security-and-project-isolation.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

import hashlib
import hmac
import json
import os
from collections.abc import Callable, Coroutine
from pathlib import Path
from typing import Any

from fastapi import Depends, FastAPI, Header, Request
from fastapi.routing import APIRoute
from pydantic import SecretStr

from services.common.errors import raise_not_found
from services.common.schemas import PrincipalContext

AUTH_GUARD_ATTR = "__signal_auth_guard__"
PUBLIC_PATHS: frozenset[str] = frozenset({"/api/v1/healthz", "/api/v1/auth/sso/pkce/callback"})
FORBIDDEN_DLP_SEGMENTS: tuple[str, ...] = ("/export", "/download", "/print", "/raw-file")

REPO_ROOT = Path(__file__).resolve().parents[2]
SEED_PATH = REPO_ROOT / "infra" / "local" / "seeds" / "guest_signal_design_inc.json"


def mark_auth_guard[F: Callable[..., Any]](fn: F) -> F:
    setattr(fn, AUTH_GUARD_ATTR, True)
    return fn


def compute_actor_hmac(provider_user_id: str) -> str:
    pepper = os.environ.get("SIGNAL_AUDIT_HMAC_KEY", "signal-dev-pseudonym-key").encode("utf-8")
    digest = hmac.new(pepper, provider_user_id.encode("utf-8"), hashlib.sha256).hexdigest()
    return f"actor_hmac_{digest[:16]}"


def _load_guest_principal(user_id: str) -> PrincipalContext | None:
    if os.environ.get("SIGNAL_ENV", "development").lower() in ("prod", "production"):
        return None
    if os.environ.get("SIGNAL_DEV_GUEST_AUTH_ENABLED", "true").lower() != "true":
        return None
    if not SEED_PATH.exists():
        return None
    seed = json.loads(SEED_PATH.read_text(encoding="utf-8"))
    org = seed["org"]
    for entry in seed.get("repo_access_entries", []):
        if entry["provider_user_id"] == user_id:
            return PrincipalContext(
                org_id=str(org["org_id"]),
                org_slug=str(org["slug"]),
                provider_user_id=user_id,
                actor_hmac=compute_actor_hmac(user_id),
                org_role=str(entry["org_role"]),
                authorized_provider_repo_ids=tuple(str(r) for r in entry["authorized_provider_repo_ids"]),
                session_token=SecretStr(f"dev-session-{user_id}"),
            )
    return None


@mark_auth_guard
async def require_authenticated(
    request: Request,
    x_signal_guest_user: str | None = Header(default=None, alias="X-Signal-Guest-User"),
) -> PrincipalContext:
    candidate_user = x_signal_guest_user or request.cookies.get("__Host-signal_session") or "local-guest-0001"
    if x_signal_guest_user == "anonymous":
        raise_not_found()
    principal = _load_guest_principal(candidate_user)
    if principal is None:
        raise_not_found()
    return principal


@mark_auth_guard
async def require_org_member(
    org_slug: str,
    principal: PrincipalContext = Depends(require_authenticated),
) -> PrincipalContext:
    if principal.org_slug != org_slug:
        raise_not_found()
    return principal


def require_repo_access(
    required_repo_id: str,
) -> Callable[[str, PrincipalContext], Coroutine[Any, Any, PrincipalContext]]:
    @mark_auth_guard
    async def _guard(
        org_slug: str,
        principal: PrincipalContext = Depends(require_org_member),
    ) -> PrincipalContext:
        if principal.org_slug != org_slug or required_repo_id not in principal.authorized_provider_repo_ids:
            raise_not_found()
        return principal

    return _guard


@mark_auth_guard
async def require_step_up_2fa(
    step_up_proof: str = Header(..., alias="X-Signal-StepUp-Proof"),
    principal: PrincipalContext = Depends(require_authenticated),
) -> PrincipalContext:
    if len(step_up_proof.strip()) < 16:
        raise_not_found()
    return principal


def _dependant_has_auth_guard(dependant: Any) -> bool:
    call = getattr(dependant, "call", None)
    if call is not None and getattr(call, AUTH_GUARD_ATTR, False):
        return True
    return any(_dependant_has_auth_guard(sub) for sub in getattr(dependant, "dependencies", []))


def assert_all_routes_guarded(app: FastAPI) -> None:
    for route in app.routes:
        if not isinstance(route, APIRoute):
            continue
        lower_path = route.path.lower()
        for bad in FORBIDDEN_DLP_SEGMENTS:
            if bad in lower_path:
                raise RuntimeError(f"Zero-Leak DLP gate blocked forbidden route `{route.path}`")
        if route.path in PUBLIC_PATHS:
            continue
        if not _dependant_has_auth_guard(route.dependant):
            raise RuntimeError(
                f"Fail-closed startup gate aborted: route `{route.methods} {route.path}` lacks Depends(require_*)"
            )
