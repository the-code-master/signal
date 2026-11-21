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
| Product Vision, 120 Dual Features & Integrated CAD/CAM/3D-Print/PLM-Ops | [`01`](./architecture/01-product-vision-and-features.md), [`17`](./architecture/17-extended-feature-specifications-ui-and-rust.md), [`19`](./architecture/19-cad-cam-3d-printing-and-plm-operations.md) | `signal-token-efficient-editing`, `signal-cad-cam-plm-ops` |
| System Topology, 3-Tier K8s (`1–4` Extensible Rust Nodes), Versioning, Monorepo Build/Deploy & Roadmap | [`02`](./architecture/02-system-architecture.md), [`11`](./architecture/11-implementation-roadmap.md), [`14`](./architecture/14-universal-scale-accessibility-and-expandability.md), [`20`](./architecture/20-versioning-release-monorepo-build-and-k8s-topology.md) | `signal-session-start`, `signal-release-deploy-topology` |
| Rust EDA Core (`engine/**`, `benchmarks/**`) | [`03`](./architecture/03-rust-eda-engine.md), [`RUST_ENGINEERING_GUIDE.md`](./docs/engineering/RUST_ENGINEERING_GUIDE.md) | `signal-rust-crate-architect`, `signal-rust-physics-and-geometry` |
| Angular 19/20+ Zoneless UI (`web/**`) & Zero-Leak Canvas | [`06`](./architecture/06-frontend-angular-and-nodejs-bff.md), [`ANGULAR_ENGINEERING_GUIDE.md`](./docs/engineering/ANGULAR_ENGINEERING_GUIDE.md) | `signal-angular-workbench`, `signal-zero-leak-canvas-and-dlp` |
| Node.js 22 BFF (`services/**`), Identity & Access Review | [`06`](./architecture/06-frontend-angular-and-nodejs-bff.md), [`08`](./architecture/08-identity-enterprise-and-project-model.md) | `signal-nodejs-bff-control-plane` |
| Customer BYO-Git (GitHub/GitLab/Bitbucket) & Encrypted Vault | [`13`](./architecture/13-vcs-git-server-and-storage-analysis.md), [`07`](./architecture/07-security-encryption-dlp-and-sftp.md) | `signal-byo-git-and-vault` |
| Complete Isolation, OWASP ASVS L3, K8s Pods, PQC & 148-Flaw Audit | [`04`](./architecture/04-kubernetes-runtime-and-collaboration.md), [`07`](./architecture/07-security-encryption-dlp-and-sftp.md), [`15`](./architecture/15-application-security-and-project-isolation.md), [`16`](./architecture/16-architectural-audit-flaws-and-futuristic-innovations.md) | `signal-security-isolation-audit` |
| AI Agents, Multimodal Quarantine, 3-Tier `pgvector` RAG & Universal Parts (`ai/**`) | [`05`](./architecture/05-ai-agent-and-datasheet-rag.md), [`18`](./architecture/18-universal-component-and-datasheet-repository.md) | `signal-ai-neuro-symbolic-rag`, `signal-universal-parts-registry` |
| Canonical Contracts (`contracts/**`: Proto, OpenAPI, Fixtures, Transcripts) | [`10`](./architecture/10-api-and-data-contracts.md) | `signal-contract-first` |
| macOS Apple Silicon Dev (`infra/local/**`) & Parallel IDE Workflow | [`09`](./architecture/09-local-development-environment.md), [`12`](./architecture/12-multi-ide-collaboration.md), [`TOKEN_EFFICIENCY_AND_CLEAN_CODE.md`](./docs/engineering/TOKEN_EFFICIENCY_AND_CLEAN_CODE.md) | `signal-local-mac-dev`, `signal-cross-ide-handoff`, `signal-verification-and-pr-gate` |

---

## 4. Non-Negotiable Architectural & Security Invariants (PR Rejection If Violated)

