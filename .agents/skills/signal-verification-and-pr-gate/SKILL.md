---
name: signal-verification-and-pr-gate
description: Executes Signal's complete pre-commit and PR Definition-of-Done verification suite across Rust, Angular, Node.js, Contracts, and Architecture docs, updates the active agent's single-writer journal, and prepares clean Conventional Commits and PRs. Use right before committing, pushing, or opening a pull request in Antigravity or Cursor.
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

## 2. Domain Verification Gates (Run All Applicable Checks)
- **Planning / Docs / Rules Changes:**
  - Verify UTF-8 + LF endings, valid relative links & anchors, balanced code fences, exact GFM table column counts (escape any literal `|` inside table cells), valid Mermaid syntax (no HTML tags, balanced `subgraph`/`end`, quoted parens), escaped prose `\$`, and `git fsck --full`.
- **Contracts (`contracts/**`):**
  - `buf lint`, `buf breaking --against '.git#branch=main'`, JSON Schema/OpenAPI validation, Rust↔TS `bigint` fixture round-trip & `contracts/transcripts/*.jsonl` replay.
- **Rust (`engine/**`, `benchmarks/**`):**
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check && cargo vet`
  - Verify `#![forbid(unsafe_code)]`, zero `.unwrap()` on untrusted inputs, `dev-guest-auth` release compile-error guard, and golden `benchmarks/` parity.
- **Angular & Node.js (`web/**`, `services/**`):**
  - `pnpm lint && pnpm typecheck && pnpm test --run`
  - `pnpm audit --ignore-scripts`
  - Verify separate `.component.ts`/`.component.html` (`<= 150 LOC`, nesting `<= 2`, zero `@if`-soup/duplication)/`.logic.ts`/`.model.ts` files, zero inline `template:`/`styles:`, zero component-local `.css`/`.scss` outside `design-system`/`ui-kit` without `/* @sig-style-exception */`, zero `Number()` casts on `bigint`, zero `NgModule`/legacy decorators, zero `bypassSecurityTrust*`/`eval`/`child_process.exec`, zero browser disk design storage, and zero `/export`/`/download`/`/print` routes.

## 3. Anti-Hallucination Verification & 2–3 Sentence Reporting
- **Observed Ground Truth Only:** Never report a check as passing unless you executed it after your final edit and observed exit code `0`; report exact failing command output or `"Unverified — <reason>"` if skipped.
- **2–3 Sentence Dev Summary:** After completing a coding fix/PR gate, respond to the user in **2–3 sentences max** (`Issue -> Fix -> Verification command & result`), updating ONLY your own journal (`coordination/agents/antigravity.md` OR `coordination/agents/cursor.md`) and committing with a clean Conventional Commit (`docs:`, `contract:`, `feat:`, `fix:`, `refactor:`, `chore:`).
