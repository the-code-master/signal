# Signal — Parallel AI-IDE Task Board (Antigravity + Cursor)

> **Protocol:** Governed by [`architecture/12-multi-ide-collaboration.md`](../architecture/12-multi-ide-collaboration.md) and [`AGENTS.md`](../AGENTS.md).  
> **Anti-Deadlock Rule:** Parallel tasks MUST have disjoint `touched_paths` and depend only on already-merged `contracts/` commits.

---

## Current Phase: Architecture Complete (Awaiting Phase 0 Kickoff)

| Status | Milestone | Notes |
|---|---|---|
| `DONE` | **Architecture, 112-Flaw Adversarial Hardening & Planning** | All 16 specification docs (`01`–`16` + `README.md`) published and 2-pass scrubbed across `architecture/`, `AGENTS.md`, and `.cursor/rules/signal-architecture.mdc`. |
| `READY` | **Phase 0 — Foundation, SI-Unit Contracts & Local Mac Harness** | Ready to start once the user approves kicking off Phase 0 coding. |

---

## Queued Phase 0 Backlog (Pre-Partitioned for Zero-Conflict Parallel Execution)

### Step 0A — Serialized Contract Freeze (Merge to `main` FIRST)
| Task ID | Branch | Owner | Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-01` | `contract/P0-01-core-contracts` | Antigravity or Cursor | `contracts/**` | Initialize Buf Protobuf (`signal/v1/*` with explicit SI-unit suffixes `_nm`, `_uv`, `_ps` + `vcs_sync`, `formal_verify`, `em_thermal`, `digital_twin`, `import_sandbox`), OpenAPI `control-plane.v1.yaml`, JSON schemas (`datasheet`, `template`, `sftp-manifest`, `super-commit-manifest`, `upf-power-intent`), and `contracts/fixtures/` golden round-trip payloads | `QUEUED` |

### Step 0B — Parallel Disjoint Execution (Run Simultaneously After `P0-01` Merges)
| Task ID | Lane & Owner | Branch | Disjoint Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-02` | **Lane A (Antigravity)** | `ag/P0-02-rust-workspace-skeleton` | `engine/**`, `benchmarks/**` | Cargo workspace (`signal-core` with `SecretBox<T>`, `signal-geom` with `i128`/Shewchuk + curves, `signal-graph`, `signal-component`, `signal-sim`, `signal-diff-sim`, `signal-git-sync` with 2PC `Super-Commit` & convergent `AES-256-GCM-SIV` + Hybrid PQC, `signal-pod`), `cargo vet`/`deny` config, `dev-guest-auth` guard, build-time `tonic` codegen | `BLOCKED_ON_P0-01` |
| `P0-03` | **Lane B (Cursor)** | `cursor/P0-03-angular-node-skeleton` | `web/**`, `services/**` | Angular 19+ Signals IDE shell (`/org/:org_slug/project/:proj_slug`, Progressive Profiles, Okabe-Ito palettes, DLP print/storage guardrails), Node.js 22 stateless services (`api-gateway`, `identity-org`, `project-catalog`, `vcs-connector`, `vcs-auditor`, `session-broker`, `sftp-egress`, `audit-service`), `@Require*` uniform-`404` decorators, build-time TS contract codegen | `BLOCKED_ON_P0-01` |
| `P0-04` | **Lane A (Antigravity)** | `ag/P0-04-local-mac-infra` | `infra/local/**` | Native `linux/arm64` Docker Compose (`dev:full`) + `< 1.5 GB RAM` SQLite/in-proc harness (`dev:lite`), `local-git-mock` (with `local-guest-0001` seeded in `REPO_ACCESS_ENTRY`), Envoy edge relay, `signal-egress-proxy`, DevKMS (Hybrid PQC), Mock SFTP, and `GBNF` local Ollama/MLX probe | `QUEUED` |

---

## Agent Status Journals (Single-Writer Per Agent)
- **Antigravity Journal (Antigravity writes only):** [`coordination/agents/antigravity.md`](./agents/antigravity.md)
- **Cursor Journal (Cursor writes only):** [`coordination/agents/cursor.md`](./agents/cursor.md)
- **Cross-IDE Handoffs (Append-only files):** [`coordination/handoffs/`](./handoffs/README.md)
