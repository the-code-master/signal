# Signal — Canonical AI Agent Instructions (`AGENTS.md`)

> **Applies To:** **Antigravity** AND **Cursor** (loaded automatically in Cursor via [`.cursor/rules/signal-architecture.mdc`](./.cursor/rules/signal-architecture.mdc)).  
> **Single Source of Truth:** All architectural decisions live in [`architecture/`](./architecture/README.md). Never contradict or bypass them without an explicit Architecture Decision update approved by the user.

---

## 1. Current Phase & Gate

- **Current Phase:** **Architecture & Planning Complete** (Pre-Phase 0).
- **Hard Gate:** **DO NOT write production application code (`engine/`, `services/`, `web/`, `ai/`, `contracts/`, `infra/`) until the user explicitly approves starting Phase 0.**
- **Allowed Now:** Architecture refinements, contract design reviews, planning docs, coordination board updates, and repository hygiene.
- **Live Backlog:** Tracked in [`coordination/TASK_BOARD.md`](./coordination/TASK_BOARD.md).

---

## 2. Mandatory Session-Start Ritual (Every New Session)

Before modifying any file, **both Antigravity and Cursor** MUST run this sequence:

1. **Sync with `origin/main`:**
   ```bash
   git fetch origin --prune
   ```
2. **Read Coordination State (No Guessing):**
   - Check [`coordination/TASK_BOARD.md`](./coordination/TASK_BOARD.md) for active phase and queued tasks.
   - Read the peer agent's journal **read-only**:
     - Antigravity reads [`coordination/agents/cursor.md`](./coordination/agents/cursor.md).
     - Cursor reads [`coordination/agents/antigravity.md`](./coordination/agents/antigravity.md).
   - Check [`coordination/handoffs/`](./coordination/handoffs/README.md) for any new handoff notes since your last commit.
3. **Read the Governing Architecture Spec** for your task area (see Section 3).
4. **Verify Disjoint Scope & Branch First:**
   - Never overlap `touched_paths` with an active task in the peer agent's journal.
   - Never commit parallel implementation work directly to `main`; branch from `origin/main` using `ag/<task-id>-<slug>`, `cursor/<task-id>-<slug>`, or `contract/<task-id>-<slug>`.

---

## 3. Architecture Lookup Matrix (Read Before Editing)

| If Your Task Touches... | Read First |
|---|---|
| Product scope, EDA capabilities, cross-domain workflows | [`architecture/01-product-vision-and-features.md`](./architecture/01-product-vision-and-features.md) |
| System topology, tech stack, monorepo layout, SLAs | [`architecture/02-system-architecture.md`](./architecture/02-system-architecture.md) |
| Rust EDA core, `ElectronicModule` trait, MNA SPICE, PCB/IC kernels, `wgpu` | [`architecture/03-rust-eda-engine.md`](./architecture/03-rust-eda-engine.md) |
| Kubernetes Project Pods, sticky multi-user routing, CRDT, burst workers | [`architecture/04-kubernetes-runtime-and-collaboration.md`](./architecture/04-kubernetes-runtime-and-collaboration.md) |
| AI multi-agent flows, Universal Datasheet DB, Hybrid RAG, LLM gateway | [`architecture/05-ai-agent-and-datasheet-rag.md`](./architecture/05-ai-agent-and-datasheet-rag.md) |
| Angular 19+ IDE shell, Zero-Leak stream / WebGPU canvas, Node.js BFF | [`architecture/06-frontend-angular-and-nodejs-bff.md`](./architecture/06-frontend-angular-and-nodejs-bff.md) |
| Encryption, DLP (no export/print/screenshot), watermarks, 2FA SFTP | [`architecture/07-security-encryption-dlp-and-sftp.md`](./architecture/07-security-encryption-dlp-and-sftp.md) |
| Accounts, Enterprise onboarding approvals, RBAC, Multi-Type Projects, Templates | [`architecture/08-identity-enterprise-and-project-model.md`](./architecture/08-identity-enterprise-and-project-model.md) |
| macOS Apple Silicon local dev, Local LLM default, `Guest @ Signal Design, Inc.` | [`architecture/09-local-development-environment.md`](./architecture/09-local-development-environment.md) |
| Protobuf (`signal/v1`), OpenAPI, `ElectronicModule` & SFTP/Datasheet schemas | [`architecture/10-api-and-data-contracts.md`](./architecture/10-api-and-data-contracts.md) |
| Phase milestones, exit criteria, technical risk register | [`architecture/11-implementation-roadmap.md`](./architecture/11-implementation-roadmap.md) |
| Deadlock-free Antigravity + Cursor workflow, worktrees, handoffs | [`architecture/12-multi-ide-collaboration.md`](./architecture/12-multi-ide-collaboration.md) |

---

## 4. Non-Negotiable Architectural & Security Invariants

