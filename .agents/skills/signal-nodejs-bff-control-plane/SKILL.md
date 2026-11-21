---
name: signal-nodejs-bff-control-plane
description: Builds stateless Node.js 22 strict-TypeScript Control Plane and BFF services under services/* (api-gateway, identity-org, project-catalog, vcs-connector, vcs-auditor, session-broker, sftp-egress, audit-service). Use when creating REST routes, RBAC decorators, PostgreSQL RLS queries, Step-Up 2FA flows, or Fail-Closed Tier-0 audit hooks.
---

# Signal Node.js 22 Control Plane & BFF Builder (`services/**`)

Rules: `.cursor/rules/nodejs-control-plane.mdc` | Specs: `architecture/06-*`, `08-*`, `10-*`, `15-*`.

## Service File Layout (`services/<svc>/src/`)
- `main.ts` — Fastify/NestJS bootstrap, health checks, fail-closed decorator boot validator.
- `routes/*.route.ts` — URL-scoped `/v1/orgs/:org_slug/...` route registration + OpenAPI schema validation.
- `policies/*.guard.ts` — `@RequireOrgMember`, `@RequireRepoAccess`, `@RequireOrgAdmin`, `@RequireProjectAdmin`, `@RequireStepUp2FA`.
- `handlers/*.handler.ts` — Stateless orchestration (`<= 250 LOC` per file, zero physics logic).
- `repos/*.repo.ts` — Parameterized SQL wrapped inside mandatory PostgreSQL RLS transactions.

## Mandatory Security & Correctness Patterns
1. **Fail-Closed Uniform `404` Anti-Enumeration:** Every route MUST carry an explicit `@Require*` decorator (boot fails if unannotated). Unauthorized org/project/sub-project/resource UUID lookups ALWAYS return `404 Not Found` (never distinguish `403` vs `404`).
2. **PostgreSQL RLS (`NOBYPASSRLS`):** Execute every query via `withRlsTx(orgId, userId, async (tx) => ...)` issuing `SET LOCAL app.org_id = $1; SET LOCAL app.user_id = $2;`. Name migrations `YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql`.
3. **Auth Cookie Upgrade & Route-Partitioned Headers:** Top-level `/auth/*` PKCE/SAML redirects use `5 min` `SameSite=Lax` state cookies upgraded immediately on callback into `__Host-signal_session` (`HttpOnly; Secure; SameSite=Strict`) + `Sec-Fetch-Site` checks. Workspace routes set `COOP: same-origin` + `COEP: require-corp` + `Cache-Control: no-store`.
4. **Replay-Proof Step-Up 2FA:** Require fresh WebAuthn passkey or Redis-nonce-burned TOTP (single-use per 30s window) for VCS credentials, Access Review revocations, ECN/21 CFR Part 11 sign-offs, and Authorized SFTP operations.
5. **Fail-Closed 3x Quorum Tier-0 Audit Outbox:** Tier-0 mutations (`SFTP Release`, `VCS Credentials`, `Access Revocation`, `Break-Glass`, `BYOK Edit`) MUST commit synchronously to the 3-replica outbox (`503` fail-closed if unavailable) using pseudonymous IDs (zero raw PII in WORM logs).
6. **Zero Direct Outbound Sockets & `bigint` Safety:** Route all Git/SFTP outbound requests via socket-pinned `signal-egress-proxy`. Use build-time `longAsBigint: true` contract types; forbid `child_process.exec`, `eval`, raw secret logging (`Redacted<T>`), and any `/export`/`/download`/`/print` endpoint.

## Verification Commands
```bash
pnpm --filter @signal/services lint && pnpm --filter @signal/services typecheck
pnpm --filter @signal/services test --run && pnpm audit --ignore-scripts
```
