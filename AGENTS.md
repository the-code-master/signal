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
| Provider Org/Repo user auth, Org & Project Admin Access Review, Multi-Type Projects, Templates | [`architecture/08-identity-enterprise-and-project-model.md`](./architecture/08-identity-enterprise-and-project-model.md) |
| macOS Apple Silicon local dev, Local LLM default, `Guest @ Signal Design, Inc.` | [`architecture/09-local-development-environment.md`](./architecture/09-local-development-environment.md) |
| Protobuf (`signal/v1`), OpenAPI, `ElectronicModule` & SFTP/Datasheet schemas | [`architecture/10-api-and-data-contracts.md`](./architecture/10-api-and-data-contracts.md) |
| Phase milestones, exit criteria, technical risk register | [`architecture/11-implementation-roadmap.md`](./architecture/11-implementation-roadmap.md) |
| Deadlock-free Antigravity + Cursor workflow, worktrees, handoffs | [`architecture/12-multi-ide-collaboration.md`](./architecture/12-multi-ide-collaboration.md) |
| Customer-Owned BYO-Git (GitHub/GitLab/Bitbucket), Multi-Repo `<project>-<type>`, `ENCRYPTED_VAULT`, Security Auditor | [`architecture/13-vcs-git-server-and-storage-analysis.md`](./architecture/13-vcs-git-server-and-storage-analysis.md) |
| Universal scale (kid hobby → 100B GPU/ASIC), developing-nation accessibility, future plugins | [`architecture/14-universal-scale-accessibility-and-expandability.md`](./architecture/14-universal-scale-accessibility-and-expandability.md) |
| Application Security (OWASP ASVS L3), Complete Project Isolation (MicroVM/eBPF/RLS/SVID/WASM), AI & SDLC security | [`architecture/15-application-security-and-project-isolation.md`](./architecture/15-application-security-and-project-isolation.md) |
| Adversarial 112-Flaw Audit & 12 Futuristic AI-EDA Innovations (Differentiable SPICE, Formal, CPO, Twin, PQC, Super-Commit) | [`architecture/16-architectural-audit-flaws-and-futuristic-innovations.md`](./architecture/16-architectural-audit-flaws-and-futuristic-innovations.md) |

---

## 4. Non-Negotiable Architectural & Security Invariants

Every PR from either Antigravity or Cursor is rejected if it violates any of these hard red lines:

