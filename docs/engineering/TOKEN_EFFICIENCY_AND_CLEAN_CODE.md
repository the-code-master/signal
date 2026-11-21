# Token Efficiency & Human-Maintainable Clean Code Standard

> **Applies To:** Every file in `engine/`, `web/`, `services/`, `ai/`, `contracts/`, and `infra/`.  
> **Empirical Rationale:** Clean, modular, predictably structured code cuts AI agent token consumption, eliminates repeated file re-reads, lowers review friction for human engineers, and prevents silent regressions across parallel Antigravity + Cursor branches.

---

## 1. Structural Budgets (Hard Soft-Limits for Maintainability & Token Economy)

| Dimension | Target Budget | Hard Split Trigger | Why It Matters |
|---|---|---|---|
| **Source File Size** | `80 – 250 LOC` | `> 350 LOC` (must decompose) | Fits comfortably in one concise read without spilling context windows or forcing multi-chunk edits. |
| **Function / Method Length** | `10 – 35 LOC` | `> 60 LOC` | Single-responsibility functions are trivially unit-testable and readable at a glance. |
| **Cyclomatic Nesting Depth** | `<= 2` levels | `> 3` levels | Use early returns (`guard` clauses), Rust `?` / `let ... else`, and small pure helpers. |
| **Function Parameters** | `<= 4` params | `>= 5` params | Group related parameters into a typed `struct` (`Rust`) or `readonly` options `interface` (`TS`). |
| **Public Exports per Module** | Minimal surface | Zero leaked internals | Keep internal helpers `pub(crate)` (Rust) or unexported from `index.ts` (TS) so callers never couple to internals. |
| **Component / Kernel Responsibility** | 1 domain concept per file | Never "god files" (`utils.ts`, `misc.rs`, `helpers.rs`) | Eliminates ambiguous search hits and prevents merge conflicts across parallel branches. |

---

## 2. Mandatory 3-Line Module Header (Fast Context Discovery)

Every non-trivial source file MUST open with a crisp **3-line header** summarizing purpose, owned invariants, and governing contract/spec—allowing both humans and AI agents to verify relevance in `< 40` tokens without reading the full file!

**Rust (`engine/crates/<crate>/src/<module>.rs`):**
```rust
//! Module: <Concise 1-line responsibility>
//! Invariants: <Key safety/determinism/unit invariants, e.g. i128 predicates, #![forbid(unsafe_code)]>
//! Spec: architecture/03-rust-eda-engine.md | contracts/proto/signal/v1/<file>.proto
```

**TypeScript / Angular (`web/` or `services/`):**
```ts
/**
 * Module: <Concise 1-line responsibility>
 * Invariants: <e.g. Zoneless Signals, bigint nm coords, zero browser storage, uniform 404>
 * Spec: architecture/06-frontend-angular-and-nodejs-bff.md | contracts/proto/signal/v1/<file>.proto
 */
```

---

## 3. Predictable Naming & Directory Symmetry

Consistent file names make `rg --files` instant and remove guesswork:

| Stack | Canonical File Suffixes / Structure | Anti-Pattern (Forbidden) |
|---|---|---|
| **Rust (`engine/crates/*`)** | `lib.rs` (crate root + re-exports only), `types.rs`, `error.rs`, `kernel.rs`, `stamping.rs`, `solver.rs`, `tests/` (integration) + inline `#[cfg(test)] mod tests` (unit) | Dumping 1,500 lines into `lib.rs` or creating generic `utils.rs` / `common.rs` grab-bags |
| **Angular (`web/libs/<lib>/`)** | `src/index.ts` (sole public barrel), `feature/*.component.ts`, `ui/*.component.ts`, `data-access/*.store.ts`, `data-access/*.client.ts`, `models/*.model.ts`, `utils/*.fn.ts` | Cross-library deep imports (`../../other-lib/src/internal/...`), `shared-everything` dumping grounds |
| **Node.js (`services/<svc>/`)** | `src/main.ts`, `routes/*.route.ts`, `handlers/*.handler.ts`, `policies/*.guard.ts`, `repos/*.repo.ts`, `types/*.dto.ts` | Mixing SQL queries, HTTP parsing, and business policy in one controller file |

---

## 4. AI Agent Token-Efficiency Protocol (Mandatory for Antigravity & Cursor)

1. **Search Before Reading:** Run `rg -n` / `rg --files` to locate exact symbol definitions and line ranges before calling `view_file`—never open 800-line files speculatively.
2. **Read Narrow Slices:** Inspect module headers (`lines 1–20`), public interfaces (`lib.rs` / `index.ts`), and the specific target function range rather than ingesting entire unrelated files.
3. **Batch Independent Calls:** Issue parallel read/search calls for independent files; issue surgical `replace_file_content` edits rather than full-file overwrites.
4. **Never Paste Generated Code Into Context:** Keep `contracts/` codegen outputs in `target/` and `dist/`/`generated/` (gitignored); inspect `.proto` / `.yaml` source files directly instead of reading thousands of generated boilerplate lines.
5. **Run Narrowest Feedback Loop First:**
   - Rust: `cargo test -p <crate> <test_name>` → then `cargo clippy -p <crate> --all-targets -- -D warnings` → full workspace check only at PR readiness.
   - Angular/Node: `pnpm --filter <pkg> test <spec>` → `typecheck` → full workspace check at PR readiness.
6. **Zero Redundant Narration in Code:** Write concise doc comments explaining *why* (mathematical invariance, physical law, security boundary, `// SAFETY:`), never *what* obvious syntax already states.
7. **Deterministic Diffs:** Never reformat unrelated files, reorder unrelated imports, or touch files outside declared `touched_paths`.
