# Signal — Parallel AI-IDE Task Board (Antigravity + Cursor)

> **Protocol:** Governed by [`architecture/12-multi-ide-collaboration.md`](../architecture/12-multi-ide-collaboration.md) and [`AGENTS.md`](../AGENTS.md).  
> **Anti-Deadlock Rule:** Parallel tasks MUST have disjoint `touched_paths` and depend only on already-merged `contracts/` commits.

---

## Current Phase: Architecture Complete (Awaiting Phase 0 Kickoff)

| Status | Milestone | Notes |
|---|---|---|
| `DONE` | **Full Architecture (21 Specs): EDA, Parts, CAD/CAM/PLM, Collab/Reviews & 3-Tier K8s** | All **21 specification docs (`01`–`21` + `README.md`)**—covering 148 resolved flaws (`F-001`–`F-148`), 12 AI-EDA innovations (`I-01`–`I-12`), 120 dual UI+Rust extended features (`FEAT-001`–`FEAT-120`), Universal Parts (`18-*`), CAD/CAM/3D-Print/PLM-Ops (`19-*`), Platform Versioning/Auto-Release Notes/Monorepo Build & 3-Tier `1–4` Extensible Rust Node K8s (`20-*`), and Native 5-Team Collaboration Space, Project Status & Automated Weekly/Monthly/Status Reviews (`21-*`)—published and verified. |
| `DONE` | **Cross-IDE Rules, Angular v22+ Style System, FastAPI Guide & 19 Portable Skills** | Optimized zero-truncation `AGENTS.md` (`< 12 KB`), 6 scoped `.cursor/rules/*.mdc` rules, 19 Open-Standard `.agents/skills/*/SKILL.md` skills (Antigravity + Cursor) enforcing **Angular `v22+` ONLY** (TS `6.0+`, Node `26 LTS+`, `pnpm 11+`), **Rust `1.100+`**, **Python `3.15+` FastAPI (`Granian 2.8+`, `Pydantic v2`, `uv`)**, **PostgreSQL `19+`**, **K8s `1.37+`**, 2–3 sentence dev replies, anti-hallucination, and strict Angular `.component.ts`/`.html`/`.logic.ts`/`.model.ts` + centralized `@signal/design-system`/`@signal/ui-kit` rules, plus 6 guides (`docs/engineering/`). |
| `READY` | **Phase 0 — Foundation, `bigint` SI-Unit Contracts, Monorepo `justfile` & 3-Tier K8s Harness** | Ready to start once the user approves kicking off Phase 0 coding (`docs/engineering/PHASE0_LAUNCH_PLAYBOOK.md`). |

---

## Queued Phase 0 Backlog (Pre-Partitioned for Zero-Conflict Parallel Execution)

