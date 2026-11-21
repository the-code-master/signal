# Signal — Canonical AI Agent Instructions (`AGENTS.md`)

> **Applies To:** **Antigravity** AND **Cursor** (paired with [`.cursor/rules/*.mdc`](./.cursor/rules/signal-architecture.mdc) and [`.agents/skills/*/SKILL.md`](./.agents/skills/README.md)).  
> **Single Source of Truth:** Architecture lives in [`architecture/`](./architecture/README.md); coding standards & Phase 0 launch playbooks live in [`docs/engineering/`](./docs/engineering/README.md).

---

## 1. Current Phase & Hard Gate

- **Current Phase:** **Architecture, Engineering Rules & Skills Complete** (Ready for Phase 0).
- **Hard Gate:** **DO NOT write production application code (`engine/`, `services/`, `web/`, `ai/`, `contracts/`, `infra/`) until the user explicitly approves starting Phase 0.**
- **Transition Runbook:** [`docs/engineering/PHASE0_LAUNCH_PLAYBOOK.md`](./docs/engineering/PHASE0_LAUNCH_PLAYBOOK.md) | **Backlog:** [`coordination/TASK_BOARD.md`](./coordination/TASK_BOARD.md).

---

## 2. Mandatory Session-Start Ritual (Skill: `signal-session-start`)

Before modifying any file, **both Antigravity and Cursor** MUST:
1. Run `git fetch origin --prune && git status -sb`.
2. Read [`coordination/TASK_BOARD.md`](./coordination/TASK_BOARD.md), [`coordination/handoffs/`](./coordination/handoffs/README.md), and the **peer journal read-only** (Antigravity reads [`cursor.md`](./coordination/agents/cursor.md); Cursor reads [`antigravity.md`](./coordination/agents/antigravity.md)).
3. Confirm **zero `touched_paths` overlap** with active peer tasks; branch from `origin/main` using `ag/<task-id>-<slug>`, `cursor/<task-id>-<slug>`, or serialized `contract/<task-id>-<slug>`.
4. Activate only the matching scoped rules (`.cursor/rules/*.mdc`), portable skills (`.agents/skills/*/SKILL.md`), and architecture spec(s) below.

---

## 3. Compact Architecture, Engineering & Skill Lookup Matrix

| Domain / Task Area | Read Spec / Guide | Activate Skill(s) |
|---|---|---|
| Product Vision, Unified Workspace & 120 Dual UI+Rust Features | [`01`](./architecture/01-product-vision-and-features.md), [`17`](./architecture/17-extended-feature-specifications-ui-and-rust.md) | `signal-token-efficient-editing` |
| System Topology, Monorepo, Universal Scale & Roadmap | [`02`](./architecture/02-system-architecture.md), [`11`](./architecture/11-implementation-roadmap.md), [`14`](./architecture/14-universal-scale-accessibility-and-expandability.md) | `signal-session-start` |
| Rust EDA Core (`engine/**`, `benchmarks/**`) | [`03`](./architecture/03-rust-eda-engine.md), [`RUST_ENGINEERING_GUIDE.md`](./docs/engineering/RUST_ENGINEERING_GUIDE.md) | `signal-rust-crate-architect`, `signal-rust-physics-and-geometry` |
| Angular 19/20+ Zoneless UI (`web/**`) & Zero-Leak Canvas | [`06`](./architecture/06-frontend-angular-and-nodejs-bff.md), [`ANGULAR_ENGINEERING_GUIDE.md`](./docs/engineering/ANGULAR_ENGINEERING_GUIDE.md) | `signal-angular-workbench`, `signal-zero-leak-canvas-and-dlp` |
| Node.js 22 BFF (`services/**`), Identity & Access Review | [`06`](./architecture/06-frontend-angular-and-nodejs-bff.md), [`08`](./architecture/08-identity-enterprise-and-project-model.md) | `signal-nodejs-bff-control-plane` |
| Customer BYO-Git (GitHub/GitLab/Bitbucket) & Encrypted Vault | [`13`](./architecture/13-vcs-git-server-and-storage-analysis.md), [`07`](./architecture/07-security-encryption-dlp-and-sftp.md) | `signal-byo-git-and-vault` |
| Complete Isolation, OWASP ASVS L3, K8s Pods, PQC & 148-Flaw Audit | [`04`](./architecture/04-kubernetes-runtime-and-collaboration.md), [`07`](./architecture/07-security-encryption-dlp-and-sftp.md), [`15`](./architecture/15-application-security-and-project-isolation.md), [`16`](./architecture/16-architectural-audit-flaws-and-futuristic-innovations.md) | `signal-security-isolation-audit` |
| AI Agents, Multimodal Quarantine & 3-Tier `pgvector` RAG (`ai/**`) | [`05`](./architecture/05-ai-agent-and-datasheet-rag.md) | `signal-ai-neuro-symbolic-rag` |
| Canonical Contracts (`contracts/**`: Proto, OpenAPI, Fixtures, Transcripts) | [`10`](./architecture/10-api-and-data-contracts.md) | `signal-contract-first` |
| macOS Apple Silicon Dev (`infra/local/**`) & Parallel IDE Workflow | [`09`](./architecture/09-local-development-environment.md), [`12`](./architecture/12-multi-ide-collaboration.md), [`TOKEN_EFFICIENCY_AND_CLEAN_CODE.md`](./docs/engineering/TOKEN_EFFICIENCY_AND_CLEAN_CODE.md) | `signal-local-mac-dev`, `signal-cross-ide-handoff`, `signal-verification-and-pr-gate` |