| Invariant | ALWAYS DO | NEVER DO |
|---|---|---|
| **Complete Project Isolation ("1 Project = 1 Security Cell") & AppSec** | Enforce **PostgreSQL RLS** (`NOBYPASSRLS`) + **physically partitioned per-org `pgvector` HNSW indexes**, unique **per-project DEK wrapped via Hybrid Post-Quantum `ML-KEM-768 + X25519`** + **per-project SPIFFE SVID**, dedicated **Kata/gVisor** or **`CONFIDENTIAL_SILICON` (SEV-SNP/TDX)** pod + **Hardware-Isolated NVIDIA MIG** GPU per project, **Cilium eBPF** default-deny blocking 100% of Project Pod ↔ Project Pod lateral traffic, scoped **NATS JWT/NKey** subjects, fail-closed `@Require*` auth decorators with **uniform `404` anti-enumeration**, **socket-level resolved-IP anti-SSRF** (`signal-egress-proxy`) + pinned Custom Root CAs, Angular Trusted Types + CSP nonces, `wasmtime` fuel/memory sandboxing, and bomb-capped parser sandboxes (`<= 20:1` ratio, `0` XXE, depth `<= 64`). | Never share a pod/DEK/SVID/NATS subject or time-sliced GPU across projects, never open direct Pod-to-Pod lateral sockets, never run queries without RLS context, never leak `403` vs `404` UUID existence, never allow `child_process.exec`/`eval`/`bypassSecurityTrust*`, never trust customer Git/SFTP URLs or unverified webhooks without socket-pinned anti-SSRF + HTTPS re-query, and never log unredacted secrets outside `SecretBox<T>`. |
| **Pure-Rust Differentiable & Multi-Physics Engine** | Implement all EDA physics, `ElectronicModule` components, Ruiz-equilibrated & exact `q(v)`/`Φ(i)` charge-conserving MNA SPICE, Adjoint Differentiable SPICE (`signal-diff-sim`), Formal verification (`signal-formal`), 3D EM (`signal-em3d`), 15-yr Aging (`signal-aging`), `i128`/Shewchuk + Euler/Bézier geometry (`signal-geom`), `signal-git-sync`, and Gerber/GDSII/ODB++ writers in Rust (`engine/`). | Never implement SPICE, circuit math, DRC/LVS, routing, or manufacturing writers in Node.js or Angular, and never rely on naive `C(V)dV/dt` integration or overflowing `i64 * i64` cross products. |
| **Modular Component Model** | Model every passive, active, IC, RF, photonic, digital, and subcircuit block as a composable Rust `ElectronicModule` with typed multi-physics conservation ports, analytical/autodiff Jacobians, tolerances, TCR/parasitics, thermal self-heating, and SOA watchdogs. | Never hardcode black-box component behaviors that bypass the unified `ElectronicModule` trait or port graph. |
| **Unified Multi-Repo BYO-Git, Convergent Vault & `Super-Commit`** | Under the customer's **GitHub**, **GitLab**, or **Bitbucket** namespace (`ORG_NAMESPACE` or student `USER_NAMESPACE`), map 1 Signal Project → `<project>-meta` + **1 neatly prefixed repo per Project Type** bound by immutable `provider_repo_id`. Default Enterprise repos to **Entity-Chunked Convergent `AES-256-GCM-SIV` `ENCRYPTED_VAULT`** (`(kek_id, dek_version)` headers + `MUST_ENCRYPT` taint gate), coordinate multi-repo saves via **Two-Phase `Super-Commit` (`repos.lock.json`)**, and run periodic `services/vcs-auditor` scans (70/30 rate-limit budget). | Never build a competing persistent VCS server, never retain permanent customer design files on Signal storage, never delete customer Git repos on detach, never use whole-file random-IV blobs that destroy Git packfile deduplication, and never log/return VCS tokens in plaintext. |
| **Provider Org/Repo User Auth & Admin Access Review** | Bind Dual-Identity SAML + GitHub/GitLab/Bitbucket OAuth PKCE (`provider_user_id`), allow Project/Sub-Project access **ONLY** if authorized on the provider namespace & specific `provider_repo_id` (and unexpired `expires_at`), and expose the live matrix in the **Access Review Console** for **Org Admins** and **Project Admins** (with ≥2 Super Admin 2-of-3 Quorum Break-Glass). | Never grant visibility to users absent from the Git provider's Org/Repo user list, never trust mutable usernames/emails over `provider_user_id`, and never leak unauthorized Sub-Project graph nodes or clipboard payloads inside a multi-repo Project Pod. |
| **Zero-Floor, Infinite-Ceiling Scale** | Preserve Progressive UI Profiles (`EXPLORER` / `MAKER` / `PRO` / `SILICON_SCALE`), low-bandwidth transport + `<= 60 s` volatile JS RAM reconnect buffer, `< 10 KB` declarative sim recipes, and 100B-transistor hierarchical federation via `ML-DSA`-signed Abstract Views (`*.lef.sig`, `*.lib.sig`) + `signal-federation-broker`. | Never fork separate engines for students vs. professionals or open direct lateral Pod-to-Pod sockets that violate Project Cell isolation. |
| **Pod-per-Project Linearizable Sticky Routing** | Route all concurrent authorized users and AI Agents opening Project X to the **exact same active Kubernetes Rust Engine Pod** via **linearizable Kubernetes `coordination.k8s.io/v1 Lease` fencing (`fencing_epoch`)**, supporting K8s In-Place Vertical Resize and `< 150 ms` Live Migration. | Never rely on non-linearizable cache locks alone for single-writer pod exclusivity or spawn duplicate writable engine pods. |
| **Zero Local Design Files** | Keep active design state strictly in volatile pod RAM (`zeroize` on drop) synced to customer Git. Set `Cache-Control: no-store` on design endpoints. | Never write schematics, netlists, layouts, firmware, waveforms, or specs to `localStorage`, `sessionStorage`, `IndexedDB`, Service Workers, or disk caches. |
| **No Export / Print / Screenshot** | Enforce print blocking, dynamic visible + **Tardos collusion-resistant invisible forensic watermarks**, privacy curtain, and Enterprise **hybrid `YUV 4:4:4` + lossless dirty-tile** Zero-Leak streaming. | Never add HTTP file-download, raw export, blurry `4:2:0`-only text rendering, or print-view endpoints for any design artifact. |
| **Replay-Proof 2FA SFTP, VCS & 21 CFR Part 11** | Require **fresh Step-Up 2FA** (WebAuthn or Redis-nonce-burned TOTP) for VCS credentials, Access Review revocations, and Authorized SFTP targets/releases—executed via **disposable write-only `russh` pods** (`.tmp.part` → SHA-256 verify → `rename`) with pseudonymous RFC-3161 WORM Merkle logs. | Never allow TOTP replay within a 30s window, bypass Step-Up 2FA, allow SFTP read-back (`SSH_FXP_READ`), or store non-erasable raw PII inside WORM hash chains. |
| **Unified Multi-Type Projects** | Unite PRD, HFS, Schematic, PCB, IC (Analog/Digital/Package + UPF), FPGA, Firmware, BIOS, Software, Verification, Digital Twin, and Mechanical Refs inside **one unified Signal Project** with cross-domain `SignalEntityId` traceability. | Never fragment product artifacts into isolated apps lacking cross-domain links. |
| **Neuro-Symbolic & Quarantined AI Agents** | Delegate numerical sizing to Rust Adjoint `signal-diff-sim`, ground parts/errata via **Dual-VLM Universal Datasheet Hybrid RAG**, isolate untrusted PDFs/notes via **Dual-LLM Quarantine + `GBNF` decoding**, and validate edits in speculative `jj` CoW branches before human merge. | Never let LLMs guess component math, feed untrusted external text directly into tool-calling prompts, mutate live branches silently, or train on customer IP. |
| **Hard-Guarded Zero-Bypass Local Dev Mode** | Support `pnpm dev:lite` (`< 1.5 GB RAM`) & `pnpm dev:full` on macOS with Local LLM (Ollama/MLX) and `Guest` (`local-guest-0001` seeded in `local-git-mock` `REPO_ACCESS_ENTRY` with active PQC/GCM-SIV encryption) **only** when `dev-guest-auth` + `SIGNAL_ENV=local` + loopback are all true. | Never bypass RBAC/encryption code paths in local dev or allow `dev-guest-auth` to compile into staging/production binaries. |

