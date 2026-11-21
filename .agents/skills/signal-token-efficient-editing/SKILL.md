---
name: signal-token-efficient-editing
description: Enforces Signal's token-efficient context engineering and human-maintainable clean-code standards across Rust, Angular, Node.js, and Markdown. Use when navigating the codebase, designing new modules, refactoring large files, or making multi-file code edits to minimize token waste and keep files clean and modular.
---

# Signal Token-Efficient Editing & Clean Code

Reference: `docs/engineering/TOKEN_EFFICIENCY_AND_CLEAN_CODE.md`.

## Context Discovery Protocol (Save 50%+ Input Tokens)
1. **Locate Before Reading:** Use `rg --files` and `rg -n "<symbol>"` to pinpoint exact paths and line ranges before opening any file.
2. **Read Headers & Boundaries First:** Inspect lines `1–25` (the 3-line module header + imports) and public exports (`lib.rs` in Rust, `index.ts` in TS) before reading implementations.
3. **Never Ingest Generated Artifacts:** Read source `.proto`, `.yaml`, and `.schema.json` files under `contracts/` — NEVER read generated code in `target/` or `generated/`.
4. **Parallel Independent Reads:** Batch unrelated `view_file` or search calls in one turn; use surgical `replace_file_content` patches instead of full-file rewrites.

## Clean Code Size & Modularity Budgets
- **Source File Ceiling:** Target `80–250 LOC`; **split any file exceeding `350 LOC`** along cohesive domain seams.
- **Function Ceiling:** Target `10–35 LOC`; split above `60 LOC`. Keep nesting `<= 3` via early returns (`?`, `let ... else`, guard clauses).
- **Parameter Ceiling:** Group `>= 5` parameters into a typed `struct` (Rust) or `readonly` options `interface` (TypeScript).
- **Mandatory 3-Line Module Header:** Every new/refactored source file opens with:
  - Rust: `//! Module: ...`, `//! Invariants: ...`, `//! Spec: ...`
  - TS: `/** Module: ... | Invariants: ... | Spec: ... */`
- **No Grab-Bag Files:** Never create `utils.ts`, `helpers.rs`, `misc.rs`, or `common.ts`. Name files by exact domain role (`stamping.rs`, `creepage.rs`, `pdn.store.ts`, `meander-hud.directive.ts`).

## Fast Feedback Loop Discipline
- Run the narrowest verification command during iteration:
  - Rust: `cargo test -p <crate> <test_name>` → `cargo clippy -p <crate> --all-targets -- -D warnings`
  - Web/Node: `pnpm --filter <pkg> test <file>` → `pnpm --filter <pkg> typecheck`
- Escalate to full-workspace checks only once local tests are green.
- Never reformat unrelated files or touch paths outside your declared `touched_paths`.