---

## 4. Non-Negotiable Architectural & Security Invariants (PR Rejection If Violated)

| Invariant | ALWAYS DO | NEVER DO |
|---|---|---|
| **1 Project = 1 Security Cell & AppSec** | PostgreSQL RLS (`NOBYPASSRLS`) + 3-Tier `pgvector` HNSW (`<= 500` enterprise partitions/cluster + 64 hash buckets), per-project Hybrid PQC (`ML-KEM-768 + X25519` + 2-level `HKDF`) DEK & SPIFFE SVID, `< 8 MB` `signal-cell-wasm` (`EXPLORER`) or Kata/`CONFIDENTIAL_SILICON` (SEV-SNP/TDX) + Exclusive Hopper `H100 CC` or Blackwell `B200+ TEE-I/O` Confidential MIG (or standard Hardware MIG), Cilium eBPF 100% lateral drop, scoped NATS JWTs, `@Require*` uniform-`404`, socket-pinned + `X-Signal-Fencing-Token` `signal-egress-proxy`, Route-Partitioned `COOP`/`COEP` + `__Host-` `SameSite=Strict` cookies, child `Firecracker`/`gVisor` IPC for `IBIS-AMI`/`OSDI` (LLVM-free `signal-pod`), bomb-capped parsers (`<= 20:1`, `0` XXE, depth `<= 64`). | Never share a pod/DEK/SVID/NATS subject or time-sliced GPU across projects, never claim simultaneous `MIG + CC` on H100, never `dlopen` vendor `.so`/`.dll` or link LLVM in `signal-pod`, never open Pod-to-Pod sockets, never leak `403` vs `404`, never allow `eval`/`child_process.exec`/`bypassSecurityTrust*`, never log secrets outside `SecretBox<T>`. |
| **Pure-Rust Physics & Exact Geometry** | Implement all EDA physics, `ElectronicModule`, charge/flux `q(v)`/`Φ(i)` Ruiz MNA (`KLU`/`CKTSO` `< 25k` ↔ `cuDSS` `>= 25k`), Filippov Saltation + `Revolve` $O(\log N_t)$ Adjoint SPICE (`signal-diff-sim`) + PSS/HB, Formal, Baseband CPO/3D EM, 15-yr Aging, Cryo-CMOS, Pelgrom/LVF/Halo IC & Metal/PCB ECO, extended domain crates (`signal-magnetics`, `signal-power-integrity`, `signal-emc`, `signal-dfm`, `signal-harness`, `signal-fpga`, `signal-security-hw`), `i128`/Shewchuk + Euler/Bézier geometry, `0.5-ULP` `core_math`, and Gerber/GDSII/IPC-2581 writers in Rust (`engine/`). | Never put SPICE, magnetics, creepage, EMC/FMEDA, DRC/LVS, routing, or manufacturing writers in Node.js/Angular; never differentiate switching discontinuities without Saltation matrices; never rely on naive `C(V)dV/dt` or overflowing `i64 * i64`. |
| **BYO-Git, Padmé Vault & Dual-Fenced `Super-Commit`** | Under customer GitHub/GitLab/Bitbucket (`ORG_NAMESPACE` / student `USER_NAMESPACE`), create `<project>-meta` + lazy `<project>-<type>` (`<= 40/min` pacer) bound by `provider_repo_id`, blobless `--filter=blob:none` clones + `64 KB` streaming LFS, Padmé-padded path-blinded (`blobs/*.sigv`) `.signal/vault.merkle.sig`-bound RFC 8452 `AES-256-GCM-SIV` (`MUST_ENCRYPT` taint gate), Dual-Fenced 2PC `Super-Commit` (proxy epoch gate + `<project>-meta` CAS first), BYOK-only Continuity WAL, Strictly In-Jurisdiction Paired-Region DR, and `vcs-auditor` (70/30 budget). | Never store permanent customer design files on Signal storage, never burst-create repos, never leak filenames/sizes or permit rollback swaps in `ENCRYPTED_VAULT`, never push without `X-Signal-Fencing-Token`, never replicate Sovereign DR across borders. |
| **Provider Auth, Zero-Leak DLP & 2FA SFTP** | Dual-bind SSO + OAuth PKCE (`provider_user_id`), gate visibility strictly on live `provider_repo_id` + unexpired `expires_at`, expose Org & Project Admin Access Review (≥2 Super Admin 2-of-3 Quorum Break-Glass). Zero browser disk design persistence (`localStorage`/`IndexedDB`/SW); zero `/export`/`/download`/`/print` routes; Dual-Plane `Zstd-QOI`/WebP + hardware `HEVC`/Dual-Luma streaming + `HudOverlayPrimitive` + Tardos & Constellation watermarks. Fresh Step-Up 2FA (WebAuthn or Redis-burned TOTP) + disposable write-only `russh` SFTP + Fail-Closed 3x Quorum Tier-0 Audit Outbox → pseudonymous RFC-3161 WORM logs. | Never bypass provider user lists, never allow TOTP replay or SFTP read-back, never drop Tier-0 security audit logs asynchronously, never store raw PII in WORM chains, never use software `AV1 4:4:4` decode. |
| **Contracts, AI Quarantine & Local Mac Dev** | Build-time-only codegen (`signal-proto` in Rust, `pnpm codegen` in TS), explicit SI suffixes (`_nm`, `_ps`, `_uv`), mandatory `[jstype = JS_STRING]` / ES2020 `bigint` on all 64-bit contract fields, `contracts/fixtures/` + `contracts/transcripts/*.jsonl`. Neuro-symbolic AI delegates math to Rust, isolates all untrusted text/PDFs/VLM images via Zero-Tool Quarantine + `GBNF`, re-verifies `base_graph_merkle_root` proofs on merge, and uses mirrored/`4×` chaff-padded BOM DB. Local dev (`dev:lite` `< 1.5 GB RAM`, `lavapipe` CI, Ollama/`deterministic_mock`, `local-guest-0001`) executes full RBAC + encryption behind `dev-guest-auth` + `SIGNAL_ENV=local` + loopback. | Never commit generated stubs, never cast 64-bit contract fields to JS `number`, never feed untrusted text/images to tool-calling LLMs, never bypass RBAC/encryption in local dev, never let `dev-guest-auth` compile into release binaries. |