| Invariant | ALWAYS DO | NEVER DO |
|---|---|---|
| **1 Project = 1 Security Cell, 3-Tier K8s & AppSec** | Separate `pool-web` (Angular+Node pod), HA `pool-api` (`signal-api-gateway` + Envoy `signal-engine-bridge`), & `1–4 -> N` extensible `pool-rust-engine` worker nodes hosting isolated `< 8 MB` `signal-cell-wasm` or Kata/`CONFIDENTIAL_SILICON` (SEV-SNP/TDX) + Exclusive `H100 CC` / Blackwell `B200+ TEE-I/O` MIG cells; Postgres RLS (`NOBYPASSRLS`) + 3-Tier `pgvector` HNSW, Hybrid PQC (`ML-KEM-768` + 2-level `HKDF`) DEK & SPIFFE SVID, Cilium 100% lateral drop, scoped NATS JWTs, `@Require*` uniform-`404`, socket-pinned + `X-Signal-Fencing-Token` `signal-egress-proxy`, Route-Partitioned `COOP`/`COEP`, child MicroVM IPC for `IBIS-AMI`/`OSDI` (LLVM-free `signal-pod`), bomb-capped parsers (`<= 20:1`, depth `<= 64`). | Never share pod/DEK/SVID/NATS/time-sliced GPU, let `signal-web` dial Rust/DB, proxy 120 FPS pixels via Node JSON, claim `MIG + CC` on H100, `dlopen`/link LLVM in `signal-pod`, leak `403` vs `404`, use `eval`/`bypassSecurityTrust*`, or log outside `SecretBox<T>`. |
| **Pure-Rust Physics, CAD/CAM & PLM** | All EDA/MCAD/CAM/3D-Print/PLM kernels (`ElectronicModule`, `q(v)`/`Φ(i)` Ruiz MNA `KLU`↔`cuDSS`, Saltation + `Revolve` Adjoint + PSS/HB, Formal, Baseband CPO/3D EM, Aging, Cryo, Pelgrom/LVF/Halo, ECO, `signal-{magnetics,power-integrity,emc,dfm,harness,fpga,security-hw,parts,mcad,cam,plm}`, `i128`/Shewchuk, `0.5-ULP` `core_math`, Gerber/GDSII/STEP/G-code/3MF) in Rust (`engine/`). | Never put SPICE, magnetics, creepage, EMC/FMEDA, CAD/CAM/slicing, DRC/LVS, routing, or manufacturing writers in Node/Angular; never differentiate switches without Saltation or rely on naive `C(V)dV/dt` / overflowing `i64 * i64`. |
| **BYO-Git, Padmé Vault & Universal Parts** | Customer GitHub/GitLab/Bitbucket (`ORG_NAMESPACE`/`USER_NAMESPACE`): `<project>-meta` + lazy `<project>-<type>` (`<= 40/min`, `provider_repo_id`), blobless `--filter=blob:none` + `64 KB` LFS, Padmé-padded path-blinded (`blobs/*.sigv`) `.signal/vault.merkle.sig` RFC 8452 `AES-256-GCM-SIV` (`MUST_ENCRYPT`), Dual-Fenced 2PC `Super-Commit` (proxy epoch + `<project>-meta` CAS first), BYOK Continuity WAL, In-Jurisdiction DR, `vcs-auditor`, 3-Tier Universal Parts (`parts.lock.json` + `PadstackDef` + Org AVL). | Never hold permanent customer IP on Signal storage, burst-create repos, leak filenames/sizes or allow rollback swaps in `ENCRYPTED_VAULT`, push without `X-Signal-Fencing-Token`, replicate Sovereign DR across borders, or leak Org NDA parts into Global CAS. |
| **Provider Auth, Zero-Leak DLP & 2FA SFTP** | Dual-bind SSO + PKCE (`provider_user_id`), gate on live `provider_repo_id` + unexpired `expires_at`, Org/Project Admin Access Review (2-of-3 Break-Glass). Zero browser disk IP storage (`localStorage`/`IndexedDB`/SW); zero `/export`/`/download`/`/print` routes; Dual-Plane `Zstd-QOI`/WebP + HW `HEVC`/Dual-Luma + `HudOverlayPrimitive` + Tardos/Constellation watermarks. Fresh Step-Up 2FA (WebAuthn or Redis-burned TOTP) + write-only `russh` SFTP + Fail-Closed 3x Quorum Tier-0 Audit Outbox → pseudonymous RFC-3161 WORM logs. | Never bypass provider rosters, allow TOTP replay or SFTP read-back, drop Tier-0 audits asynchronously, store raw PII in WORM chains, or use software `AV1 4:4:4` decode. |
| **Contracts, AI, Versioning & Monorepo Dev** | Build-time codegen (`signal-proto`, `pnpm codegen`), SI suffixes (`_nm`, `_ps`, `_uv`), `bigint` on 64-bit fields, `fixtures/` + `transcripts/`. Neuro-symbolic AI delegates math to Rust, isolates untrusted text/images via Zero-Tool Quarantine + `GBNF`, re-verifies proofs on merge. Monorepo `justfile` + `VERSION` + auto Release Notes (`CHANGELOG.md`, `releases/notes/`, `physics_determinism_epoch`, LTS pinning, SLSA L3/Cosign/SBOM, Expand-and-Contract SQL); local dev (`dev:lite`, `dev:k8s`, Ollama/`deterministic_mock`, `local-guest-0001`) enforces full RBAC/crypto behind `dev-guest-auth`. | Never commit generated stubs, cast 64-bit fields to JS `number`, feed untrusted text/images to tool-calling LLMs, silently shift pinned LTS physics hashes, drop columns mid-rollout, bypass RBAC in local dev, or ship `dev-guest-auth` in releases. |

