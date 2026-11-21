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

**TypeScript / Angular `v22+` (`web/`):**
```ts
/**
 * Module: <Concise 1-line responsibility>
 * Invariants: <e.g. Angular v22+ Zoneless Signals, bigint nm coords, zero browser storage>
 * Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md | contracts/proto/signal/v1/<file>.proto
 */
```

**Python `3.15+` FastAPI (`services/` or `ai/`):**
```python
"""Module: <Concise 1-line responsibility>
Invariants: <e.g. Depends(require_*) uniform 404, with_rls_tx NOBYPASSRLS, SecretStr hygiene>
Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md | contracts/openapi/control-plane.v1.yaml
"""
```

---

## 3. Predictable Naming & Directory Symmetry

Consistent file names make `rg --files` instant and remove guesswork:

| Stack | Canonical File Suffixes / Structure | Anti-Pattern (Forbidden) |
|---|---|---|
| **Rust `1.100+` (`engine/crates/*`)** | `lib.rs` (crate root + re-exports only), `types.rs`, `error.rs`, `kernel.rs`, `stamping.rs`, `solver.rs`, `tests/` (integration) + inline `#[cfg(test)] mod tests` (unit) | Dumping 1,500 lines into `lib.rs` or creating generic `utils.rs` / `common.rs` grab-bags |
| **Angular `v22+` (`web/libs/<lib>/`)** | `src/index.ts` (sole public barrel), `feature/<name>/<name>.{component.ts,component.html,logic.ts,model.ts}`, `ui/<name>/*`, `data-access/*.{store,client}.ts` | Cross-library deep imports (`../../other-lib/src/internal/...`), component-local CSS, `shared-everything` dumping grounds |
| **Python `3.15+` FastAPI (`services/<svc>/`)** | `src/<pkg>/app.py`, `routers/*.py`, `security/*.py`, `services/*.py`, `repositories/*.py`, `schemas/*.py`, `clients/*.py` | Mixing SQL queries, HTTP parsing, and business policy in one router file or `helpers.py` grab-bags |

---

## 4. AI Agent Token-Efficiency Protocol (Mandatory for Antigravity & Cursor)

1. **Search Before Reading:** Run `rg -n` / `rg --files` to locate exact symbol definitions and line ranges before calling `view_file`—never open 800-line files speculatively.
2. **Read Narrow Slices:** Inspect module headers (`lines 1–20`), public interfaces (`lib.rs` / `index.ts` / `__init__.py`), and the specific target function range rather than ingesting entire unrelated files.
3. **Batch Independent Calls:** Issue parallel read/search calls for independent files; issue surgical `replace_file_content` edits rather than full-file overwrites.
4. **Never Paste Generated Code Into Context:** Keep `contracts/` codegen outputs in `target/` and `dist/`/`generated/` (gitignored); inspect `.proto` / `.yaml` source files directly instead of reading thousands of generated boilerplate lines.
5. **Run Narrowest Feedback Loop First:**
   - Rust `1.100+`: `cargo test -p <crate> <test_name>` → then `cargo clippy -p <crate> --all-targets -- -D warnings` → full workspace check only at PR readiness.
   - Angular `v22+`: `pnpm --filter <pkg> test <spec>` → `typecheck` → full workspace check at PR readiness.
   - Python `3.15+` FastAPI/AI: `uv run pytest services/<svc>/tests/<test>.py` → `uv run ruff check` & `uv run pyright` → full suite at PR readiness.
6. **Zero Redundant Narration in Code:** Write concise doc comments explaining *why* (mathematical invariance, physical law, security boundary, `// SAFETY:`), never *what* obvious syntax already states.
7. **Deterministic Diffs:** Never reformat unrelated files, reorder unrelated imports, or touch files outside declared `touched_paths`.

---

## 5. Ultra-Compact Coding Response Standard (2–3 Sentences Max)

During active code development, debugging, refactoring, or bug-fixing turns, **both Antigravity and Cursor MUST limit final responses to 2–3 concise sentences** to eliminate output-token waste:
1. **Sentence 1 — Issue / Scope:** State the root cause or task scope with exact file/symbol reference.
2. **Sentence 2 — Fix Applied:** State the exact code change made.
3. **Sentence 3 — Verification:** State the exact check run and observed pass/fail outcome (or explicitly note if unverified/blocked).

- **Do NOT** dump unrequested multi-paragraph summaries, repeated code blocks already visible in the diff, or lengthy narration during coding turns.
- **Exception:** Expand into structured tables/sections ONLY when the user explicitly requests an architecture review, design proposal, or detailed walkthrough.

---

## 6. Anti-Hallucination Best Practices (Evidence-First Engineering)

1. **Read Before Referencing:** Never invent or guess file paths, crate names, symbol signatures, `.proto` fields, Angular `v22+`/FastAPI APIs, or CLI flags—ALWAYS verify with `rg` / `view_file` first.
2. **Verify Exact Installed Versions:** Check `Cargo.toml`, `package.json`, `pyproject.toml`, `rust-toolchain.toml`, or official docs before using version-sensitive **Rust `1.100+`** (2024 Ed.), **Angular `v22+`**, **Python `3.15+` FastAPI/Pydantic v2**, `wgpu`, or **K8s `1.37+`** APIs.
3. **Observed Execution Is Ground Truth:** Never claim `"fixed"`, `"compiles"`, or `"tests pass"` unless you ran the exact verification command in the current turn and observed exit code `0`. If a check could not run, state `"Unverified — <reason>"`.
4. **Never Extrapolate Physical or Contract Constants:** Ground every SI unit, pinout, tolerance, register address, or error code strictly in `contracts/`, `architecture/01..21-*`, or verified `pins.json` / datasheet citations—never recall hardware specs from model memory.
5. **Fail-Closed on Ambiguity:** If a contract, requirement, or spec is missing or contradictory, STOP and ask one direct question (or log `BLOCKED_ON_CONTRACT`) instead of inventing silent assumptions.
6. **Narrow Grounded Edits:** Re-read the exact target line range before patching so edits never fabricate surrounding context or overwrite peer changes.
