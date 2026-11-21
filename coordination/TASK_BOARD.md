# Signal — Parallel AI-IDE Task Board (Antigravity + Cursor)

> **Protocol:** Governed by [`architecture/12-multi-ide-collaboration.md`](../architecture/12-multi-ide-collaboration.md) and [`AGENTS.md`](../AGENTS.md).  
> **Anti-Deadlock Rule:** Parallel tasks MUST have disjoint `touched_paths` and depend only on already-merged `contracts/` commits.

---

## Current Phase: Architecture Complete (Awaiting Phase 0 Kickoff)

| Status | Milestone | Notes |
|---|---|---|
| `DONE` | **Architecture & Complete Planning** | All 13 specification docs published in `architecture/`, `AGENTS.md`, and `.cursor/rules/signal-architecture.mdc`. |
| `READY` | **Phase 0 — Foundation, Contracts & Local Mac Harness** | Ready to start once the user approves kicking off Phase 0 coding. |

---

## Queued Phase 0 Backlog (Pre-Partitioned for Zero-Conflict Parallel Execution)

### Step 0A — Serialized Contract Freeze (Merge to `main` FIRST)
| Task ID | Branch | Owner | Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-01` | `contract/P0-01-core-contracts` | Antigravity or Cursor | `contracts/**` | Initialize Buf Protobuf (`signal/v1/*`), OpenAPI `control-plane.v1.yaml`, JSON schemas (`datasheet`, `template`, `sftp-manifest`) | `QUEUED` |

### Step 0B — Parallel Disjoint Execution (Run Simultaneously After `P0-01` Merges)
| Task ID | Lane & Owner | Branch | Disjoint Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-02` | **Lane A (Antigravity)** | `ag/P0-02-rust-workspace-skeleton` | `engine/**` | Cargo workspace skeleton (`signal-core`, `signal-graph`, `signal-component`, `signal-sim`, `signal-pod`), `dev-guest-auth` compile guard, Protobuf codegen | `BLOCKED_ON_P0-01` |
| `P0-03` | **Lane B (Cursor)** | `cursor/P0-03-angular-node-skeleton` | `web/**`, `services/**` | Angular 19+ IDE shell skeleton, Node.js BFF skeleton, OpenAPI/Proto TS codegen, DLP print/storage guardrails, Local `Guest @ Signal Design, Inc.` dev banner | `BLOCKED_ON_P0-01` |
| `P0-04` | **Lane A (Antigravity)** | `ag/P0-04-local-mac-infra` | `infra/local/**` | Docker Compose for Mac (Postgres+pgvector, Redis, NATS, MinIO, DevKMS, Mock SFTP) + local Ollama/MLX LLM probe | `QUEUED` |

---

## Agent Status Journals (Single-Writer Per Agent)
- **Antigravity Journal (Antigravity writes only):** [`coordination/agents/antigravity.md`](./agents/antigravity.md)
- **Cursor Journal (Cursor writes only):** [`coordination/agents/cursor.md`](./agents/cursor.md)
- **Cross-IDE Handoffs (Append-only files):** [`coordination/handoffs/`](./handoffs/README.md)
