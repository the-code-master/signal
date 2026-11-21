---
name: signal-token-efficient-editing
description: Enforces Signal's token-efficient context engineering, ultra-compact 2-3 sentence coding responses, anti-hallucination evidence-first discipline, and human-maintainable clean-code standards across Rust 1.100+, Angular v22+, Python 3.15+ FastAPI, and Markdown. Use during all code development, debugging, refactoring, or multi-file edits to eliminate token waste and prevent hallucinations.
---

# Signal Token-Efficient Editing, 2–3 Sentence Replies & Anti-Hallucination

Reference: `docs/engineering/TOKEN_EFFICIENCY_AND_CLEAN_CODE.md`.

## 1. Mandatory 2–3 Sentence Coding Response Format (Zero Output-Token Waste)
During active code development, debugging, or refactoring turns, keep your final reply strictly to **2–3 concise sentences**:
1. **Issue / Scope:** Root cause or change scope with exact file/symbol reference.
2. **Fix Applied:** Exact modification performed.
3. **Verification:** Exact command run and observed pass/fail outcome (or `"Unverified — <reason>"`).

- NEVER write multi-paragraph summaries, repeated diffs, or unrequested essays during coding turns. Expand ONLY when the user explicitly requests an architecture review or detailed explanation.

## 2. Anti-Hallucination Best Practices (Evidence-First Engineering)
1. **Verify Before Citing:** Use `rg` and `view_file` to confirm every file path, symbol name, `.proto` field, and function signature before referencing or editing—never guess from memory.
2. **Verify Versions & APIs:** Inspect `Cargo.toml`, `package.json`, `pyproject.toml`, or installed docs before using version-sensitive **Rust `1.100+`** (2024 Edition), **Angular `v22+`**, **Python `3.15+` FastAPI/Pydantic v2**, `wgpu`, or **K8s `1.37+`** APIs.
3. **Observed Exit Code 0 Only:** Never claim code compiles or tests pass unless you ran the exact check in this turn and saw exit code `0`. Report exact error output or `"Unverified"` otherwise.
4. **Ground Physical & Contract Facts:** Source all SI units, pinouts, tolerances, and register maps strictly from `contracts/`, `pins.json`, or `architecture/01..21-*`—never extrapolate hardware specs from model memory.
5. **Fail-Closed on Ambiguity:** If requirements or contracts are missing/contradictory, STOP and ask one crisp question (or log `BLOCKED_ON_CONTRACT`) rather than fabricating assumptions.

## 3. Input-Token & Clean-Code Discipline
- **Locate Narrowly:** Run `rg --files` / `rg -n` first; read module headers (`lines 1–25`) and public barrels (`lib.rs` / `index.ts` / `__init__.py`) before full files; never read generated stubs in `target/` or `generated/`.
- **Batch & Patch:** Batch independent reads in parallel; apply surgical edits instead of full-file rewrites; run single-crate/single-spec tests before full-workspace suites.
- **File & Function Budgets:** Keep files `80–250 LOC` (hard split `> 350 LOC`), functions `<= 40–60 LOC`, nesting `<= 3`, params `<= 4`, mandatory 3-line `Module / Invariants / Spec` header, and zero grab-bag `utils.ts`/`helpers.rs`/`helpers.py` files.