---

## 5. Clean-Code, Token-Efficiency & Anti-Hallucination Guardrails

- **2–3 Sentence Dev Replies:** During coding/fixes, reply in **2–3 sentences max** (`Issue -> Fix -> Verification`); omit recaps unless asked.
- **Anti-Hallucination:** Verify paths, symbols, `.proto` fields, & deps via `rg`/`view_file`/`Cargo.toml`/`package.json`; claim pass ONLY on observed exit `0`; never recall pinouts/SI constants from memory; stop/ask or log `BLOCKED_ON_CONTRACT` on ambiguity.
- **Modularity:** Files `80–250 LOC` (split `> 350`), functions `<= 40–60 LOC`, nesting `<= 3`, params `<= 4`, 3-line `Module / Invariants / Spec` header.
- **Rust (`engine/`):** Rust 2024, `#![forbid(unsafe_code)]`, acyclic DAG, `signal-proto` + `cargo-hakari`, `thiserror`, zero `.unwrap()` on untrusted input, `SecretBox<T>`, `core_math`, `cargo vet`/`deny`.
- **Angular (`web/`):** Angular 19/20+ Zoneless, Standalone ONLY, `OnPush`, Signals (`input()`/`output()`/`computed()`), `@if`/`@for`/`@defer`, `@signal/<lib>` `index.ts`, `bigint` SI types, `cdk-virtual-scroll`, WCAG 2.2 AA.
- **Node.js (`services/`):** Node 22 strict TS, stateless, fail-closed `@Require*` uniform-`404`, `withRlsTx` (`NOBYPASSRLS`), `Idempotency-Key`, `Redacted<T>`, zero physics.

---

## 6. Deadlock-Free Antigravity + Cursor Parallel Swimlanes

| Swimlane | Default Owner | Primary Paths | Branch Prefix |
|---|---|---|---|
| **Lane A — Rust, K8s & RAG** | **Antigravity** | `engine/**`, `benchmarks/**`, `infra/k8s/**`, `ai/datasheet-ingest/**`, `ai/rag-service/**` | `ag/<task-id>-<slug>` |
| **Lane B — Angular, Node & AI UI** | **Cursor** | `web/**`, `services/**`, `ai/orchestrator/**` | `cursor/<task-id>-<slug>` |
| **Shared Governed Zone (Serialized)** | Either (sequential to `main`) | `contracts/**`, `architecture/**`, `docs/engineering/**`, `.agents/skills/**`, `.cursor/rules/**`, `AGENTS.md`, `infra/local/**`, `VERSION`, `justfile`, `releases/**` | `contract/<task-id>-<slug>` |

- Single-writer journals (`coordination/agents/antigravity.md` vs `cursor.md`); immutable handoffs (`coordination/handoffs/YYYYMMDD-<task-id>-<from>-to-<to>.md`); UTC migrations (`YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql`); `> 24h` stale claims reclaimable via handoff.

---

## 7. Definition of Done (Skill: `signal-verification-and-pr-gate`)

- [ ] Rebased on `origin/main` with ONLY declared `touched_paths` + own journal modified; zero architectural/security/DLP/`bigint`/LOC violations.
- [ ] All checks green (Docs: links/Mermaid/GFM/UTF-8/`git fsck`; Contracts: `buf lint/breaking` + fixtures/transcripts; Rust: `fmt`/`clippy -D warnings`/`test`/`deny`/`vet`; Web/Node: `lint`/`typecheck`/`test`/`pnpm audit --ignore-scripts`).
- [ ] Clean tree (zero generated stubs/secrets), Conventional Commit, and own journal updated.
