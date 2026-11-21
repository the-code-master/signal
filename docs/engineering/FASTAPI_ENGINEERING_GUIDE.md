# Signal — Python 3.15+ & FastAPI Control-Plane Engineering Guide (`services/`)

> **Applies To:** All stateless Control-Plane API services under `services/*` (`api-gateway`, `identity-org`, `project-catalog`, `parts-registry`, `plm-ops`, `collab-hub`, `vcs-connector`, `vcs-auditor`, `session-broker`, `sftp-egress`, `audit-service`) sharing one unified `uv` Python workspace with `ai/*`.  
> **Governing Specs:** [`architecture/06-frontend-angular-and-fastapi-control-plane.md`](../../architecture/06-frontend-angular-and-fastapi-control-plane.md), [`architecture/08-identity-enterprise-and-project-model.md`](../../architecture/08-identity-enterprise-and-project-model.md), [`architecture/10-api-and-data-contracts.md`](../../architecture/10-api-and-data-contracts.md), [`architecture/15-application-security-and-project-isolation.md`](../../architecture/15-application-security-and-project-isolation.md), [`architecture/20-versioning-release-monorepo-build-and-k8s-topology.md`](../../architecture/20-versioning-release-monorepo-build-and-k8s-topology.md), [`architecture/21-collaboration-space-project-status-and-automated-reviews.md`](../../architecture/21-collaboration-space-project-status-and-automated-reviews.md).

---

## 1. Canonical 2026 Python / FastAPI Runtime & Why Rust Is Kept Only Where It Shines

Signal pairs **Python `3.15+` + FastAPI (`0.165+`)** with **Rust-powered runtime primitives** in Tier 2A (`pool-api`) while reserving pure **Rust `1.100+`** for Tier 3 physics/CAD compute and security edge daemons:

| Layer | Runtime / Library (Minimum Version) | Engineering Rationale |
|---|---|---|
| **Language & Workspace** | **Python `3.15+`** (`>= 3.15.2`, lazy imports, `frozendict`, UTF-8 default) + **`uv` `0.13+`** (Rust lockfile resolver) | Unifies `services/` and `ai/` into ONE fast, auditable Python workspace (`uv.lock`, zero `pip`/virtualenv drift). |
| **Web Framework & Validation** | **FastAPI `0.165+`** + **Pydantic v2 `2.14+`** (Rust `pydantic-core` engine) | Strict type-driven DI (`Depends`), Rust-speed JSON validation/serialization, zero-drift OpenAPI 3.1 parity with `contracts/openapi/control-plane.v1.yaml`. |
| **HTTP/ASGI Server** | **Granian `2.8+`** (Rust Tokio/Hyper HTTP/1 & HTTP/2 server) | `~4×` faster than Uvicorn (`~1.5 ms` avg, `< 5 ms` p99, `~15 MB` per worker), zero Gunicorn process-manager bloat. |
| **Database Layer** | **SQLAlchemy `2.0+` Async** + **`asyncpg` / `psycopg 3`** → **PostgreSQL `19+`** (`NOBYPASSRLS`) | Native binary wire protocol + mandatory per-request RLS transaction context (`SET LOCAL app.org_id / app.user_id`). |
| **Static Analysis & Security** | **`ruff`** (Rust lint/format), **`pyright --strict` / `mypy --strict`**, **`bandit`**, **`pip-audit`**, **`pytest-asyncio`** | Catches type mismatches, injection flaws, untyped dicts, and CVEs before merge in `< 2 s`. |
| **Rust Escape Hatch (`PyO3`)** | Optional in-process **`PyO3` / `maturin`** binding to `engine/` Rust `1.100+` crates | If any CPU hotspot ever appears (e.g., bulk BOM Merkle diffing or Tardos fingerprinting), offload that single function to Rust without rewriting the API! |

**Hard Architectural Split (Zero Overlap):**
- **FastAPI (`services/*`) OWNS:** Auth/SSO/PKCE, Step-Up 2FA, Org/Project/Parts/PLM/Collab CRUD, BYO-Git orchestration, automated Weekly/Monthly/Status review compilation, RBAC, RLS, and Fail-Closed Tier-0 Audit hooks.
- **Rust `1.100+` (`engine/`) + Envoy OWN:** 100% of SPICE/EDA/MCAD/CAM/3D-Print/PLM kernels (`signal-pod`), K8s pod leases (`signal-operator`), socket-pinned anti-SSRF + epoch fencing (`signal-egress-proxy`), write-only `russh` SFTP (`signal-sftp-egress`), and zero-copy 120 FPS WebTransport/gRPC/Arrow streaming (`signal-engine-bridge` Envoy).

---

## 2. Clean 4-Layer Modular Service Structure (`services/<svc>/src/`)

Every Control-Plane service follows one strict, human-maintainable 4-layer architecture (`80–250 LOC` per file, hard split `> 350 LOC`, 3-line docstring header `Module / Invariants / Spec`, zero circular imports):

