---
name: signal-rust-physics-and-geometry
description: Implements deterministic EDA physics, ElectronicModule components, MNA SPICE, Filippov Saltation/Revolve Adjoints, exact i128/Shewchuk geometry, curved waveguides, and extended domain solvers (magnetics, PDN, creepage, EMC/FMEDA, IC, FPGA, HW security) in Rust. Use when writing numerical solvers, component models, DRC/LVS kernels, or golden physics benchmarks.
---

# Signal Rust Physics, Components & Exact Geometry

Specs: `architecture/03-rust-eda-engine.md`, `architecture/16-*`, `architecture/17-*`.

## 1. Unified `ElectronicModule` Component Contract (`signal-component`)
- Implement typed multi-physics conservation ports (`Electrical`, `Thermal`, `DigitalLogic`, `RFWave`, `OpticalEnvelope`, `MagneticFlux`, `Mechanical`, `QuantumFlux`).
- Support condition-parameterized curves (`C(V_bias, T, f)`, copper-coupled `R_thetaJC/JB`), Pelgrom spatial mismatch (`A_VT / sqrt(W*L)` + centroid distance), 15-year aging (`signal-aging`), `4 K–10 mK` cryo-CMOS band-tail regularization, autonomous SOA watchdogs (`evaluate_soa`), and fault injection (`apply_fault` for FMEDA).
- Never hardcode scalar single-point MLCC/thermal derating or bypass the unified port graph.

## 2. Numerical Stability & Adjoint Solver Rules (`signal-sim`, `signal-diff-sim`)
- **Charge/Flux Conservation:** Stamp exact `q(v)` and `Φ(i)` states (`dq/dt`, `dΦ/dt`) with `C^2`-smooth limiting (`pnjlim`/`fetlim`), TR-BDF2 integration, and zero-crossing breakpoint root-finding — NEVER naive `C(V)dV/dt`.
- **Ill-Conditioning Immunity:** Apply Ruiz row/column equilibration, Neumaier compensated KCL summation, rook threshold pivoting, and automatic `f128` iterative refinement when `cond > 1e12`.
- **Workload-Aware Sparse LU:** Dispatch `< 25k` nodes to pure-Rust CPU Block-BTF `KLU`/`CKTSO`; dispatch `>= 25k` nodes to batched GPU device evaluation + `cuDSS` (`FP64` on H100/B200 or `FP32` preconditioner + `f64`/`f128` refinement on L4/A10G).
- **Hybrid Switching Adjoints:** Across PWM/comparator switching surfaces (`sigma(x,t,p) = 0`), multiply adjoint sensitivities by exact **Filippov Saltation Matrices**, warm-start via Two-Stage PSS/HB (`TFHA`), and cap trajectory RAM to $O(\log N_t)$ (`< 64 MB`) via **Binomial `Revolve` checkpointing**.
- **Optical & RF:** Use Complex Baseband Optical Envelope (`E_env(t)`) for CPO (never `0.2 fs` carrier stepping) and passive/causal Hamiltonian Vector-Fitted `.sNp` macromodels.

## 3. Exact Geometry & Cross-ISA Determinism (`signal-geom`)
- Store compact `Nanometers(i64)` coordinates; ALWAYS promote cross-products, orientation, and intersection determinants to **exact `i128` or Shewchuk adaptive predicates** (zero overflow on `> 3 m` backplanes!).
- Preserve native **Circular Arc, Cubic Bézier, and Adiabatic Euler-Bend** primitives; enforce instanced DAG traversal depth `<= 64`.
- **Cross-ISA Bitwise Parity (`ARM64 Mac == x86_64 Linux`):** Use `core_math` (`0.5-ULP`), disable FMA contraction where required, iterate canonically (`BTreeMap`/sorted CSR), use deterministic `rayon` tree-reductions, and seed randomness per `SignalEntityId` with `ChaCha20Rng`.

## 4. Verification & Benchmarks
- Unit-test every new `ElectronicModule` analytical Jacobian against central finite differences (`< 1e-6` relative error) and verify KCL/charge conservation.
- Add or update golden parity fixtures in `benchmarks/` (NGSPICE/XYCE `< 0.2%` parity, Sky130 DRC/LVS, exact geometry edge cases).
