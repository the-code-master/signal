# Signal — Canonical AI Agent Instructions (`AGENTS.md`)

> **Applies To:** **Antigravity** & **Cursor** ([`.cursor/rules/*.mdc`](./.cursor/rules/signal-architecture.mdc) + [`.agents/skills/*/SKILL.md`](./.agents/skills/README.md)).  
> **Single Source of Truth:** [`architecture/`](./architecture/README.md) & [`docs/engineering/`](./docs/engineering/README.md).

---

## 1. Phase & Hard Gate

- **Phase:** **Architecture, Rules & Skills Complete** (Ready for Phase 0). **Gate:** **DO NOT write production code (`engine/`, `services/`, `web/`, `ai/`, `contracts/`, `infra/`) until explicit Phase 0 approval.**
- **Stack (Nov 2026):** **Angular `v22+` ONLY** (TS `6.0+`, **Node `26 LTS+`** Tier 1 edge, `pnpm 11+`), **Rust `1.100+`** (2024 Ed.), **Python `3.15+` FastAPI** (`Granian 2.8+`, `Pydantic v2`, `uv`), **Postgres `19+`**, **K8s `1.37+`**.
- **Runbook:** [`PHASE0_LAUNCH_PLAYBOOK.md`](./docs/engineering/PHASE0_LAUNCH_PLAYBOOK.md) | **Backlog:** [`TASK_BOARD.md`](./coordination/TASK_BOARD.md).

---

## 2. Session-Start (`signal-session-start`)

Before editing, **both IDEs** MUST:
1. Run `git fetch origin --prune && git status -sb`.
2. Read [`TASK_BOARD.md`](./coordination/TASK_BOARD.md), [`handoffs/`](./coordination/handoffs/README.md), & peer journal read-only (Antigravity → [`cursor.md`](./coordination/agents/cursor.md); Cursor → [`antigravity.md`](./coordination/agents/antigravity.md)).
3. Verify zero `touched_paths` overlap; branch from `origin/main` (`ag/*`, `cursor/*`, `contract/*`) and load matching specs/skills.

---

## 3. Architecture, Engineering & Skill Lookup Matrix

