# Module: services/api-gateway/src/api_gateway/main.py — Unified Granian / FastAPI Control-Plane ASGI Entrypoint
# Invariants: Enforces assert_all_routes_guarded(app) before serving a single request (F-084).
# Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

import sys
from collections.abc import AsyncGenerator
from contextlib import asynccontextmanager
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
for svc_src in sorted((REPO_ROOT / "services").glob("*/src")):
    src_str = str(svc_src)
    if src_str not in sys.path:
        sys.path.insert(0, src_str)
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from audit_service.router import router as audit_router  # noqa: E402
from collab_hub.router import router as collab_router  # noqa: E402
from fastapi import FastAPI  # noqa: E402
from identity_org.router import router as identity_router  # noqa: E402
from parts_registry.router import router as parts_router  # noqa: E402
from plm_ops.router import router as plm_router  # noqa: E402
from project_catalog.router import router as catalog_router  # noqa: E402
from session_broker.router import router as session_router  # noqa: E402
from sftp_egress.router import router as sftp_router  # noqa: E402
from vcs_auditor.router import router as auditor_router  # noqa: E402
from vcs_connector.router import router as vcs_router  # noqa: E402

from api_gateway.releases import router as releases_router  # noqa: E402
from api_gateway.router import router as gateway_router  # noqa: E402
from services.common.errors import register_uniform_error_handlers  # noqa: E402
from services.common.security import assert_all_routes_guarded  # noqa: E402


@asynccontextmanager
async def _lifespan(application: FastAPI) -> AsyncGenerator[None]:
    assert_all_routes_guarded(application)
    yield


def create_app() -> FastAPI:
    application = FastAPI(
        title="Signal Tier-2A Control Plane",
        version="0.1.0",
        lifespan=_lifespan,
    )
    register_uniform_error_handlers(application)
    for rtr in (
        gateway_router,
        releases_router,
        identity_router,
        catalog_router,
        parts_router,
        plm_router,
        collab_router,
        vcs_router,
        auditor_router,
        session_router,
        sftp_router,
        audit_router,
    ):
        application.include_router(rtr)
    assert_all_routes_guarded(application)
    return application


app = create_app()
