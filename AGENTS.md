# Signal — Canonical AI Agent Instructions (`AGENTS.md`)

> **Applies to:** **Antigravity** AND **Cursor** (referenced by `.cursor/rules/signal-architecture.mdc`).  
> **Single Source of Truth:** All architectural decisions live in [`architecture/`](./architecture/README.md). Do NOT contradict or bypass them without an explicit Architecture Decision update approved by the user.

---

## 1. Current Project Status

- **Phase:** Architecture & Planning Complete — **DO NOT write production application code until the user explicitly initiates Phase 0.**
- **Master Index:** Start by reading [`architecture/README.md`](./architecture/README.md).

---

## 2. Non-Negotiable Architectural & Security Invariants

Every future implementation PR from either Antigravity or Cursor MUST uphold these hard invariants:

1. **Stack Separation:**
   - **Core EDA & Physics Engine (`engine/`):** 100% **Rust**. Every passive and active electronic component is a composable Rust module (`ElectronicModule` trait) with typed ports, tolerances, parasitics, thermal model, and SOA limits, solved via Acausal MNA SPICE + Event-Driven Signal-Flow ([`architecture/03-rust-eda-engine.md`](./architecture/03-rust-eda-engine.md)).
   - **Web UI (`web/`):** **Angular 19+** (Standalone + Signals + strict TypeScript).
   - **Control Plane & BFF (`services/`):** **Node.js (TypeScript)** — auth, SSO, 2FA, Enterprise onboarding, project catalog/templates, session broker, SFTP egress. Never implement physics/SPICE/DRC in Node.js.
2. **Kubernetes Pod-per-Project & Sticky Multi-User Routing:**
   - 1 active Rust engine pod per loaded Project. Multiple concurrent users + AI Agents opening the same project MUST route to the **exact same pod** ([`architecture/04-kubernetes-runtime-and-collaboration.md`](./architecture/04-kubernetes-runtime-and-collaboration.md)).
3. **Zero-Trust Cloud-Only IP Protection ([`architecture/07-security-encryption-dlp-and-sftp.md`](./architecture/07-security-encryption-dlp-and-sftp.md)):**
   - **ZERO local design copies:** Never persist schematics, netlists, layouts, firmware, waveforms, or specs in `localStorage`, `sessionStorage`, `IndexedDB`, Service Workers, or client downloads.
   - **ZERO export/download/print endpoints:** Do not add raw file download routes or print views.
   - **Full encryption:** AES-256-GCM envelope encryption at rest (KMS/BYOK) and RAM-only decryption (`zeroize`) inside K8s pods.
   - **Authorized SFTP-Only Egress with Mandatory 2FA:** Every SFTP server registration/edit/release push (Enterprise AND Individual) MUST enforce fresh Step-Up 2FA (WebAuthn/TOTP), SSH host-key pinning, and immutable audit logging.
4. **Unified Multi-Type Projects ([`architecture/08-identity-enterprise-and-project-model.md`](./architecture/08-identity-enterprise-and-project-model.md)):**
   - A single Project holds rich metadata, supports reusable Project Templates, and contains multiple typed Sub-Projects (`PRD`, `HFS`, `SCHEMATIC`, `PCB`, `IC_ANALOG`, `IC_DIGITAL`, `IC_PACKAGE`, `FPGA`, `FIRMWARE`, `BIOS`, `SOFTWARE`, `VERIFICATION`, `MECHANICAL_REF`).
5. **AI-Agent-First Grounding ([`architecture/05-ai-agent-and-datasheet-rag.md`](./architecture/05-ai-agent-and-datasheet-rag.md)):**
   - AI agents MUST query the **Universal Datasheet Database (Hybrid RAG)** for specs/pinouts and verify every circuit/layout mutation against the Rust engine inside a speculative CoW branch before proposing merges.
6. **Local Mac Development Mode ([`architecture/09-local-development-environment.md`](./architecture/09-local-development-environment.md)):**
   - Default local LLM: Apple Silicon local runtime (Ollama / MLX).
   - Default local user: `Guest` under organization `Signal Design, Inc.` — strictly compile-guarded (`dev-guest-auth`) and runtime-guarded (`SIGNAL_ENV=local` + loopback) so it can NEVER activate in staging or production.

---

## 3. Deadlock-Free Parallel Collaboration Protocol (Antigravity + Cursor)

Both AI IDEs MUST strictly follow [`architecture/12-multi-ide-collaboration.md`](./architecture/12-multi-ide-collaboration.md):

1. **Physical Isolation:**
   - **Antigravity** works on branches prefixed `ag/<task-id>-<slug>` (e.g., in cloud VM or dedicated `signal-antigravity` worktree).
   - **Cursor** works on branches prefixed `cursor/<task-id>-<slug>` (e.g., in `~/Developer/signal` on macOS or dedicated `signal-cursor` worktree).
   - NEVER edit directly on `main` during parallel development; rebase on `origin/main` before opening/merging PRs.
2. **Contract-First Sequencing (Zero Circular Wait):**
   - Changes to `contracts/` (`proto/`, `openapi/`, `schemas/`) land on `main` FIRST via small `contract/<task-id>-<slug>` PRs before parallel implementation branches consume them.
   - NEVER block/sleep waiting for the peer AI IDE — build against frozen contracts/stubs.
3. **Disjoint Ownership Swimlanes & Non-Overlapping Paths:**
   - Check [`coordination/TASK_BOARD.md`](./coordination/TASK_BOARD.md) before claiming a task.
   - Default Lane A (**Antigravity**): `engine/`, `infra/k8s/`, `ai/datasheet-ingest/`, `ai/rag-service/`.
   - Default Lane B (**Cursor**): `web/`, `services/`, `ai/orchestrator/` (UI bridge).
   - Active parallel tasks MUST declare disjoint `touched_paths` globs.
4. **Lock-Free Per-Agent Journals (Zero Coordination Merge Conflicts):**
   - **Antigravity** writes ONLY to [`coordination/agents/antigravity.md`](./coordination/agents/antigravity.md) (read-only for Cursor).
   - **Cursor** writes ONLY to [`coordination/agents/cursor.md`](./coordination/agents/cursor.md) (read-only for Antigravity).
   - Cross-IDE handoff notes go in new immutable files under `coordination/handoffs/YYYYMMDD-<id>.md`.
5. **Contention-Free Migrations & Shared Files:**
   - Database migrations MUST use `YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql` (never sequential integers).
   - Keep dependencies scoped to individual crates/packages rather than editing shared files unnecessarily.