### Step 0A — Serialized Contract Freeze (Merge to `main` FIRST)
| Task ID | Branch | Owner | Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-01` | `contract/P0-01-core-contracts` | Antigravity or Cursor | `contracts/**` | Initialize Buf Protobuf (`signal/v1/*` with explicit SI-unit suffixes `_nm`, `_uv`, `_ps` + mandatory `[jstype = JS_STRING]` / ES2020 `bigint` / exact Python `int` on all 64-bit fields + `universal_parts`, `mcad_cam_print`, `plm_ops`, `platform_release`, `collab_review`, and extended domain protos), OpenAPI 3.1 `control-plane.v1.yaml`, JSON schemas (including Universal Parts, MCAD/CAM/3D-Print, eBOM/mBOM, ECO/ECN/MCN/PCN/Deviation/NCR, Release Manifest, Platform Lockfile & Collab/Reviews), `contracts/fixtures/` golden round-trip payloads, and `contracts/transcripts/*.jsonl` stateful multi-step replays | `QUEUED` |

### Step 0B — Parallel Disjoint Execution (Run Simultaneously After `P0-01` Merges)
| Task ID | Lane & Owner | Branch | Disjoint Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-02` | **Lane A (Antigravity)** | `ag/P0-02-rust-workspace-skeleton` | `engine/**`, `benchmarks/**` | **Rust `1.100+`** (2024 Ed.) Cargo workspace (`signal-proto` shared build-time codegen + `cargo-hakari`, `signal-core`, `signal-geom`, `signal-graph`, `signal-component`, `signal-parts`, `signal-sim`, `signal-diff-sim`, `signal-git-sync`, `signal-cell-wasm`, `signal-pod`, `signal-operator` multi-version reconciler, `signal-egress-proxy`, `signal-sftp-egress`, plus skeleton crates for `17-*` & `19-*` `signal-mcad`/`signal-cam`/`signal-plm`), Cargo dependency cooldowns + `cargo vet`/`deny` config, `dev-guest-auth` guard | `BLOCKED_ON_P0-01` |
| `P0-03` | **Lane B (Cursor)** | `cursor/P0-03-angular-fastapi-skeleton` | `web/**`, `services/**` | **Angular `v22+` ONLY** (TS `6.0+`, Node `26 LTS+`, `pnpm 11+`) Signals IDE shell (`/org/:org_slug/project/:proj_slug`, Route-Partitioned `COOP`/`COEP`, Dual-Plane `Zstd-QOI`/WebP + `HudOverlayPrimitive`, Omni-Palette `Cmd+K`, Progressive Profiles, What's New release drawer, centralized `@signal/design-system` tokens/primitives & `@signal/ui-kit` shared HTML components with ESLint/Stylelint `.html`/`.logic.ts`/no-local-CSS gates, lazy-route registries for 23 `web/libs/*` including `library-studio`, `mcad-studio`, `cam-print`, `plm-ops`, `collab-hub`); **Python `3.15+` FastAPI (`Granian 2.8+`, `Pydantic v2`, `uv`)** stateless services (`api-gateway` w/ Release Notes API, `identity-org`, `project-catalog`, `parts-registry`, `plm-ops`, `collab-hub`, `vcs-connector`, `vcs-auditor`, `session-broker`, `sftp-egress`, `audit-service` w/ Fail-Closed 3x Quorum Tier-0 Outbox), startup `Depends(require_*)` uniform-`404` introspection gate, PostgreSQL `19+` `with_rls_tx`, and build-time Rust/Python/TS `bigint` contract parity | `BLOCKED_ON_P0-01` |
| `P0-04` | **Lane A (Antigravity)** | `ag/P0-04-monorepo-build-local-k8s` | `VERSION`, `CHANGELOG.md`, `signal.sh`, `scripts/**`, `justfile`, `releases/**`, `.github/workflows/**`, `infra/**` | Unified **`./signal.sh` → stdlib Python `3.15+` `scripts/signal_ops.py` + `scripts/components.toml`** (`doctor`, `dev`, `build`, `check`, `image`, `release`, `deploy`, `rollback` w/ `--changed`, `--dry-run`, `--json` + thin `justfile` aliases & thin `~30 LOC` `.github/workflows/*`), `VERSION` + automated Release Notes (`CHANGELOG.md`, `releases/notes/`, `compatibility.json`, SLSA L3/Cosign), multi-arch Dockerfiles (`infra/docker/`), Umbrella Helm Chart (`infra/k8s/helm/signal/`: K8s `1.37+`, separate `signal-web` Angular `v22+`/Node `26` pod, HA FastAPI/Granian `signal-api` + Envoy `signal-engine-bridge`, `1–4` extensible Rust `1.100+` worker nodes + Karpenter/Cilium/Kyverno), plus native `linux/arm64` `dev:full` & `< 1.5 GB RAM` `dev:lite` Mac harness (`local-git-mock`, `signal-egress-proxy`, DevKMS, Mock SFTP, `lavapipe`, Ollama/`deterministic_mock`) | `QUEUED` |

---

## Agent Status Journals (Single-Writer Per Agent)
- **Antigravity Journal (Antigravity writes only):** [`coordination/agents/antigravity.md`](./agents/antigravity.md)
- **Cursor Journal (Cursor writes only):** [`coordination/agents/cursor.md`](./agents/cursor.md)
- **Cross-IDE Handoffs (Append-only files):** [`coordination/handoffs/`](./handoffs/README.md)
