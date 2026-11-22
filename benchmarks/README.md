# Signal Golden Physics & Geometry Benchmark Suite (`benchmarks/`)

> **Governing Specs:** [`03-rust-eda-engine.md`](../architecture/03-rust-eda-engine.md) • [`16-architectural-audit-flaws-and-futuristic-innovations.md`](../architecture/16-architectural-audit-flaws-and-futuristic-innovations.md) (`F-110`)

## Invariants
1. Every MNA SPICE (`signal-sim`), Differentiable Adjoint (`signal-diff-sim`), 3D EM (`signal-em3d`), and Exact Geometry (`signal-geom`) solver MUST pass golden reference comparisons within `<= 0.1%` relative and `<= 1 uV` / `<= 1 nm` absolute tolerance against NGSPICE / XYCE / analytical closed-form solutions.
2. Cross-ISA (`x86_64` vs `aarch64`) outputs must remain within `<= 0.5 ULP` under `physics_determinism_epoch` (`F-033`, `20-*`).