```text
services/<svc>/
├── pyproject.toml                  # Workspace-member deps pinned via root uv.lock
└── src/<svc_pkg>/
    ├── app.py                      # FastAPI lifespan, Granian ASGI entrypoint, startup route-auth self-check
    ├── routers/                    # Thin HTTP endpoints ONLY (<= 150 LOC): Depends(...) wiring, HTTP status mapping
    ├── security/                   # Fail-closed Depends guards: require_org_member, require_repo_role, require_step_up_2fa
    ├── services/                   # Pure async business orchestration (<= 250 LOC): zero SQL, zero FastAPI Request objects
    ├── repositories/               # Parameterized SQLAlchemy/asyncpg queries strictly inside with_rls_tx()
    ├── schemas/                    # Strict Pydantic v2 models (ConfigDict(extra="forbid", frozen=True)), int64 as str/int
    └── clients/                    # Typed gRPC (Rust pods/operator), signal-egress-proxy HTTPX, NATS, Redis clients
```

---

## 3. Non-Negotiable Security, RLS & Precision Invariants

1. **Fail-Closed Startup Route-Introspection Gate:**
   - Every non-health endpoint MUST declare an explicit security dependency (`Annotated[Principal, Depends(require_project_repo_role(...))]`, `require_org_admin`, or `require_step_up_2fa`).
   - `app.py` runs an automated route-table audit on startup AND in `pytest`: any route missing an explicit Signal auth dependency (or exposing `/export`, `/download`, `/raw-file`, or `/print`) **crashes boot / fails CI immediately**.
2. **Uniform `404 Not Found` Anti-Enumeration:**
   - Unauthorized org, project, sub-project, or resource UUID lookups ALWAYS raise a constant-shape `404 Not Found` (`error_code = "RESOURCE_NOT_FOUND"`) — NEVER leak `403` vs `404` resource existence!
3. **Mandatory PostgreSQL `19+` RLS (`NOBYPASSRLS`):**
   - Every DB query runs inside an async context manager `async with with_rls_tx(session_factory, principal) as tx:` executing `SET LOCAL app.org_id = :org_id; SET LOCAL app.user_id = :user_id;` under a `NOBYPASSRLS` application role.
   - Name Alembic/SQL migrations strictly `YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql` following zero-downtime Expand-and-Contract rules.
4. **64-Bit Integer Wire Fidelity (Zero JSON Precision Loss):**
   - All 64-bit contract fields (`_nm`, `_ps`, `_fs`, `fencing_epoch`, `provider_repo_id`) use Python exact `int` internally and serialize via Pydantic v2 custom annotated types as **JSON strings** (`[jstype = JS_STRING]`) so Angular `v22+` parses them directly into ES2020 `bigint` without `2^53 - 1` truncation!
5. **Auth Cookies, Replay-Proof Step-Up 2FA & Fail-Closed Tier-0 Audit:**
   - Top-level `/auth/*` PKCE/SAML redirects upgrade `5 min` `SameSite=Lax` state cookies into `__Host-signal_session` (`HttpOnly; Secure; SameSite=Strict`) + `Sec-Fetch-Site` enforcement.
   - Step-Up 2FA accepts fresh WebAuthn passkeys or Redis-burned single-use TOTP tokens (per 30s window).
   - Tier-0 security mutations (`SFTP Release`, `VCS Credentials`, `Access Revocation`, `Break-Glass`, `BYOK Edit`, `ECN/MCN/Stage-Gate Sign-Off`) commit synchronously to the **3x Quorum Audit Outbox** (`503` fail-closed if unavailable) using pseudonymous IDs (zero raw PII in WORM Merkle logs).
6. **Zero Direct Outbound Sockets, Zero Shell Injection & Secret Hygiene:**
   - Route ALL outbound GitHub/GitLab/Bitbucket/ERP/SFTP calls through socket-pinned `signal-egress-proxy` (zero direct outbound `httpx` to arbitrary user URLs).
   - Wrap secrets in Pydantic `SecretStr` / `Redacted[T]` (never log raw tokens/DEKs in `structlog`/OTel); strictly forbid `eval`, `exec`, `pickle`, `yaml.load` (use `safe_load`), `subprocess(..., shell=True)`, and any `Idempotency-Key`-less mutating endpoint.

---

## 4. Mandatory FastAPI Verification Gate (Before Every Commit/PR)

```bash
uv run ruff check services/ && uv run ruff format --check services/
uv run pyright services/
uv run pytest services/ -q
uv run bandit -r services/ -ll && uv run pip-audit --strict
```
*(Verify startup route-auth introspection test, OpenAPI schema parity against `contracts/openapi/control-plane.v1.yaml`, and golden `contracts/fixtures/` + `contracts/transcripts/` replay before opening a PR.)*
