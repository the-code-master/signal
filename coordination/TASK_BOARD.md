# Signal — Parallel AI-IDE Task Board (Antigravity + Cursor)

> **Protocol:** Governed by [`architecture/12-multi-ide-collaboration.md`](../architecture/12-multi-ide-collaboration.md) and [`AGENTS.md`](../AGENTS.md).  
> **Anti-Deadlock Rule:** Parallel tasks MUST have disjoint `touched_paths` and depend only on already-merged `contracts/` commits.

---

## Current Phase: Architecture Complete (Awaiting Phase 0 Kickoff)

| Status | Milestone | Notes |
|---|---|---|
| `DONE` | **Architecture, 148-Flaw Vetting & 120-Feature Dual UI/Rust Spec** | All **17 specification docs (`01`–`17` + `README.md`)**—covering 148 resolved flaws (`F-001`–`F-148`), 12 AI-EDA innovations (`I-01`–`I-12`), and 120 dual UI+Rust extended features (`FEAT-001`–`FEAT-120`)—published and 2-pass scrubbed across `architecture/`, `AGENTS.md`, and `.cursor/rules/signal-architecture.mdc`. |
| `READY` | **Phase 0 — Foundation, `bigint` SI-Unit Contracts & Local Mac Harness** | Ready to start once the user approves kicking off Phase 0 coding. |

---

## Queued Phase 0 Backlog (Pre-Partitioned for Zero-Conflict Parallel Execution)

### Step 0A — Serialized Contract Freeze (Merge to `main` FIRST)
| Task ID | Branch | Owner | Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-01` | `contract/P0-01-core-contracts` | Antigravity or Cursor | `contracts/**` | Initialize Buf Protobuf (`signal/v1/*` with explicit SI-unit suffixes `_nm`, `_uv`, `_ps` + mandatory `[jstype = JS_STRING]` / ES2020 `bigint` on all 64-bit fields + core & extended domain protos `magnetics_power`, `emc_safety`, `dfm_harness`, `fpga_firmware`, `hw_security`, `academy_campus`), OpenAPI `control-plane.v1.yaml`, JSON schemas (including `winding-magnetics`, `emc-safety-policy`, `dfm-panel-plan`, `hw-security-audit`), `contracts/fixtures/` golden round-trip payloads, and `contracts/transcripts/*.jsonl` stateful multi-step replays | `QUEUED` |

### Step 0B — Parallel Disjoint Execution (Run Simultaneously After `P0-01` Merges)
| Task ID | Lane & Owner | Branch | Disjoint Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-02` | **Lane A (Antigravity)** | `ag/P0-02-rust-workspace-skeleton` | `engine/**`, `benchmarks/**` | Cargo workspace (`signal-proto` shared build-time codegen + `cargo-hakari`, `signal-core` with `SecretBox<T>` + Hybrid PQC + 2-level `HKDF`, `signal-geom`, `signal-graph`, `signal-component`, `signal-sim`, `signal-diff-sim`, `signal-git-sync`, `signal-cell-wasm`, `signal-pod`, plus skeleton crates for `17-*` extended domains), `cargo vet`/`deny` config, `dev-guest-auth` guard | `BLOCKED_ON_P0-01` |
| `P0-03` | **Lane B (Cursor)** | `cursor/P0-03-angular-node-skeleton` | `web/**`, `services/**` | Angular 19+ Signals IDE shell (`/org/:org_slug/project/:proj_slug`, Route-Partitioned `COOP`/`COEP`, Dual-Plane `Zstd-QOI`/WebP + `HudOverlayPrimitive`, Omni-Palette `Cmd+K`, Progressive Profiles, lazy-route registries for core + 10 extended `web/libs/*` workbenches), Node.js 22 stateless services (`api-gateway`, `identity-org`, `project-catalog`, `vcs-connector`, `vcs-auditor`, `session-broker`, `sftp-egress`, `audit-service` w/ Fail-Closed 3x Quorum Tier-0 Outbox), `@Require*` uniform-`404` decorators, build-time `bigint` TS contract codegen | `BLOCKED_ON_P0-01` |
| `P0-04` | **Lane A (Antigravity)** | `ag/P0-04-local-mac-infra` | `infra/local/**` | Native `linux/arm64` Docker Compose (`dev:full`) + `< 1.5 GB RAM` SQLite/in-proc harness (`dev:lite`, `< 2 s` incr Rust builds via `sccache`/`lld`), `local-git-mock` (with `local-guest-0001` seeded in `REPO_ACCESS_ENTRY`), Envoy edge relay, epoch-enforcing `signal-egress-proxy`, DevKMS (Hybrid PQC), Mock SFTP, headless `lavapipe` software-Vulkan WGSL check, and `GBNF` local Ollama/MLX + `deterministic_mock` offline fallback | `QUEUED` |

---

## Agent Status Journals (Single-Writer Per Agent)
- **Antigravity Journal (Antigravity writes only):** [`coordination/agents/antigravity.md`](./agents/antigravity.md)
- **Cursor Journal (Cursor writes only):** [`coordination/agents/cursor.md`](./agents/cursor.md)
- **Cross-IDE Handoffs (Append-only files):** [`coordination/handoffs/`](./handoffs/README.md)
