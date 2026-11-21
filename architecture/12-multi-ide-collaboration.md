# 12 — Multi-AI-IDE Parallel Collaboration Protocol (Antigravity + Cursor)

> **Scope:** Establishes a strict, **deadlock-free** engineering and repository workflow so **Antigravity** and **Cursor** can build **Signal** simultaneously at full speed—with zero file collisions, zero circular wait deadlocks, zero broken interface contracts, and clean Git history.

---

## 1. Why Naive Multi-AI-IDE Collaboration Fails (and How We Prevent It)

Running two autonomous AI coding agents concurrently on the same repository typically breaks down in four ways:
1. **Working-Tree Clobbering:** Both IDEs edit files in the same folder simultaneously, overwriting uncommitted changes and corrupting builds/tests.
2. **Logical Deadlocks (Circular Wait):** Agent A waits for Agent B to finish a schema while Agent B waits for Agent A's service endpoint—or both hold file-based locks waiting for each other.
3. **Shared-File Contention:** Both agents append to `Cargo.toml`, `package.json`, `index.ts`, DB migration sequence numbers, or a single shared status file, causing constant merge conflicts.
4. **Convention & Architecture Drift:** Each IDE invents slightly different naming, error formats, or security shortcuts because instructions aren't unified.

**Signal eliminates all four failure modes using five classic distributed-systems guarantees:**
1. **Physical Isolation via Git Worktrees** (zero shared working directory).
2. **Contract-First Dependency DAG** (eliminates circular wait—deadlocks become structurally impossible).
3. **Disjoint Ownership Swimlanes & File-Scope Claims** (eliminates write-write collisions).
4. **Lock-Free Append-Only Coordination Ledger** (no mutual waiting; each agent only writes to its own journal).
5. **Single Canonical Rulebook & Automated CI Merge Gates** (guarantees identical engineering quality from both IDEs).

---

## 2. Physical Isolation: Mandatory Git Worktrees

**HARD RULE:** Antigravity and Cursor MUST **NEVER** operate in the same local checkout directory at the same time.

Instead, use **Git Worktrees** backed by a single shared `.git` object store on the developer's machine:

```text
~/workspaces/
├── signal-main/          # Clean read-only reference tracking origin/main
├── signal-antigravity/   # Worktree dedicated EXCLUSIVELY to Antigravity (branches: ag/*)
└── signal-cursor/        # Worktree dedicated EXCLUSIVELY to Cursor      (branches: cursor/*)
```

### Branch Naming Convention
- **Antigravity branches:** `ag/<task-id>-<short-slug>` (e.g., `ag/P1-04-resistor-mna-module`)
- **Cursor branches:** `cursor/<task-id>-<short-slug>` (e.g., `cursor/P1-07-angular-schematic-canvas`)
- **Contract-change branches (sequenced first):** `contract/<task-id>-<short-slug>` (merged into `main` BEFORE parallel implementation branches consume it).

---

## 3. Eliminating Deadlocks: Contract-First Acyclic Workflow

In operating systems, deadlock requires four Coffman conditions—most critically **Hold-and-Wait** and **Circular Wait**. We break BOTH structurally:

```mermaid
flowchart TB
    subgraph Step1["Step 1: Contract Freeze (Serialized, Fast PR -> main)"]
        C1["Update contracts/proto/* or contracts/openapi/*\nRun Buf / OpenAPI Codegen & CI Check\nMerge to main"]
    end

    subgraph Step2["Step 2: Parallel Disjoint Execution (Zero Waiting!)"]
        direction LR
        subgraph LaneA["Swimlane A (e.g., Antigravity Worktree: ag/*)"]
            A1["Rust Engine Crates (engine/crates/*)"]
            A2["K8s Operator & Infra (infra/k8s/*)"]
            A3["AI/RAG Core & Security Crypto"]
        end
        subgraph LaneB["Swimlane B (e.g., Cursor Worktree: cursor/*)"]
            B1["Angular v22+ Web IDE (web/apps/*, web/libs/*)"]
            B2["Python 3.15+ FastAPI Control Plane (services/*)"]
            B3["UI Design System & Mock/Contract Tests"]
        end
    end

    subgraph Step3["Step 3: Independent Rebase & Automated Merge Gate"]
        M1["Each Agent rebases on origin/main\nRuns full unit + contract conformance tests\nMerges independently via fast-forward / squash PR"]
    end

    Step1 --> LaneA
    Step1 --> LaneB
    LaneA --> Step3
    LaneB --> Step3
```

