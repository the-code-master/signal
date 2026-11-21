---
name: signal-verification-and-pr-gate
description: Executes Signal's complete pre-commit and PR Definition-of-Done verification suite across Rust 1.100+, Angular v22+, Python 3.15+ FastAPI, Contracts, and Architecture docs, updates the active agent's single-writer journal, and prepares clean Conventional Commits and PRs. Use right before committing, pushing, or opening a pull request in Antigravity or Cursor.
---

# Signal Verification & PR Gate (Definition of Done)

Governing rules: `AGENTS.md` Section 7 | `architecture/12-multi-ide-collaboration.md`.

## 1. Scope & Rebase Hygiene
1. Rebase cleanly onto latest `origin/main`:
   ```bash
   git fetch origin --prune && git rebase origin/main
   ```
2. Inspect `git status -sb` and `git diff --stat`:
   - Confirm ONLY declared `touched_paths` (plus your own `coordination/agents/<agent>.md`) changed.
   - Confirm ZERO edits to the peer agent's journal, ZERO committed generated stubs, ZERO `.DS_Store` or stray temp artifacts, and ZERO secret values.

## 2. Domain Verification Gates (Run `./signal.sh check --changed` or Applicable Checks)
- **Unified CI/Local Gate (`signal.sh` + `scripts/**`):**
  - Prefer `./signal.sh check --changed` (runs identical checks locally and in CI). When editing `signal.sh` or `scripts/**`, verify `bash -n signal.sh`, `uv run ruff check scripts/`, `uv run pyright scripts/`, `uv run bandit -r scripts/ -ll`, `uv run pytest scripts/tests/ -q`, zero third-party bootstrap imports in `scripts/`, and thin `.github/workflows/*.yml` delegation only.
- **Planning / Docs / Rules Changes:**
  - Verify UTF-8 + LF endings, valid relative links & anchors, balanced code fences, exact GFM table column counts (escape any literal `|` inside table cells), valid Mermaid syntax (no HTML tags, balanced `subgraph`/`end`, quoted parens), escaped prose `\$`, and `git fsck --full`.
- **Contracts (`contracts/**`):**
  - `buf lint`, `buf breaking --against '.git#branch=main'`, OpenAPI 3.1 / JSON-Schema validation (zero drift against FastAPI), Rust↔Python↔TS `bigint` fixture round-trip & `contracts/transcripts/*.jsonl` replay.
- **Rust `1.100+` (`engine/**`, `benchmarks/**`):**
  - `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
  - `cargo deny check && cargo vet`
  - Verify `rust-version >= 1.100`, `#![forbid(unsafe_code)]`, zero `.unwrap()` on untrusted inputs, `dev-guest-auth` release compile-error guard, and golden `benchmarks/` parity.
- **Angular `v22+` ONLY (`web/**` — Node.js `26 LTS+`, `pnpm 11+`, TS `6.0+`):**
  - `pnpm --filter @signal/web lint && pnpm --filter @signal/web typecheck && pnpm --filter @signal/web test --run && pnpm audit --ignore-scripts`
  - Verify Angular `>= 22.0.0`, separate `.component.ts`/`.component.html` (`<= 150 LOC`, nesting `<= 2`, zero `@if`-soup/duplication)/`.logic.ts`/`.model.ts`, zero inline `template:`/`styles:`, zero component-local `.css`/`.scss` outside `design-system`/`ui-kit` without `/* @sig-style-exception */`, zero `Number()` casts on `bigint`, zero `NgModule`/`Zone.js`/legacy decorators, zero `bypassSecurityTrust*`/`eval`, and zero browser disk design storage.
- **Python `3.15+` FastAPI Control Plane & AI (`services/**`, `ai/**` — Granian `2.8+`, Pydantic v2, `uv`):**
  - `uv run ruff check services/ ai/ && uv run ruff format --check services/ ai/`
  - `uv run pyright services/ ai/ && uv run pytest services/ ai/ -q`
  - `uv run bandit -r services/ ai/ -ll && uv run pip-audit --strict`
  - Verify startup route-auth introspection gate (`Depends(require_*)` + uniform `404`), PostgreSQL `19+` `with_rls_tx` (`NOBYPASSRLS`), Fail-Closed 3x Quorum Audit Outbox, zero `eval`/`exec`/`pickle`/`shell=True`, and zero `/export`/`/download`/`/print` routes.

## 3. Anti-Hallucination Verification & 2–3 Sentence Reporting
- **Observed Ground Truth Only:** Never report a check as passing unless you executed it after your final edit and observed exit code `0`; report exact failing command output or `"Unverified — <reason>"` if skipped.
- **2–3 Sentence Dev Summary:** After completing a coding fix/PR gate, respond to the user in **2–3 sentences max** (`Issue -> Fix -> Verification command & result`), updating ONLY your own journal (`coordination/agents/antigravity.md` OR `coordination/agents/cursor.md`) and committing with a clean Conventional Commit (`docs:`, `contract:`, `feat:`, `fix:`, `refactor:`, `chore:`).
