# Signal — Parallel AI-IDE Task Board (Antigravity + Cursor)

> **Protocol:** Governed by [`architecture/12-multi-ide-collaboration.md`](../architecture/12-multi-ide-collaboration.md) and [`AGENTS.md`](../AGENTS.md).  
> **Anti-Deadlock Rule:** Parallel tasks MUST have disjoint `touched_paths` and depend only on already-merged `contracts/` commits.

---

## Current Phase: Architecture Complete (Awaiting Phase 0 Kickoff)

| Status | Milestone | Notes |
|---|---|---|
| `DONE` | **Full Architecture (21 Specs): EDA, Parts, CAD/CAM/PLM, Collab/Reviews & 3-Tier K8s** | All **21 specification docs (`01`–`21` + `README.md`)**—covering 148 resolved flaws (`F-001`–`F-148`), 12 AI-EDA innovations (`I-01`–`I-12`), 120 dual UI+Rust extended features (`FEAT-001`–`FEAT-120`), Universal Parts (`18-*`), CAD/CAM/3D-Print/PLM-Ops (`19-*`), Platform Versioning/Auto-Release Notes/Monorepo Build & 3-Tier `1–4` Extensible Rust Node K8s (`20-*`), and Native 5-Team Collaboration Space, Project Status & Automated Weekly/Monthly/Status Reviews (`21-*`)—published and verified. |
| `DONE` | **Cross-IDE Rules, Angular UI Style System, Anti-Hallucination & 19 Portable Skills** | Optimized zero-truncation `AGENTS.md` (`< 12 KB`), 6 scoped `.cursor/rules/*.mdc` rules, 19 Open-Standard `.agents/skills/*/SKILL.md` skills (Antigravity + Cursor) with 2–3 sentence coding response, anti-hallucination, and strict Angular `.component.ts`/`.html`/`.logic.ts`/`.model.ts` + centralized `@signal/design-system`/`@signal/ui-kit` rules, plus 6 guides (`docs/engineering/`). |
| `READY` | **Phase 0 — Foundation, `bigint` SI-Unit Contracts, Monorepo `justfile` & 3-Tier K8s Harness** | Ready to start once the user approves kicking off Phase 0 coding (`docs/engineering/PHASE0_LAUNCH_PLAYBOOK.md`). |

---

## Queued Phase 0 Backlog (Pre-Partitioned for Zero-Conflict Parallel Execution)

### Step 0A — Serialized Contract Freeze (Merge to `main` FIRST)
| Task ID | Branch | Owner | Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-01` | `contract/P0-01-core-contracts` | Antigravity or Cursor | `contracts/**` | Initialize Buf Protobuf (`signal/v1/*` with explicit SI-unit suffixes `_nm`, `_uv`, `_ps` + mandatory `[jstype = JS_STRING]` / ES2020 `bigint` on all 64-bit fields + `universal_parts`, `mcad_cam_print`, `plm_ops`, `platform_release`, `collab_review`, and extended domain protos), OpenAPI `control-plane.v1.yaml`, JSON schemas (including Universal Parts, MCAD/CAM/3D-Print, eBOM/mBOM, ECO/ECN/MCN/PCN/Deviation/NCR, Release Manifest, Platform Lockfile & Collab/Reviews), `contracts/fixtures/` golden round-trip payloads, and `contracts/transcripts/*.jsonl` stateful multi-step replays | `QUEUED` |

### Step 0B — Parallel Disjoint Execution (Run Simultaneously After `P0-01` Merges)
| Task ID | Lane & Owner | Branch | Disjoint Touched Paths | Deliverable | Status |
|---|---|---|---|---|---|
| `P0-02` | **Lane A (Antigravity)** | `ag/P0-02-rust-workspace-skeleton` | `engine/**`, `benchmarks/**` | Cargo workspace (`signal-proto` shared build-time codegen + `cargo-hakari`, `signal-core`, `signal-geom`, `signal-graph`, `signal-component`, `signal-parts`, `signal-sim`, `signal-diff-sim`, `signal-git-sync`, `signal-cell-wasm`, `signal-pod`, `signal-operator` multi-version reconciler, plus skeleton crates for `17-*` & `19-*` `signal-mcad`/`signal-cam`/`signal-plm`), `cargo vet`/`deny` config, `dev-guest-auth` guard | `BLOCKED_ON_P0-01` |
| `P0-03` | **Lane B (Cursor)** | `cursor/P0-03-angular-node-skeleton` | `web/**`, `services/**` | Angular 19+ Signals IDE shell (`/org/:org_slug/project/:proj_slug`, Route-Partitioned `COOP`/`COEP`, Dual-Plane `Zstd-QOI`/WebP + `HudOverlayPrimitive`, Omni-Palette `Cmd+K`, Progressive Profiles, What's New release drawer, centralized `@signal/design-system` tokens/primitives & `@signal/ui-kit` shared HTML components with ESLint/Stylelint `.html`/`.logic.ts`/no-local-CSS gates, lazy-route registries for 23 `web/libs/*` including `library-studio`, `mcad-studio`, `cam-print`, `plm-ops`, `collab-hub`), Node.js 22 stateless services (`api-gateway` w/ Release Notes API, `identity-org`, `project-catalog`, `parts-registry`, `plm-ops`, `collab-hub`, `vcs-connector`, `vcs-auditor`, `session-broker`, `sftp-egress`, `audit-service` w/ Fail-Closed 3x Quorum Tier-0 Outbox), `@Require*` uniform-`404` decorators, build-time `bigint` TS contract codegen | `BLOCKED_ON_P0-01` |
| `P0-04` | **Lane A (Antigravity)** | `ag/P0-04-monorepo-build-local-k8s` | `VERSION`, `CHANGELOG.md`, `justfile`, `releases/**`, `.github/workflows/**`, `infra/**` | Root `justfile` (`just dev`, `just dev-k8s`, `just build`, `just test`, `just release-preview`, `just deploy-local-k8s`), `VERSION` + automated Release Notes CI (`CHANGELOG.md`, `releases/notes/`, `compatibility.json`, SLSA L3/Cosign), multi-arch Dockerfiles (`infra/docker/`), Umbrella Helm Chart (`infra/k8s/helm/signal/`: separate `signal-web` pod, HA `signal-api` + Envoy `signal-engine-bridge`, `1–4` extensible Rust worker nodes + Karpenter/Cilium/Kyverno), plus native `linux/arm64` `dev:full` & `< 1.5 GB RAM` `dev:lite` Mac harness (`local-git-mock`, `signal-egress-proxy`, DevKMS, Mock SFTP, `lavapipe`, Ollama/`deterministic_mock`) | `QUEUED` |

---

## Agent Status Journals (Single-Writer Per Agent)
- **Antigravity Journal (Antigravity writes only):** [`coordination/agents/antigravity.md`](./agents/antigravity.md)
- **Cursor Journal (Cursor writes only):** [`coordination/agents/cursor.md`](./agents/cursor.md)
- **Cross-IDE Handoffs (Append-only files):** [`coordination/handoffs/`](./handoffs/README.md)