### Anti-Deadlock Rules
1. **No Cross-Agent Blocking Waits:** Neither AI IDE is ever permitted to "sleep/wait for the other agent." If an upstream implementation isn't merged yet, the agent builds against the **frozen `contracts/` mock/stub** generated from Protobuf/OpenAPI and merges cleanly!
2. **Strict Topological Ordering for Contracts:** If a task requires a new gRPC RPC or REST endpoint, a tiny **Contract-Only PR** (`contracts/` only, < 10 minutes to review & merge) lands on `main` FIRST. Once merged, both worktrees run `git fetch && git rebase origin/main` and implement both sides of the contract simultaneously in complete independence.
3. **One Task = Disjoint `touched_paths` Globs:** Every claimed task declares explicit directory/file globs (`touched_paths`). Two active tasks in parallel MUST have **empty path intersection** (`paths(A) ∩ paths(B) = ∅`).

---

## 4. Recommended Parallel Swimlane Ownership

Because Signal's monorepo is cleanly layered, default swimlanes minimize overlap dramatically (lanes can be swapped per sprint as long as active tasks remain path-disjoint):

| Swimlane | Primary Directories | Default Primary Owner | Shared Interface Boundary |
|---|---|---|---|
| **Lane A — Core Engine, Physics, CAD/CAM/PLM Kernels & K8s Infra** | `engine/` (Rust `1.100+` incl. `signal-parts`, `signal-mcad`, `signal-cam`, `signal-plm`), `benchmarks/`, `infra/k8s/` (3-Tier K8s `1.37+` Helm/Karpenter/Cilium/GitOps), `infra/docker/Dockerfile.{rust-pod,operator,bridge}`, `ai/datasheet-ingest/`, `ai/rag-service/` | **Antigravity** (configurable per sprint) | Implements `contracts/proto/signal/v1/*` servers |
| **Lane B — Angular v22+ UI, FastAPI Control Plane, PLM/Parts/Collab** | `web/` (Angular `v22+` ONLY incl. `libs/{design-system,ui-kit,library-studio,mcad-studio,cam-print,plm-ops,collab-hub}` + What's New UI), `services/` (Python `3.15+` FastAPI incl. `parts-registry`, `plm-ops`, `collab-hub`, Release Notes API), `infra/docker/Dockerfile.{web,api}`, `ai/orchestrator/` (UI bridge) | **Cursor** (configurable per sprint) | Consumes `contracts/proto/*`, implements `contracts/openapi/*` |
| **Shared Governed Zone (Serialized Only)** | `contracts/`, `architecture/`, `docs/engineering/`, `.agents/skills/`, `.cursor/rules/`, `AGENTS.md`, `infra/local/`, `VERSION`, `CHANGELOG.md`, `justfile`, `releases/`, `.github/workflows/` | **Single-Writer PR Rule** (either agent may propose, merged sequentially to `main` before parallel consumers rebase) | Checked by CI breaking-change detector (`buf breaking`) |

---

## 5. Lock-Free Coordination Ledger (`coordination/`)

The repository includes a lightweight, human-readable [`coordination/`](../coordination/TASK_BOARD.md) directory engineered so **Antigravity and Cursor NEVER suffer merge conflicts on status tracking**:

```text
coordination/
├── TASK_BOARD.md            # Orchestrated backlog (Human / Task Planner assigns Lane A vs Lane B)
├── agents/
│   ├── antigravity.md       # WRITTEN ONLY BY ANTIGRAVITY (read-only for Cursor)
│   └── cursor.md            # WRITTEN ONLY BY CURSOR      (read-only for Antigravity)
└── handoffs/
    └── YYYYMMDD-<id>.md     # Immutable one-file-per-handoff notes (append-only, never edited by both!)
```

### Why Separate Per-Agent Status Files Matter
- If both AI IDEs updated a single `STATUS.md` on every PR, **every single merge would conflict on `STATUS.md`**.
- With `coordination/agents/antigravity.md` (single-writer: Antigravity) and `coordination/agents/cursor.md` (single-writer: Cursor), plus immutable timestamped files under `coordination/handoffs/`, **zero coordination merge conflicts ever occur**, while each AI can read the other AI's latest status after `git fetch origin main`!

### Standard Agent Journal Entry Format
Each agent logs concise, machine-scannable entries in its own file:
```markdown
### [P1-04] Implement Resistor MNA Stamp & Tolerance Sampler
- **Status:** `IN_PROGRESS` | `MERGED` | `BLOCKED_ON_CONTRACT`
- **Branch:** `ag/P1-04-resistor-mna`
- **Touched Paths:** `engine/crates/signal-lib-passive/**`
- **Contract Dependency:** `contracts/proto/signal/v1/component.proto@v1.0` (merged in commit `abc1234`)
- **Verification:** `cargo test -p signal-lib-passive` passing
- **Handoff Notes for Peer IDE:** None — matches frozen v1.0 contract.
```

---

## 6. High-Contention Shared File Rules (Conflict Prevention)

Even with disjoint swimlanes, certain files traditionally attract conflicts in monorepos. Signal enforces deterministic rules to neutralize them:

| Contention / Drift Hotspot | Deterministic Anti-Conflict & Anti-Drift Rule |
|---|---|
| **Database Migrations** | NEVER use sequential integers (`001_`, `002_`). ALWAYS use UTC microsecond timestamp + agent tag: `YYYYMMDDHHMMSS_<ag\|cursor>_<slug>.sql` (e.g., `20261201143022_cursor_add_sftp_targets.sql`). |
| **Generated Contract Stubs** | **NEVER commit generated Protobuf/OpenAPI code to Git!** Generate deterministically at build time (`engine/crates/signal-proto` once for the Rust workspace; `pnpm codegen` prebuild in TS) with pinned `buf` / OpenAPI versions. |
| **Cross-Lane Unit & `int64` `bigint` Safety** | Every physical field in `contracts/` MUST carry an explicit SI unit suffix (`_nm`, `_ps`, `_uv`, `_na`, `_mkelvin`, `_hz`), **100% of 64-bit integer fields MUST map to ES2020 `bigint` (`jstype = JS_STRING` / `longAsBigint: true` — preventing JS `2^53` silent rounding!)**, and both IDEs verify against `contracts/fixtures/` + **stateful multi-step `contracts/transcripts/*.jsonl`**! |
| **Stale / Interrupted Task Claims** | An `IN_PROGRESS` task with **zero new commits on its branch for > 24 hours** has an expired claim lease—the peer IDE or human lead may cleanly take over by logging an immutable handoff file in `coordination/handoffs/`. |
| **`Cargo.toml` & `package.json` + Supply Chain** | Scope dependencies to individual crates/packages (using `cargo-hakari` `workspace-hack` to keep Mac incremental builds `< 2 s`); **any newly added external crate/npm dependency MUST pass `cargo vet` / `cargo deny` / `pnpm audit --ignore-scripts` + explicit human review**! |
| **Protobuf / OpenAPI Schemas (`contracts/`)** | Additive-only evolution inside `v1`; reserve deleted field tags; enforce `buf breaking` and `openapi-diff` in CI so neither agent can silently break the peer's build. |
| **Barrel Files & `.env.example`** | Enforce alphabetical formatter sorting; prefer direct module imports; split env templates under `infra/local/env/<subsystem>.env.example`. |

---

## 7. Unified AI Rulebook (Keeping Antigravity & Cursor 100% Aligned)

To ensure both AI IDEs enforce the exact same Signal architecture, security invariants (Complete Project Isolation, 3-Tier K8s Pod/Node Separation, Hybrid PQC, Padmé-blinded Vault, Dual-Gate External Fencing, Fail-Closed Tier-0 Audit, zero local files, no export/print, mandatory 2FA for SFTP/BYO-Git, RAM-only decryption, local `Guest @ Signal Design, Inc.` compile-guards), 2–3 sentence coding replies, anti-hallucination discipline, and coding standards:

1. **Single Source of Truth:** Canonical [`AGENTS.md`](../AGENTS.md) (`< 12 KB`) at the repository root points directly to `architecture/01..21`, `docs/engineering/*`, and `.agents/skills/*/SKILL.md` (`19` portable skills) and codifies all invariants, session-start rituals, engineering standards, and definition-of-done gates.
2. **IDE Adapter Files & Portable Skills (Progressive Disclosure):**
   - Cursor automatically loads [`.cursor/rules/signal-architecture.mdc`](../.cursor/rules/signal-architecture.mdc) (`alwaysApply: true`, `< 4 KB`) + 5 path-scoped `.cursor/rules/*.mdc` files → explicitly referencing `AGENTS.md`, `architecture/01..21`, and `.agents/skills/*/SKILL.md`.
   - Both **Antigravity and Cursor** natively discover all **19 portable Open-Standard skills** under [`.agents/skills/`](../.agents/skills/README.md).
3. **Automated CI Guardrails (Enforced Equally on Both Agents via `just check` / `.github/workflows/pr-gate.yml`):**
   - **Rust `1.100+`:** `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo vet`, `cargo deny check`, plus headless software-Vulkan (`lavapipe`) WGSL shader parity checks
   - **Angular `v22+` (`web/`):** `pnpm --filter @signal/web lint`, `typecheck`, `test`, `pnpm audit --ignore-scripts`, `helm lint` + `kubeconform`
   - **Python `3.15+` FastAPI & AI (`services/`, `ai/`):** `uv run ruff check`, `uv run ruff format --check`, `uv run pyright`, `uv run pytest`, `uv run bandit -ll`, `uv run pip-audit --strict`
   - **Contracts:** `buf lint`, `buf breaking`, `contracts/fixtures/` (Rust↔Python↔TS), and **`contracts/transcripts/*.jsonl` stateful protocol replay**
   - **Security & Isolation Policy CI Check:** Fails any PR that omits a fail-closed `Depends(require_*)` FastAPI auth guard, leaks `403` vs `404` resource existence, bypasses PostgreSQL `19+` RLS, uses JS `number` on `int64` contract fields, adds HTTP file-download/export headers, writes design entities to `localStorage`/`IndexedDB`, allows `signal-web` to dial Rust pods directly, enables `dev-guest-auth` in release profiles, or creates Tier-0 SFTP/BYO-Git/PLM-CCB endpoints without Step-Up 2FA + synchronous audit pre-commit!

---

## 8. Daily Parallel Operating Checklist (Human Tech Lead + Antigravity + Cursor)

1. **Start of Sprint / Session:**
   - Review `coordination/TASK_BOARD.md`. Pick non-overlapping tasks for **Lane A (Antigravity)** and **Lane B (Cursor)**.
   - If any task needs new API/Proto fields, merge the small `contract/*` PR to `main` FIRST.
2. **Parallel Execution:**
   - Sync both worktrees (`git fetch origin && git rebase origin/main`).
   - Launch Antigravity in `signal-antigravity/` on `ag/<task>` and Cursor in `signal-cursor/` on `cursor/<task>`.
   - Each agent updates ONLY its own `coordination/agents/<agent>.md` file.
3. **Short-Lived PR Merges (< 1 Day Lifecycle):**
   - First agent to finish rebases on `origin/main`, passes CI, and merges.
   - Second agent rebases on updated `origin/main` (near-zero conflict because `touched_paths` are disjoint!), passes CI, and merges.
   - **Result:** Continuous parallel velocity, zero deadlocks, zero clobbered files, and 100% architectural integrity across both AI IDEs.