---

## 5. Domain Engineering Standards (Phase 0+ Implementation)

### 5.1 Rust (`engine/`)
- **Safety & Telemetry Hygiene:** Enforce `#![forbid(unsafe_code)]` at crate level (except audited SIMD/GPU/FFI leaves with `// SAFETY:` proofs + Miri/fuzz tests). Zero `unwrap()` / `expect()` on untrusted input. Wrap all secrets, tokens, DEKs, and confidential net names in `SecretBox<T>` + `zeroize::ZeroizeOnDrop`.
- **Exact Geometry & Cross-ISA Determinism:** Use `i64` nanometer coordinates with **exact `i128` cross-products / adaptive Shewchuk predicates** and native Arc/Bézier/Euler curves in `signal-geom`. Use `0.5-ULP` rounded software math (`core_math`), disabled FMA contraction where required, and canonical CSR ordering so macOS (`aarch64`) and Linux (`x86_64`) produce bit-consistent physics results.
- **Verification & Supply Chain:** `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo vet`, `cargo deny`, plus `benchmarks/` golden SPICE/EM/geometry regressions.

### 5.2 Angular Web UI (`web/`)
- **Architecture:** Angular 19+ Standalone Components, Signals-first reactive state, strict TypeScript (`strict: true`), URL-scoped `/org/:org_slug/project/:proj_slug` multi-tab context, lazy-loaded workbenches (`web/libs/*` including `visual-diff` and `review`), Okabe-Ito color-blind + stipple layer palettes, and explicit SI unit pills (`m` vs `Meg`).
- **Viewport Abstraction:** Route all canvas interactions through the Single Composited Multi-Pane GPU Atlas & dual-mode Viewport Adapter (`Hybrid YUV 4:4:4 + Lossless Dirty-Tile Stream` over `turns:443`/WebTransport and `Standard Ephemeral WebGPU`).
- **DLP Integrity:** Preserve global print-block styles, focus-loss privacy curtains, Encrypted Cloud Clipboard (`*.sigclip`), and zero browser disk persistence (`<= 60 s` volatile JS RAM closure only for reconnects).

### 5.3 Node.js Control Plane & BFF (`services/`)
- **Architecture:** Node.js 22 LTS + strict TypeScript behind Envoy zero-copy edge relay. Keep services horizontally stateless; single-writer pod fencing lives in K8s `Lease` + `signal-operator`.
- **Strict Boundaries:** Generate types at **build time** from `contracts/`—never commit hand-edited stubs. Enforce `@Require*` decorators (uniform `404`), Redis-burned Step-Up 2FA, `Idempotency-Key` support, socket-pinned `signal-egress-proxy` calls, and pseudonymous RFC-3161 WORM audit events.

