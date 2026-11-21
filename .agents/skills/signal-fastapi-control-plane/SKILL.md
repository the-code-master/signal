---
name: signal-fastapi-control-plane
description: Builds stateless Python 3.15+ FastAPI (Granian 2.8+, Pydantic v2) Control Plane services under services/* (api-gateway, identity-org, project-catalog, parts-registry, plm-ops, collab-hub, vcs-connector, vcs-auditor, session-broker, sftp-egress, audit-service). Use when creating REST routes, fail-closed Depends RBAC guards, PostgreSQL 19+ RLS queries, Step-Up 2FA flows, or Fail-Closed Tier-0 audit hooks.
---

# Signal Python 3.15+ FastAPI Control Plane Builder (`services/**`)

Guide: `docs/engineering/FASTAPI_ENGINEERING_GUIDE.md` | Rules: `.cursor/rules/fastapi-control-plane.mdc` | Specs: `architecture/06-*`, `08-*`, `10-*`, `15-*`, `18-*`, `19-*`, `20-*`, `21-*`.

## Stack & Service File Layout (`services/<svc>/src/<pkg>/`)
- **Runtime Floor:** **Python `3.15+`** (`>= 3.15.2`), **FastAPI `0.165+`**, **Pydantic v2 `2.14+`** (Rust `pydantic-core`), **Granian `2.8+`** (Rust ASGI/RSGI server), **SQLAlchemy `2.0+` Async + `asyncpg`/`psycopg 3`** → **PostgreSQL `19+`**, **`uv` `0.13+`** (shared workspace with `ai/`), **`ruff`**, **`pyright --strict`**.
- `app.py` — FastAPI lifespan, Granian ASGI entrypoint, and automated startup route-auth introspection validator.
- `routers/*.py` — Thin URL-scoped `/v1/orgs/{org_slug}/...` endpoints (`<= 150 LOC`) declaring `Depends(...)` guards & response models.
- `security/*.py` — Fail-closed `Depends` guards (`require_org_member`, `require_project_repo_role`, `require_org_admin`, `require_project_admin`, `require_step_up_2fa`).
- `services/*.py` — Pure async domain orchestration (`<= 250 LOC`, zero SQL, zero physics logic; optional `PyO3` bridge for approved Rust CPU hotspots).
- `repositories/*.py` — Parameterized SQLAlchemy/asyncpg queries strictly wrapped inside `async with with_rls_tx(...)`.
- `schemas/*.py` — Immutable strict Pydantic v2 models (`ConfigDict(extra="forbid", frozen=True)`) serializing 64-bit SI/ID ints as JSON strings (`[jstype = JS_STRING]`).

## Mandatory Security & Correctness Patterns
1. **Fail-Closed Uniform `404` Anti-Enumeration:** Every non-health route MUST declare an explicit `Depends(require_*)` guard (startup introspection + CI fail if unannotated). Unauthorized org/project/sub-project/resource UUID lookups ALWAYS return constant-shape `404 Not Found`.
2. **PostgreSQL `19+` RLS (`NOBYPASSRLS`):** Execute every query via `async with with_rls_tx(session_factory, principal) as tx:` issuing `SET LOCAL app.org_id = :org_id; SET LOCAL app.user_id = :user_id;`. Name migrations `YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql`.
3. **Auth Cookie Upgrade & Route-Partitioned Headers:** Top-level `/auth/*` PKCE/SAML redirects use `5 min` `SameSite=Lax` state cookies upgraded immediately on callback into `__Host-signal_session` (`HttpOnly; Secure; SameSite=Strict`) + `Sec-Fetch-Site` checks. Workspace routes set `COOP: same-origin` + `COEP: require-corp` + `Cache-Control: no-store`.
4. **Replay-Proof Step-Up 2FA:** Require fresh WebAuthn passkey or Redis-nonce-burned TOTP (single-use per 30s window) for VCS credentials, Access Review revocations, ECN/MCN/Stage-Gate/21 CFR Part 11 sign-offs, and Authorized SFTP operations.
5. **Fail-Closed 3x Quorum Tier-0 Audit Outbox:** Tier-0 mutations (`SFTP Release`, `VCS Credentials`, `Access Revocation`, `Break-Glass`, `BYOK Edit`) MUST commit synchronously to the 3-replica outbox (`503` fail-closed if unavailable) using pseudonymous IDs (zero raw PII in WORM logs).
6. **Zero Direct Outbound Sockets, Exact 64-Bit Wire Parity & Hygiene:** Route all Git/SFTP/ERP outbound requests via socket-pinned Rust `signal-egress-proxy`; enforce `Idempotency-Key` on mutations; verify OpenAPI 3.1 parity against `contracts/openapi/control-plane.v1.yaml`; forbid `eval`, `exec`, `pickle`, `yaml.load`, `subprocess(..., shell=True)`, raw secret logging (`SecretStr`/`Redacted[T]`), and any `/export`/`/download`/`/print` endpoint.

## Verification Commands
```bash
uv run ruff check services/ && uv run ruff format --check services/
uv run pyright services/ && uv run pytest services/ -q
uv run bandit -r services/ -ll && uv run pip-audit --strict
```

## Dev Response & Anti-Hallucination
- Verify route paths, `Depends` guards, SQL columns, Pydantic v2 schemas, and `pyproject.toml` deps via `rg`/`view_file` before editing; never claim tests/lint pass without observed exit code `0`.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and verification result.