| Domain | Spec / Guide | Skill(s) |
|---|---|---|
| Vision, 120 Features, CAD/CAM/PLM, Collab | [`01`](./architecture/01-product-vision-and-features.md), [`17`](./architecture/17-extended-feature-specifications-ui-and-rust.md), [`19`](./architecture/19-cad-cam-3d-printing-and-plm-operations.md), [`21`](./architecture/21-collaboration-space-project-status-and-automated-reviews.md) | `signal-token-efficient-editing`, `signal-cad-cam-plm-ops`, `signal-collab-and-status-reviews` |
| Topology, 3-Tier K8s (`1–4` Nodes), Releases | [`02`](./architecture/02-system-architecture.md), [`11`](./architecture/11-implementation-roadmap.md), [`14`](./architecture/14-universal-scale-accessibility-and-expandability.md), [`20`](./architecture/20-versioning-release-monorepo-build-and-k8s-topology.md) | `signal-session-start`, `signal-release-deploy-topology` |
| Rust `1.100+` Core (`engine/**`) | [`03`](./architecture/03-rust-eda-engine.md), [`RUST_ENGINEERING_GUIDE.md`](./docs/engineering/RUST_ENGINEERING_GUIDE.md) | `signal-rust-crate-architect`, `signal-rust-physics-and-geometry` |
| Angular `v22+` UI (`web/**`), Styles, Canvas | [`06`](./architecture/06-frontend-angular-and-fastapi-control-plane.md), [`ANGULAR_ENGINEERING_GUIDE.md`](./docs/engineering/ANGULAR_ENGINEERING_GUIDE.md), [`ANGULAR_UI_STYLE_GUIDE.md`](./docs/engineering/ANGULAR_UI_STYLE_GUIDE.md) | `signal-angular-workbench`, `signal-angular-ui-style-system`, `signal-zero-leak-canvas-and-dlp` |
| Python `3.15+` FastAPI (`services/**`) | [`06`](./architecture/06-frontend-angular-and-fastapi-control-plane.md), [`08`](./architecture/08-identity-enterprise-and-project-model.md), [`21`](./architecture/21-collaboration-space-project-status-and-automated-reviews.md), [`FASTAPI_ENGINEERING_GUIDE.md`](./docs/engineering/FASTAPI_ENGINEERING_GUIDE.md) | `signal-fastapi-control-plane`, `signal-collab-and-status-reviews` |
| BYO-Git & Padmé Vault | [`13`](./architecture/13-vcs-git-server-and-storage-analysis.md), [`07`](./architecture/07-security-encryption-dlp-and-sftp.md) | `signal-byo-git-and-vault` |
| Isolation, ASVS L3, Pods, PQC, 148 Flaws | [`04`](./architecture/04-kubernetes-runtime-and-collaboration.md), [`07`](./architecture/07-security-encryption-dlp-and-sftp.md), [`15`](./architecture/15-application-security-and-project-isolation.md), [`16`](./architecture/16-architectural-audit-flaws-and-futuristic-innovations.md) | `signal-security-isolation-audit` |
| AI, Quarantine, 3-Tier RAG & Parts (`ai/**`) | [`05`](./architecture/05-ai-agent-and-datasheet-rag.md), [`18`](./architecture/18-universal-component-and-datasheet-repository.md) | `signal-ai-neuro-symbolic-rag`, `signal-universal-parts-registry` |
| Contracts (`contracts/**`) | [`10`](./architecture/10-api-and-data-contracts.md) | `signal-contract-first` |
| Mac Dev (`infra/local/**`) & Dual-IDE | [`09`](./architecture/09-local-development-environment.md), [`12`](./architecture/12-multi-ide-collaboration.md), [`TOKEN_EFFICIENCY_AND_CLEAN_CODE.md`](./docs/engineering/TOKEN_EFFICIENCY_AND_CLEAN_CODE.md) | `signal-local-mac-dev`, `signal-cross-ide-handoff`, `signal-verification-and-pr-gate` |

---

## 4. Non-Negotiable Invariants (PR Rejection If Violated)

