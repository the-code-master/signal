---
name: signal-contract-first
description: Authors, evolves, and validates Signal's canonical API and data contracts under contracts/ (Buf Protobuf signal/v1/*, OpenAPI control-plane.v1.yaml, JSON schemas, golden bigint fixtures, and stateful JSONL transcripts). Use whenever defining or modifying inter-service interfaces before Antigravity and Cursor implement parallel lanes.
---

# Signal Contract-First Workflow (`contracts/**`)

Governing spec: `architecture/10-api-and-data-contracts.md`.

## Mandatory Branching Rule
- Always use a serialized branch `contract/<task-id>-<slug>` touching **ONLY** `contracts/**` (plus your own agent journal).
- Merge the contract PR to `main` **BEFORE** parallel Lane A (`engine/`) and Lane B (`web/`, `services/`) branches build against it.

## Hard Schema & Wire Invariants
1. **Explicit SI-Unit Field Suffixes:** Every physical value MUST end in `_nm`, `_ps`, `_fs`, `_uv`, `_na`, `_uw`, `_mkelvin`, `_hz`, or `_mohm`. Reject ambiguous bare names (`width`, `voltage`, `delay`, `temp`).
2. **Mandatory 64-Bit `bigint` Safety (Zero `2^53` Corruption):**
   - Annotate 100% of `int64`, `uint64`, `sint64`, and `fixed64` fields (nanometers, femtoseconds, `fencing_epoch`, `provider_repo_id`) with `[jstype = JS_STRING]`.
   - Configure TypeScript Protobuf codegen with `longAsBigint: true`.
3. **Chunked Binary & Arrow Framing:** Cap stream payload chunks at `<= 1 MB` with CRC32C/AEAD framing and credit-based backpressure; use Apache Arrow RecordBatches for waveforms, FMEDA matrices, Shmoo grids, and visual diffs.
4. **Additive-Only `v1` Evolution (`N-1` Compatibility):** Never reuse or retype an existing field number; explicitly `reserve` retired numbers/names.
5. **Zero-Export OpenAPI Gate:** Every Control Plane endpoint MUST be URL-scoped (`/org/:org_slug/...`), require `Idempotency-Key` on mutations, and NEVER add `/export`, `/download`, `/raw-file`, or `/print` routes.
6. **Build-Time Codegen Only:** NEVER commit generated Rust (`tonic`/`prost`) or TypeScript stubs to Git.

## Required Conformance Artifacts (Every Contract PR)
1. **Golden Fixtures (`contracts/fixtures/*.json`):** Include edge values `> 9_007_199_254_740_991` (`> 2^53`) to prove lossless Rust `i64` ↔ TypeScript `bigint` round-tripping.
2. **Stateful Protocol Transcripts (`contracts/transcripts/*.jsonl`):** Record exact multi-step request/response/error transitions (e.g., `HydrateProject -> MOUNTED -> ApplyTransaction`, stale-epoch `412`, `Idempotency-Key` retry) replayed by both Rust and TS CI suites.

## Verification Checklist
```bash
buf lint contracts/proto
buf breaking contracts/proto --against '.git#branch=main,subdir=contracts/proto'
# Validate OpenAPI + JSON schemas + fixture/transcript tests
```
- Confirm `git status` contains zero generated `.rs` or `.ts` stub files.

## Dev Response & Anti-Hallucination
- Inspect existing `.proto` field numbers, reserved ranges, and schemas via `view_file` before editing; never guess wire types or claim `buf` checks passed without running them.
- Reply in **2–3 sentences max** stating ONLY the contract change/issue, fix applied, and observed `buf`/fixture result.