Every PR from either Antigravity or Cursor is rejected if it violates any of these hard red lines:

| Invariant | ALWAYS DO | NEVER DO |
|---|---|---|
| **Pure-Rust Engine Core** | Implement all EDA physics, `ElectronicModule` components, MNA SPICE, mixed-signal, DRC/LVS/PEX, geometry, rendering, and Gerber/GDSII generation in Rust (`engine/`). | Never implement SPICE, circuit math, DRC/LVS, routing, or manufacturing writers in Node.js or Angular. |
| **Modular Component Model** | Model every passive, active, IC, RF, digital, and subcircuit block as a composable Rust `ElectronicModule` with typed ports, tolerances, TCR/parasitics, thermal self-heating, and SOA watchdogs. | Never hardcode black-box component behaviors that bypass the unified `ElectronicModule` trait or port graph. |
| **Pod-per-Project Sticky Routing** | Route all concurrent users and AI Agents opening Project X to the **exact same active Kubernetes Rust Engine Pod** via fenced distributed leases. | Never spawn duplicate writable engine pods for the same Project or reuse a terminated pod across tenants. |
| **Zero Local Design Files** | Keep all design state in encrypted cloud storage + volatile pod RAM (`zeroize` on drop). Set `Cache-Control: no-store` on design endpoints. | Never write schematics, netlists, layouts, firmware, waveforms, or specs to `localStorage`, `sessionStorage`, `IndexedDB`, Service Workers, or disk caches. |
| **No Export / Print / Screenshot** | Enforce print blocking, dynamic visible + invisible server-burned forensic watermarks, privacy curtain, and Enterprise Zero-Leak pixel streaming. | Never add HTTP file-download, raw export, or print-view endpoints for any design artifact. |
| **2FA-Gated Authorized SFTP Only** | Deliver manufacturing/release packages exclusively via pre-registered, host-key-pinned SFTP servers protected by **fresh Step-Up 2FA (WebAuthn/TOTP)** for both Enterprise and Individual accounts, with immutable audit logging. | Never bypass Step-Up 2FA on SFTP server creation/editing/pushing, disable SSH host-key pinning, or stage plaintext release bundles on local disk. |
| **Unified Multi-Type Projects** | Store PRD, HFS, Schematic, PCB, IC (Analog/Digital/Package), FPGA, Firmware, BIOS, Software, Verification, and Mechanical Refs inside one Project with rich metadata, templates, and bi-directional traceability. | Never fragment product artifacts into isolated single-type silos lacking cross-domain `SignalEntityId` links. |
| **Grounded AI Agents** | Ground every part/pinout/limit via **Universal Datasheet DB Hybrid RAG** (respecting Global vs. Tenant-Private NDA ACLs) and validate edits in a speculative CoW Rust branch before proposing merge. | Never allow LLMs to hallucinate pinouts/ratings, mutate live user branches silently, or train models on customer IP. |
| **Hard-Guarded Local Dev Mode** | Default to macOS local LLM (Ollama / MLX) and `Guest` under `Signal Design, Inc.` **only** when `dev-guest-auth` feature + `SIGNAL_ENV=local` + loopback host are all true. | Never allow `dev-guest-auth` to compile into staging/production binaries or weaken local encryption paths. |

---

## 5. Domain Engineering Standards (Phase 0+ Implementation)