---

## 5. Clean-Code & Token-Efficiency Guardrails (Every Language)

- **File & Function Size:** Keep source files `80–250 LOC` (hard split `> 350 LOC`), functions `<= 40–60 LOC`, nesting `<= 3`, parameters `<= 4`. Open every file with a 3-line `Module / Invariants / Spec` header.
- **Rust (`engine/`):** Rust 2024, `#![forbid(unsafe_code)]`, acyclic crate DAG, `signal-proto` + `cargo-hakari`, `thiserror` in libs, zero `.unwrap()` on untrusted data, `SecretBox<T>`, `core_math`, `cargo vet`/`deny`.
- **Angular (`web/`):** Angular 19/20+ Zoneless, Standalone ONLY, `OnPush`, Signal APIs (`input()`/`output()`/`computed()`), `@if`/`@for`/`@defer`, strict `@signal/<lib>` `index.ts` boundaries, `bigint` SI types, `cdk-virtual-scroll`, WCAG 2.2 AA.
- **Node.js (`services/`):** Node 22 strict TS, stateless services, fail-closed `@Require*` uniform-`404`, `withRlsTx` (`NOBYPASSRLS`), `Idempotency-Key`, `Redacted<T>`, zero physics logic.

---

## 6. Deadlock-Free Antigravity + Cursor Parallel Swimlanes

| Swimlane | Default Owner | Primary Paths | Branch Prefix |
|---|---|---|---|
| **Lane A — Rust Engine, Physics, K8s & RAG** | **Antigravity** | `engine/**`, `benchmarks/**`, `infra/k8s/**`, `ai/datasheet-ingest/**`, `ai/rag-service/**` | `ag/<task-id>-<slug>` |
| **Lane B — Angular IDE, Node BFF & AI UI** | **Cursor** | `web/**`, `services/**`, `ai/orchestrator/**` | `cursor/<task-id>-<slug>` |
| **Shared Governed Zone (Serialized PRs)** | Either (sequential to `main`) | `contracts/**`, `architecture/**`, `docs/engineering/**`, `.agents/skills/**`, `.cursor/rules/**`, `AGENTS.md`, `infra/local/**` | `contract/<task-id>-<slug>` |

- Single-writer journals (`coordination/agents/antigravity.md` vs `cursor.md`); immutable handoffs (`coordination/handoffs/YYYYMMDD-<task-id>-<from>-to-<to>.md`); UTC-timestamped DB migrations (`YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql`); `> 24h` stale claims reclaimable via handoff note.

---

## 7. Definition of Done (Skill: `signal-verification-and-pr-gate`)

- [ ] Rebased cleanly onto `origin/main` with ONLY declared `touched_paths` + own journal modified.
- [ ] Zero architectural, security, DLP, `bigint`, or file-size guardrails violated.
- [ ] All applicable automated checks green (Docs: links/Mermaid/GFM/UTF-8/`git fsck`; Contracts: `buf lint/breaking` + fixtures + transcripts; Rust: `fmt`/`clippy -D warnings`/`test`/`deny`/`vet`; Web/Node: `lint`/`typecheck`/`test`/`pnpm audit --ignore-scripts`).
- [ ] Clean working tree (zero generated stubs or secrets), Conventional Commit message, and own journal updated.