### 5.4 AI Agents & Universal Datasheet RAG (`ai/`)
- **Neuro-Symbolic & Model Agnostic:** Route inference through `ai/llm-gateway` (`GBNF` schema-constrained decoding across Local Mac Ollama/MLX, Confidential VPC vLLM/TensorRT-LLM, and zero-retention cloud APIs), enforcing **Dual-LLM Quarantine** on untrusted documents and delegating numerical sizing to `signal-diff-sim`.
- **Physical Tenant Isolation:** Query `datasheet_chunks_global` plus ONLY the caller's physically partitioned `datasheet_chunks_org_<hash>` HNSW index under PostgreSQL RLS (`orderable_mpn + package + die_rev` + Silicon Errata co-retrieval).

### 5.5 Contracts (`contracts/`)
- **Single Source of Truth & SI Units:** Protobuf (`contracts/proto/signal/v1/`), OpenAPI (`contracts/openapi/`), JSON Schemas (`contracts/schemas/`), and conformance fixtures (`contracts/fixtures/`). Every physical quantity field MUST carry an explicit SI unit suffix (`_nm`, `_uv`, `_na`, `_ps`, `_mk`, `_mohm`).
- **Build-Time Codegen & Compatibility:** Additive-only changes inside `v1` (`N-1` wire support); reserve removed field numbers; run `buf lint`, `buf breaking`, and Rust↔TS fixture round-trip tests in CI. Never commit generated stubs to Git.

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
| **Lane A — Core Engine, Physics, K8s & RAG** | **Antigravity** | `engine/**`, `benchmarks/**`, `infra/k8s/**`, `ai/datasheet-ingest/**`, `ai/rag-service/**` |
| **Lane B — Angular IDE, Node BFF & AI UI Bridge** | **Cursor** | `web/**`, `services/**`, `ai/orchestrator/**` |
| **Shared Governed Zone (Serialized PRs Only)** | Either (sequential to `main`) | `contracts/**`, `architecture/**`, `AGENTS.md`, `.cursor/rules/**`, `infra/local/**` |

*(Lanes may be swapped per task on `TASK_BOARD.md` ONLY when active parallel tasks have completely disjoint `touched_paths`. Stale `IN_PROGRESS` claims with `> 24h` of commit inactivity may be reclaimed after logging a handoff note.)*

### 6.3 Zero-Conflict Coordination & Shared-File Rules
1. **Single-Writer Journals:**
   - Antigravity writes ONLY to [`coordination/agents/antigravity.md`](./coordination/agents/antigravity.md).
   - Cursor writes ONLY to [`coordination/agents/cursor.md`](./coordination/agents/cursor.md).
   - Cross-IDE handoffs go in new immutable files: `coordination/handoffs/YYYYMMDD-<task-id>-<from>-to-<to>.md`.
2. **Never Block or Poll:** Never wait for the peer IDE to finish an implementation—build and unit-test against build-time-generated `contracts/` stubs and `contracts/fixtures/` golden payloads.
3. **Database Migrations:** Always name `YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql` (UTC timestamp + agent tag; never sequential integers).
4. **Scoped Dependencies & Vetting:** Add dependencies to crate/package-level `Cargo.toml` / `package.json` (`cargo vet` + `cargo deny` + `pnpm audit --ignore-scripts`); split `.env.example` per subsystem under `infra/local/env/`.
5. **If Blocked on a Missing Contract or Overlap:** Log `BLOCKED_ON_CONTRACT` (or `OVERLAP_DETECTED`) in your own journal, create a small `contract/*` PR or notify the user, and pivot to the next unblocked disjoint task on `TASK_BOARD.md`.

---

## 7. Definition of Done (Pre-Commit & PR Checklist)

Before committing or opening any PR, verify all applicable items:
- [ ] Rebased cleanly onto latest `origin/main` (`git fetch origin && git rebase origin/main`).
- [ ] Only declared `touched_paths` (plus your own `coordination/agents/<agent>.md`) were modified.
- [ ] Zero security/architecture invariants violated (Section 4).
- [ ] All relevant automated checks pass (Markdown link/Mermaid/table/integrity checks during Planning; `cargo clippy/test/vet/deny`, `pnpm lint/typecheck/test`, `buf lint/breaking` + `contracts/fixtures/` during Implementation).
- [ ] Clean working tree (no stray artifacts, generated stubs, `.DS_Store`, or secrets) and Conventional Commit message (`docs:`, `feat:`, `fix:`, `chore:`, `contract:`).
- [ ] Own agent journal updated with task status, branch, verification output, and any handoff notes.