### 5.1 Rust (`engine/`)
- **Safety & Hygiene:** Prefer safe Rust (`#![forbid(unsafe_code)]` at crate level except explicitly audited SIMD/GPU/FFI leaf modules with `// SAFETY:` proofs). Zero `unwrap()` / `expect()` on untrusted input in production paths.
- **Exact Geometry & Determinism:** Use fixed-point `i64` nanometer coordinates in `signal-geom` (no floating-point drift). Simulation RNG seeds and solver ordering must be deterministic across macOS and Linux.
- **Memory Scrubbing:** Wrap all decrypted DEKs, PDKs, and sensitive project buffers with `zeroize::ZeroizeOnDrop`.
- **Verification:** `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, plus golden SPICE accuracy regressions for physics crates.

### 5.2 Angular Web UI (`web/`)
- **Architecture:** Angular 19+ Standalone Components, Signals-first reactive state (avoid heavy Zone.js churn on 60 FPS canvas interactions), strict TypeScript (`strict: true`), lazy-loaded domain workbenches under `web/libs/*`.
- **Viewport Abstraction:** Never couple domain tools directly to raw geometry transfer—all canvas interactions go through the dual-mode Viewport Adapter (`Zero-Leak WebRTC Pixel Stream` and `Standard Ephemeral WebGPU`).
- **DLP Integrity:** Preserve global print-block styles, focus-loss privacy curtains, and zero browser persistence of design entities.

### 5.3 Node.js Control Plane & BFF (`services/`)
- **Architecture:** Node.js 22 LTS + strict TypeScript (Fastify/NestJS). Keep services horizontally stateless; all real-time project affinity lives in Redis + `signal-operator`.
- **Strict Boundaries:** Generate request/response & gRPC types from `contracts/`—never hand-duplicate schemas. Apply Step-Up 2FA middleware to every SFTP management/release endpoint and emit immutable audit events to NATS JetStream.

### 5.4 AI Agents & Universal Datasheet RAG (`ai/`)
- **Model Agnostic:** Route all inference through `ai/llm-gateway` (OpenAI-compatible) supporting Local Mac Ollama/MLX, VPC vLLM/TensorRT-LLM, and zero-retention cloud APIs.
- **Tenant Isolation:** Every RAG query MUST pass explicit `(org_id, allowed_scopes)` predicates so Tenant A's private NDA datasheets are never retrievable by Tenant B.

### 5.5 Contracts (`contracts/`)
- **Single Source of Truth:** Protobuf (`contracts/proto/signal/v1/`), OpenAPI (`contracts/openapi/`), and JSON Schemas (`contracts/schemas/`).
- **Compatibility:** Additive-only changes inside `v1`; reserve removed field numbers; run `buf lint` and `buf breaking` before merging. Never hand-edit generated stubs.

---

## 6. Deadlock-Free Parallel Collaboration Protocol (Antigravity + Cursor)

Full specification: [`architecture/12-multi-ide-collaboration.md`](./architecture/12-multi-ide-collaboration.md).

### 6.1 Isolation & Branching
- **Antigravity** runs in its cloud VM (or dedicated `signal-antigravity` worktree) on branches `ag/<task-id>-<slug>`.
- **Cursor** runs in `~/Developer/signal` on macOS (or dedicated `signal-cursor` worktree) on branches `cursor/<task-id>-<slug>`.
- **Contract PRs** use `contract/<task-id>-<slug>` and MUST land on `main` BEFORE parallel implementation branches consume them.

### 6.2 Disjoint Ownership Swimlanes
| Swimlane | Default Owner | Primary Paths |
|---|---|---|
| **Lane A — Core Engine, Physics, K8s & RAG** | **Antigravity** | `engine/**`, `infra/k8s/**`, `ai/datasheet-ingest/**`, `ai/rag-service/**` |
| **Lane B — Angular IDE, Node BFF & AI UI Bridge** | **Cursor** | `web/**`, `services/**`, `ai/orchestrator/**` |
| **Shared Governed Zone (Serialized PRs Only)** | Either (sequential to `main`) | `contracts/**`, `architecture/**`, `AGENTS.md`, `.cursor/rules/**`, `infra/local/**` |

*(Lanes may be swapped per task on `TASK_BOARD.md` ONLY when active parallel tasks have completely disjoint `touched_paths`.)*

### 6.3 Zero-Conflict Coordination & Shared-File Rules
1. **Single-Writer Journals:**
   - Antigravity writes ONLY to [`coordination/agents/antigravity.md`](./coordination/agents/antigravity.md).
   - Cursor writes ONLY to [`coordination/agents/cursor.md`](./coordination/agents/cursor.md).
   - Cross-IDE handoffs go in new immutable files: `coordination/handoffs/YYYYMMDD-<task-id>-<from>-to-<to>.md`.
2. **Never Block or Poll:** Never wait for the peer IDE to finish an implementation—build and unit-test against frozen `contracts/` stubs/mocks.
3. **Database Migrations:** Always name `YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql` (UTC timestamp + agent tag; never sequential integers).
4. **Scoped Dependencies & Config:** Add dependencies to crate/package-level `Cargo.toml` / `package.json`; split `.env.example` per subsystem under `infra/local/env/`.
5. **If Blocked on a Missing Contract or Overlap:** Log `BLOCKED_ON_CONTRACT` (or `OVERLAP_DETECTED`) in your own journal, create a small `contract/*` PR or notify the user, and pivot to the next unblocked disjoint task on `TASK_BOARD.md`.

---

## 7. Definition of Done (Pre-Commit & PR Checklist)

Before committing or opening any PR, verify all applicable items:
- [ ] Rebased cleanly onto latest `origin/main` (`git fetch origin && git rebase origin/main`).
- [ ] Only declared `touched_paths` (plus your own `coordination/agents/<agent>.md`) were modified.
- [ ] Zero security/architecture invariants violated (Section 4).
- [ ] All relevant automated checks pass (Markdown link/integrity checks during Planning; `cargo clippy/test`, `pnpm lint/typecheck/test`, `buf lint/breaking` during Implementation).
- [ ] Clean working tree (no stray artifacts, `.DS_Store`, or secrets) and Conventional Commit message (`docs:`, `feat:`, `fix:`, `chore:`, `contract:`).
- [ ] Own agent journal updated with task status, branch, verification output, and any handoff notes.