| Invariant | ALWAYS DO | NEVER DO |
|---|---|---|
| **1 Project = 1 Cell, 3-Tier K8s & AppSec** | Separate `pool-web` (Angular `v22+` + Node `26 LTS+`), HA `pool-api` (FastAPI/Granian + Envoy `signal-engine-bridge`), & `1–4 -> N` `pool-rust-engine` nodes (`signal-cell-wasm` or Kata/`CONFIDENTIAL_SILICON` + Exclusive `H100 CC`/`B200 TEE-I/O` MIG); Postgres `19+` RLS (`NOBYPASSRLS`) + 3-Tier `pgvector`, Hybrid PQC DEK/SVID, Cilium 100% lateral drop, scoped NATS JWTs, fail-closed `Depends(require_*)` uniform-`404`, epoch-fenced Rust `signal-egress-proxy`, Route `COOP`/`COEP`, child MicroVM IPC for `IBIS-AMI`/`OSDI` (LLVM-free `signal-pod`), bomb-capped parsers (`<= 20:1`, depth `<= 64`). | Never share pod/DEK/SVID/NATS/time-sliced GPU, let `signal-web` dial Rust/DB, proxy 120 FPS pixels via FastAPI JSON, claim `MIG + CC` on H100, `dlopen`/link LLVM in `signal-pod`, leak `403` vs `404`, use `eval`/`exec`/`pickle`/`bypassSecurityTrust*`, or log secrets outside `SecretBox`/`SecretStr`. |
| **Pure-Rust `1.100+` Physics, CAD/CAM & PLM** | All EDA/MCAD/CAM/3D-Print/PLM kernels (`ElectronicModule`, `q(v)`/`Φ(i)` Ruiz MNA `KLU`↔`cuDSS`, Saltation + `Revolve` Adjoint + PSS/HB, Formal, CPO/3D EM, Aging, Cryo, Pelgrom/LVF, ECO, `signal-{magnetics,power-integrity,emc,dfm,harness,fpga,security-hw,parts,mcad,cam,plm}`, `i128`/Shewchuk, `0.5-ULP` `core_math`, Gerber/GDSII/STEP/G-code/3MF) in Rust `1.100+` (`engine/`). | Never put SPICE, magnetics, creepage, EMC/FMEDA, CAD/CAM/slicing, DRC/LVS, routing, or manufacturing writers in FastAPI/Angular; never differentiate switches without Saltation or rely on naive `C(V)dV/dt` / overflowing `i64 * i64`. |
| **BYO-Git, Padmé Vault & Parts** | Customer GitHub/GitLab/Bitbucket (`ORG`/`USER_NAMESPACE`): `<project>-meta` + lazy `<project>-<type>` (`<= 40/min`, `provider_repo_id`), blobless `--filter=blob:none` + `64 KB` LFS, Padmé-padded path-blinded (`blobs/*.sigv`) `.signal/vault.merkle.sig` `AES-256-GCM-SIV` (`MUST_ENCRYPT`), Dual-Fenced 2PC `Super-Commit`, BYOK Continuity WAL, In-Jurisdiction DR, `vcs-auditor`, 3-Tier Universal Parts (`parts.lock.json` + `PadstackDef` + Org AVL). | Never hold permanent customer IP on Signal storage, burst-create repos, leak filenames/sizes or allow rollback swaps in `ENCRYPTED_VAULT`, push without `X-Signal-Fencing-Token`, replicate Sovereign DR across borders, or leak Org NDA parts into Global CAS. |
| **Provider Auth, Zero-Leak DLP & SFTP** | Dual-bind SSO + PKCE (`provider_user_id`), gate on live `provider_repo_id` + unexpired `expires_at`, Access Review (2-of-3 Break-Glass). Zero browser disk IP (`localStorage`/`IndexedDB`/SW); zero `/export`/`/download`/`/print` routes; Dual-Plane `Zstd-QOI`/WebP + HW `HEVC`/Dual-Luma + `HudOverlayPrimitive` + Tardos/Constellation watermarks. Fresh Step-Up 2FA (WebAuthn or Redis-burned TOTP) + write-only Rust `russh` SFTP + Fail-Closed 3x Quorum Tier-0 Audit Outbox → pseudonymous RFC-3161 WORM logs. | Never bypass provider rosters, allow TOTP replay or SFTP read-back, drop Tier-0 audits asynchronously, store raw PII in WORM chains, or use software `AV1 4:4:4` decode. |
| **Contracts, AI, Releases & Local Dev** | Build-time codegen (`signal-proto`, `uv`, `pnpm codegen`), SI suffixes (`_nm`, `_ps`, `_uv`), `bigint` / Python `int` JSON-string on 64-bit fields, `fixtures/` + `transcripts/`. Neuro-symbolic AI delegates math to Rust, isolates untrusted text/images via Zero-Tool Quarantine + `GBNF`, re-verifies proofs on merge. Unified `./signal.sh` → stdlib Python `scripts/signal_ops.py` + `components.toml` (Local == thin CI YAML) + `VERSION` + auto Release Notes (`CHANGELOG.md`, `releases/notes/`, `physics_determinism_epoch`, LTS pinning, SLSA L3/Cosign/SBOM, Expand-and-Contract SQL); local dev (`./signal.sh dev`, Ollama/`deterministic_mock`, `local-guest-0001`) enforces full RBAC/crypto behind `dev-guest-auth`. | Never commit generated stubs, cast 64-bit fields to JS `number`, feed untrusted text/images to tool-calling LLMs, bury build/deploy shell scripts in CI YAML, silently shift pinned LTS physics hashes, drop columns mid-rollout, bypass RBAC in local dev, or ship `dev-guest-auth` in releases. |

---

## 5. Clean-Code, Token-Efficiency & Anti-Hallucination

