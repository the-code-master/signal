---
name: signal-rust-crate-architect
description: Scaffolds and builds clean, idiomatic, human-maintainable Rust 2024 crates inside engine/crates/*. Use when creating a new Rust crate or module, designing crate boundaries, error enums, SecretBox secret handling, async/rayon concurrency, or managing Cargo workspace dependencies and supply-chain vetting.
---

# Signal Rust Crate Architect (`engine/crates/*`)

Reference: `docs/engineering/RUST_ENGINEERING_GUIDE.md` | Rules: `.cursor/rules/rust-engine.mdc`.

## Crate Scaffolding Blueprint
Create every new crate under `engine/crates/signal-<domain>/` with this clean, minimal layout:
```text
engine/crates/signal-<domain>/
├── Cargo.toml           # Workspace-inherited edition/lints; minimal scoped deps + workspace-hack
└── src/
    ├── lib.rs           # 3-line header, #![forbid(unsafe_code)], concise pub use re-exports only
    ├── types.rs         # Strongly-typed SI newtypes & domain structs (<= 250 LOC)
    ├── error.rs         # thiserror::Error typed enum (never anyhow in public library APIs)
    ├── model.rs         # Pure deterministic domain logic / math (side-effect free)
    └── kernel.rs        # Adapter bridging signal-graph ECS snapshots <-> contracts/Arrow
```

## Non-Negotiable Rust Engineering Guardrails
1. **Safety:** Declare `#![forbid(unsafe_code)]` at the top of `lib.rs`. Zero `.unwrap()`, `.expect()`, or unchecked `[i]` indexing on untrusted inputs.
2. **Crate Layering (Acyclic DAG):**
   - Tier 0 (`signal-core`, `signal-proto`) ← Tier 1 (`signal-graph`, `signal-geom`, `signal-component`) ← Tier 2 (`signal-lib-*`, `signal-module-sdk`) ← Tier 3 (physics/domain solvers) ← Tier 4 (`signal-render`, `signal-git-sync`, `signal-cell-wasm`, `signal-pod`, `signal-operator`).
   - Core/solver crates NEVER depend on `tonic`, `wgpu`, or `signal-pod`.
3. **Single Protobuf Crate:** Consume generated types exclusively from `signal-proto` — never add duplicate `tonic-build` scripts.
4. **Secret Hygiene:** Wrap tokens, DEKs, PDKs, and confidential net names in `secrecy::SecretBox<T>` + `zeroize::ZeroizeOnDrop`; implement redacted `Debug`; never emit secrets in `tracing` events.
5. **Concurrency Safety:** Offload CPU-heavy solvers/crypto from `tokio` onto bounded `rayon` pools or `spawn_blocking` with `CancellationToken` checks; NEVER hold a `Mutex`/`RwLock` across `.await`.
6. **LLVM-Free Pod:** Never link LLVM or `dlopen` vendor `.so`/`.dll` binaries in crates linked by `signal-pod`.
7. **Supply Chain:** Add dependencies at crate level (not blindly to every crate), refresh `cargo hakari generate` if workspace-hack changes, and audit every new crate via `cargo deny check` and `cargo vet`.

## Verification Commands
```bash
cargo fmt --all -- --check
cargo clippy -p signal-<domain> --all-targets -- -D warnings
cargo test -p signal-<domain>
cargo deny check && cargo vet
```