- **2–3 Sentence Dev Replies:** During coding/fixes, reply in **2–3 sentences max** (`Issue -> Fix -> Verification`); omit recaps unless asked.
- **Anti-Hallucination:** Verify paths, symbols, `.proto` fields, & deps via `rg`/`view_file`/`Cargo.toml`/`package.json`/`pyproject.toml`; claim pass ONLY on exit `0`; never recall pinouts/SI constants from memory; stop/ask or log `BLOCKED_ON_CONTRACT` on ambiguity.
- **Modularity:** Files `80–250 LOC` (split `> 350`), functions `<= 40–60 LOC`, nesting `<= 3`, params `<= 4`, 3-line `Module / Invariants / Spec` header.
- **Rust (`engine/`):** **Rust `1.100+`** (2024 Ed.), `#![forbid(unsafe_code)]`, acyclic DAG, `signal-proto` + `cargo-hakari`, `thiserror`, zero `.unwrap()` on untrusted input, `SecretBox<T>`, `core_math`, Cargo cooldowns, `cargo vet`/`deny`.
- **Angular (`web/`):** **Angular `v22+` ONLY** (TS `6.0+`, **Node `26 LTS+`**, `pnpm 11+`), Zoneless, Standalone, `OnPush`, Signal Forms & Signals (`input()`/`output()`/`computed()`), split `.component.ts`/`.html` (`<= 150 LOC`, nesting `<= 2`, zero `@if`-soup/duplication)/`.logic.ts`/`.model.ts`, centralized `@signal/design-system` + `@signal/ui-kit` (NO per-component CSS unless `@sig-style-exception`), `bigint` SI types, WCAG 2.2 AA.
- **FastAPI (`services/`):** **Python `3.15+` + FastAPI (`0.165+`) + Granian `2.8+` + Pydantic v2 (`2.14+`)**, `uv`, stateless, fail-closed `Depends(require_*)` + startup auth gate + uniform-`404`, Postgres `19+` `with_rls_tx` (`NOBYPASSRLS`), `Idempotency-Key`, `SecretStr`, `PyO3` bridge, zero physics.

---

## 6. Deadlock-Free Parallel Swimlanes

| Swimlane | Owner | Primary Paths | Branch Prefix |
|---|---|---|---|
| **Lane A — Rust, K8s & RAG** | **Antigravity** | `engine/**`, `benchmarks/**`, `infra/k8s/**`, `ai/datasheet-ingest/**`, `ai/rag-service/**` | `ag/<task-id>-<slug>` |
| **Lane B — Angular, FastAPI & AI UI** | **Cursor** | `web/**`, `services/**`, `ai/orchestrator/**` | `cursor/<task-id>-<slug>` |
| **Shared Governed Zone (Serialized)** | Either (sequential to `main`) | `contracts/**`, `architecture/**`, `docs/engineering/**`, `.agents/skills/**`, `.cursor/rules/**`, `AGENTS.md`, `infra/local/**`, `VERSION`, `signal.sh`, `scripts/**`, `justfile`, `releases/**` | `contract/<task-id>-<slug>` |

- Single-writer journals (`coordination/agents/antigravity.md` vs `cursor.md`); immutable handoffs (`coordination/handoffs/YYYYMMDD-<task-id>-<from>-to-<to>.md`); UTC migrations (`YYYYMMDDHHMMSS_<ag|cursor>_<slug>.sql`); `> 24h` stale claims reclaimable via handoff.

---

## 7. Definition of Done (`signal-verification-and-pr-gate`)

- [ ] Rebased on `origin/main`; ONLY declared `touched_paths` + own journal modified; zero security/DLP/`bigint`/LOC violations.
- [ ] Checks green (Docs: links/Mermaid/GFM/UTF-8/`git fsck`; Contracts: `buf lint/breaking` + fixtures; Rust `1.100+`: `fmt`/`clippy -D warnings`/`test`/`deny`/`vet`; Web `v22+`: `pnpm lint/typecheck/test/audit --ignore-scripts`; FastAPI/AI `3.15+`: `uv run ruff/pyright/pytest/bandit/pip-audit`).
- [ ] Clean tree (zero generated stubs/secrets), Conventional Commit, own journal updated.
