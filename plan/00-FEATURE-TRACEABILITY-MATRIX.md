# 00 — Complete Zero-Miss Feature, Crate, Service, Workbench & Audit Traceability Matrix

> **Purpose:** Guarantees mathematically that **0 features are missed** across Signal's 9 implementation phases (`Phase 0` – `Phase 8`). Every specification (`01`–`21`), every baseline product capability, every Rust crate (`31`), every FastAPI service (`11` + Envoy Bridge + Huddle SFU), every AI service (`4`), every Angular `v22+` workbench library (`23` + IDE Shell), all **`120` extended features (`FEAT-001`–`FEAT-120`)**, all **`12` innovations (`I-01`–`I-12`)**, and all **`148` audit flaw mitigations (`F-001`–`F-148`)** map to explicit, tickable Task IDs in [`TODO-LEDGER-SHARED-CONTRACTS.md`](./TODO-LEDGER-SHARED-CONTRACTS.md), [`TODO-LEDGER-ANTIGRAVITY-LANE-A.md`](./TODO-LEDGER-ANTIGRAVITY-LANE-A.md), and [`TODO-LEDGER-CURSOR-LANE-B.md`](./TODO-LEDGER-CURSOR-LANE-B.md).

---

## 1. Architecture Specifications (`01`–`21`) & Baseline Product Pillars → Phase & Task Map

| Spec | Baseline Capabilities Included | Phase(s) | Contract Tasks | Antigravity (Lane A) Tasks | Cursor (Lane B) Tasks |
|---|---|---|---|---|---|
| **[`01`](../architecture/01-product-vision-and-features.md)** | PRD/HFS executable specs, Schematic & Symbol, Equilibrated/Differentiable SPICE, Formal, 224G PCB + Fab Profiles + ECO/ECN, 3D EM/Aging, IC/UPF/Pelgrom/CPO/Cryo/IEEE 1735, FPGA, FW/BIOS, Lab Digital Twin, Clean-Room Import, Mirrored BOM, Multi-Type Projects | `P0`–`P8` | `C-P0-01`..`C-P8-01` | `AG-P1-01`..`AG-P8-04` | `CU-P1-01`..`CU-P8-04` |
| **[`02`](../architecture/02-system-architecture.md)** | 5-Plane Cloud Topology, Envoy HTTP/3 WebTransport + `turns:443` relay, In-Jurisdiction Paired-Region DR, Monorepo layout | `P0`, `P2`, `P8` | `C-P0-01` | `AG-P0-04`, `AG-P2-03`, `AG-P8-04` | `CU-P0-04`, `CU-P2-03` |
| **[`03`](../architecture/03-rust-eda-engine.md)** | `ElectronicModule`, Ruiz `KLU`↔`cuDSS` MNA, Filippov Saltation + `Revolve` Adjoint, PSS/HB, AOT Cranelift Verilog-A, child-MicroVM `OSDI`/`IBIS-AMI`, `i128`/Shewchuk geometry, `0.5-ULP` `core_math`, `Zstd-QOI`/WebP GPU renderer | `P0`, `P1`, `P4`, `P6` | `C-P0-01`, `C-P1-01`, `C-P4-01`, `C-P6-01` | `AG-P0-01`, `AG-P1-01`..`AG-P1-06`, `AG-P4-01`..`AG-P4-06`, `AG-P6-01`..`AG-P6-06` | `CU-P1-01`..`CU-P1-04` |
| **[`04`](../architecture/04-kubernetes-runtime-and-collaboration.md)** | `< 8 MB` `signal-cell-wasm`, Host-Aggregated K8s Leases, `< 150 ms` 3-Phase Pre-Copy Live Migration, Topological CRDT & Selective Undo, Exclusive `H100 CC` / `B200 TEE-I/O` MIG | `P1`, `P2`, `P8` | `C-P0-01`, `C-P2-01` | `AG-P1-03`, `AG-P2-03`, `AG-P2-04`, `AG-P8-04` | `CU-P2-03` |
| **[`05`](../architecture/05-ai-agent-and-datasheet-rag.md)** | Neuro-symbolic orchestrator, Multimodal VLM + Dual-LLM Zero-Tool Quarantine, `GBNF` grammar decoding, Merge-Time Proof Delta Re-Verification, Dual-VLM `C(V,T,f)` Datasheet DB, 3-Tier Sharded `pgvector` HNSW (`<= 500` partitions) | `P3` | `C-P3-01` | `AG-P3-01`..`AG-P3-05` | `CU-P3-01`..`CU-P3-05` |
| **[`06`](../architecture/06-frontend-angular-and-fastapi-control-plane.md)** | Angular `v22+` ONLY Zoneless Signals IDE, Dual-Plane `Zstd-QOI`/WebP + `HEVC 4:4:4`/Dual-Luma stream, `HudOverlayPrimitive`, Route-Partitioned `COOP`/`COEP`, `bigint`, Python `3.15+` FastAPI/Granian/Pydantic v2 Control Plane (ADR-002) + Tier-0 Quorum Audit Outbox | `P0`, `P1`, `P4` | `C-P0-01`..`C-P0-03` | `AG-P1-02`, `AG-P4-06` | `CU-P0-01`..`CU-P0-05`, `CU-P1-01`..`CU-P1-06`, `CU-P4-05` |
| **[`07`](../architecture/07-security-encryption-dlp-and-sftp.md)** | Hybrid PQC (`ML-KEM-768` + `ML-DSA-65`) + 2-level `HKDF`, Padmé-blinded `.signal/vault.merkle.sig` `AES-256-GCM-SIV`, Tardos + Constellation watermarks, Attested Browser DLP, Step-Up 2FA (WebAuthn + Redis-burned TOTP), write-only `russh` SFTP, RFC-3161 pseudonymous WORM logs | `P0`, `P1`, `P2`, `P4`, `P8` | `C-P0-02`, `C-P2-01`, `C-P4-02` | `AG-P1-01`, `AG-P2-01`, `AG-P4-06`, `AG-P4-07`, `AG-P8-04` | `CU-P0-05`, `CU-P1-01`, `CU-P2-01`, `CU-P4-05`, `CU-P4-06`, `CU-P8-04` |
| **[`08`](../architecture/08-identity-enterprise-and-project-model.md)** | Dual SSO + PKCE Cookie Upgrade, `ORG_NAMESPACE` & `USER_NAMESPACE`, Paired-Region DR, 2-of-3 Break-Glass, uniform `404`, contractor `expires_at`, Org/Project Admin Access Review, Multi-Type Projects, Templates, Fab Profiles, ECN & 21 CFR Part 11 | `P0`, `P2`, `P4`, `P5`, `P8` | `C-P0-02`, `C-P2-01` | `AG-P2-01`, `AG-P4-01`, `AG-P5-05` | `CU-P0-04`, `CU-P2-01`..`CU-P2-03`, `CU-P2-06`, `CU-P5-01`, `CU-P8-04` |
| **[`09`](../architecture/09-local-development-environment.md)** | macOS Apple Silicon `./signal.sh dev [lite\|full\|k8s]` (`< 1.5 GB RAM`, `< 2 s` incr Rust via `signal-proto`), `lavapipe` Vulkan CI, `GBNF` Local Ollama/MLX + `deterministic_mock`, triple-guarded zero-bypass `Guest @ Signal Design, Inc.` | `P0` | `C-P0-01` | `AG-P0-01`, `AG-P0-03`, `AG-P0-05` | `CU-P0-03`, `CU-P0-04` |
| **[`10`](../architecture/10-api-and-data-contracts.md)** | Build-time codegen (`signal-proto`, `uv`, `pnpm`), SI-unit suffixes (`_nm`, `_ps`, `_uv`), mandatory `[jstype = JS_STRING]` / ES2020 `bigint` / exact Python `int`, `<= 1 MB` Arrow IPC chunks, `contracts/fixtures/` & `contracts/transcripts/*.jsonl` | `P0`–`P8` | `C-P0-01`..`C-P8-01` | `AG-P0-01` | `CU-P0-01`, `CU-P0-04` |
| **[`11`](../architecture/11-implementation-roadmap.md)** | Phased implementation sequence, golden `benchmarks/` physics suites, verification exit gates, Wave 1 + Wave 2 risk mitigations | `P0`–`P8` | All `C-P*` | All `AG-P*` | All `CU-P*` |
| **[`12`](../architecture/12-multi-ide-collaboration.md)** | Antigravity (Lane A) + Cursor (Lane B) disjoint swimlanes, single-writer journals & TODO ledgers, immutable handoffs, `> 24h` stale-claim takeover, supply-chain gates (`cargo vet`/`deny`, `pip-audit`, `pnpm audit --ignore-scripts`) | `P0`–`P8` | All `C-P*` | `AG-P0-01`, `AG-P0-03` | `CU-P0-01`, `CU-P0-04` |
| **[`13`](../architecture/13-vcs-git-server-and-storage-analysis.md)** | Unified BYO-Git (`GitHub`, `GitLab`, `Bitbucket`), immutable `provider_repo_id`, lazy `<= 40/min` repo pacer, blobless clones + `64 KB` LFS, Padmé-blinded `ENCRYPTED_VAULT`, Dual-Fenced 2PC `Super-Commit` (`repos.lock.json`), external `git push` auto-rebase, BYOK Continuity WAL, `vcs-auditor` | `P2` | `C-P2-01` | `AG-P2-01`, `AG-P2-02` | `CU-P2-02`, `CU-P2-06` |
| **[`14`](../architecture/14-universal-scale-accessibility-and-expandability.md)** | `< 8 MB` Wasmtime `EXPLORER` isolates, volatile `<= 60 s` RAM reconnect buffer, `< 10 KB` sim recipes, 100B-transistor `*.lef.sig`/`*.halo.gds.sig` `signal-federation-broker`, Baseband CPO, 4K Cryo-CMOS, signed WASM multi-physics plugin SDK | `P1`, `P2`, `P6`, `P8` | `C-P2-01`, `C-P6-01`, `C-P8-01` | `AG-P2-03`, `AG-P6-01`, `AG-P6-02`, `AG-P8-01` | `CU-P1-01`, `CU-P1-05`, `CU-P8-01`, `CU-P8-02` |
| **[`15`](../architecture/15-application-security-and-project-isolation.md)** | OWASP ASVS L3, "1 Project = 1 Security Cell", Cilium 100% lateral drop, socket-pinned + epoch-fenced `signal-egress-proxy`, startup `Depends(require_*)` uniform-`404` gate, Postgres `19+` RLS (`NOBYPASSRLS`), child-MicroVM `OSDI`/`IBIS-AMI`, bomb-capped parsers (`<= 20:1`, depth `<= 64`) | `P0`, `P1`, `P2`, `P3`, `P4`, `P8` | `C-P0-02` | `AG-P0-04`, `AG-P1-04`, `AG-P2-02`..`AG-P2-04`, `AG-P3-05`, `AG-P8-04` | `CU-P0-04`, `CU-P0-05`, `CU-P8-04` |
| **[`16`](../architecture/16-architectural-audit-flaws-and-futuristic-innovations.md)** | All `148` resolved flaws (`F-001`–`F-148`) + all `12` futuristic AI-EDA innovations (`I-01`–`I-12`) | `P0`–`P8` | See Section 4 below | See Section 4 below | See Section 4 below |
| **[`17`](../architecture/17-extended-feature-specifications-ui-and-rust.md)** | All `120` dual Angular `v22+` UI + Pure-Rust `1.100+` features (`FEAT-001`–`FEAT-120`) | `P1`–`P8` | See Section 3 below | See Section 3 below | See Section 3 below |
| **[`18`](../architecture/18-universal-component-and-datasheet-repository.md)** | 3-Tier Federated Universal Parts & Datasheet Repo (Sharded Global Git + BLAKE3 CAS → Encrypted Org BYO-Git `<org>/signal-org-parts` AVL → Project `.signal/parts.lock.json`), Master Pin Table (`pins.json`), IEEE/IEC Dual-Standard Symbols, Parametric Padstacks (`PadstackDef`), IPC-7351 Footprints, 5-Gate 3D STEP pin-to-pad verifier | `P2`, `P3` | `C-P2-02` | `AG-P2-05`, `AG-P2-06`, `AG-P3-01` | `CU-P2-04`, `CU-P2-05` |
| **[`19`](../architecture/19-cad-cam-3d-printing-and-plm-operations.md)** | `CAD_MCAD` Parametric B-Rep CAD & Sheet-Metal, `CAM_CNC` 2.5D–5-Axis Trochoidal CNC, `PRINT_3D` Direct-B-Rep Slicer, Unified `wgpu` 3D View, `PLM_OPS` (`eBOM`/`mBOM`/`sBOM`, `ECR`/`ECO`/`ECN`, `MCN`, `PCN`, `Deviation`/`Waiver` hard SFTP gate, `NCR`/`CAPA`, `CTB`, 3D Assembly Work Instructions, ERP/MES sync) | `P5` | `C-P5-01`, `C-P5-02` | `AG-P5-01`..`AG-P5-05` | `CU-P5-01`..`CU-P5-05` |
| **[`20`](../architecture/20-versioning-release-monorepo-build-and-k8s-topology.md)** | Platform SemVer (`VERSION`), `releases/compatibility.json` (`N-2` window, `physics_determinism_epoch`), `.signal/platform.lock.json` LTS pinning, auto Release Notes (`CHANGELOG.md`, `releases/notes/vX.Y.Z.{md,json}`, In-App What's New), unified `./signal.sh` → `scripts/signal_ops.py` + `components.toml` CI/CD CLI, 3-Tier K8s `1.37+` (`pool-web`, `pool-api`, `1–4 -> N` `pool-rust-engine`) | `P0`, `P2`, `P8` | `C-P0-03` | `AG-P0-03`, `AG-P0-04`, `AG-P2-04` | `CU-P0-03`, `CU-P0-04` |
| **[`21`](../architecture/21-collaboration-space-project-status-and-automated-reviews.md)** | Zero-Leak 5-Team Collaboration Space (`Design`, `Dev`, `Software`, `Manufacturing`, `Ops`), DEK-encrypted RLS messages, `required_repo_scopes[]` RBAC masking, live `@entity` anchors, encrypted WebRTC/viewport huddles (`signal-huddle-sfu`), Project Status Control Tower (stage gates, RAG, dependencies, actions, `ADR`/`EDR`), Automated **Weekly, Monthly Executive & Stage-Gate Reviews** | `P7` | `C-P7-01` | `AG-P7-01`, `AG-P7-02` | `CU-P7-01`..`CU-P7-04` |

---

## 2. Complete Codebase Inventory (`31` Rust Crates, `13` Tier-2 Services, `4` AI Services, `24` Angular Apps/Libs) → Task Map

### 2.1 Pure-Rust `1.100+` Workspace Crates (`engine/crates/*` — Antigravity Lane A)

| # | Rust Crate / Binary | Role | Scaffolded In | Full Implementation Task(s) |
|---|---|---|---|---|
| 1 | `signal-proto` | Single shared build-time Protobuf/gRPC codegen crate (+ `cargo-hakari`) | `AG-P0-01` | `AG-P0-01`, updated on `C-P1..P8` |
| 2 | `signal-core` | Strongly-typed SI units (`m` vs `Meg`), IDs, Hybrid PQC (`ML-KEM-768`/`ML-DSA-65`), `SecretBox<T>`, `0.5-ULP` `core_math` | `AG-P0-01` | `AG-P1-01` |
| 3 | `signal-geom` | Exact `i128` fixed-point & Shewchuk adaptive predicates, R*-tree spatial index, Euler curved waveguides | `AG-P0-01` | `AG-P1-02` |
| 4 | `signal-graph` | In-memory ECS multi-domain Design Graph, topological-invariant CRDT, selective causal undo/redo, CoW branches, UPF graph | `AG-P0-01` | `AG-P1-03`, `AG-P6-01` |
| 5 | `signal-component` | `ElectronicModule` trait (`q(v)`/`Φ(i)` ports, Jacobians, tolerances, TCR, parasitics, thermal self-heating, SOA watchdog, faults) | `AG-P0-01` | `AG-P1-04` |
| 6 | `signal-lib-passive` | Built-in Resistors, MLCCs (DC-bias/temp/freq derating), Inductors (core saturation), Transformers, fuses, crystals | `AG-P1-04` | `AG-P1-04` |
| 7 | `signal-lib-semiconductor` | Diodes, Zeners, BJTs (Gummel-Poon/VBIC), MOSFETs (BSIM4/BSIM-CMG/EKV), SiC/GaN HEMTs, Op-Amps, LDOs, Buck/Boost, Logic/ADC/DAC | `AG-P1-04` | `AG-P1-04` |
| 8 | `signal-module-sdk` | Multi-dialect SPICE subcircuit parser, AOT Cranelift Verilog-A compiler, child-MicroVM `OSDI`/`IBIS-AMI` IPC bridge, curve fitter | `AG-P1-04` | `AG-P1-04`, `AG-P2-06` |
| 9 | `signal-sim` | Ruiz-equilibrated `f64`/`f128` MNA solver, CPU `KLU`/`CKTSO` (`< 25k`) ↔ GPU `cuDSS` dispatcher, DC/AC/Noise/TR-BDF2/PSS/HB/Monte-Carlo | `AG-P0-01` | `AG-P1-05` |
| 10 | `signal-diff-sim` | Differentiable SPICE (`I-01`): Filippov Saltation Matrices at switching events + `Revolve` $O(\log N_t)$ Adjoint checkpointing + PSS warm-start | `AG-P0-01` | `AG-P1-06` |
| 11 | `signal-parts` | 3-Tier Universal Parts resolver, Master Pin Table bijection, IEEE/IEC Symbols, Parametric Padstacks, IPC-7351 Footprints, 5-Gate 3D STEP verifier (`18-*`) | `AG-P0-01` | `AG-P2-05`, `AG-P2-06` |
| 12 | `signal-git-sync` | `gitoxide`/`jj-lib` BYO-Git engine, Padmé-padded path-blinded `.signal/vault.merkle.sig` `AES-256-GCM-SIV`, Dual-Fenced 2PC `Super-Commit`, external push auto-rebase | `AG-P0-01` | `AG-P2-01` |
| 13 | `signal-egress-proxy` | Socket-pinned anti-SSRF + linearizable `X-Signal-Fencing-Token` epoch gate for all outbound Git/LFS/BYO-S3/ERP traffic | `AG-P0-01` | `AG-P2-02` |
| 14 | `signal-cell-wasm` | `< 8 MB` Wasmtime `EXPLORER` circuit/simulation cell with live `< 2 s` promotion to full `signal-pod` MicroVM | `AG-P0-01` | `AG-P2-03` |
| 15 | `signal-operator` | `kube-rs` K8s operator: Host-Aggregated Node Leases (`100x` fewer `etcd` writes), `< 150 ms` 3-Phase Pre-Copy Live Migration, multi-version LTS pod scheduler, `H100 CC`/`B200 TEE-I/O` MIG | `AG-P0-01` | `AG-P2-04` |
| 16 | `signal-formal` | Neuro-symbolic SAT/SMT/BDD + IC3/PDR formal property & equivalence prover (`I-03`) | `AG-P3-03` | `AG-P3-03` |
| 17 | `signal-surrogate` | GPU Physics-Informed Neural Operators (PINNs) for `< 50 ms` interactive field exploration backed by exact Rust sign-off (`I-02`) | `AG-P3-04` | `AG-P3-04` |
| 18 | `signal-import` | Zero-network sandboxed Clean-Room legacy importers (`Altium`, `KiCad`, `OrCAD`, `Eagle`, `LTspice`, `GDSII/OASIS`, `LEF/DEF`, bomb-capped `<= 20:1`) (`I-10`) | `AG-P3-05` | `AG-P3-05` |
| 19 | `signal-pcb` | Interactive PNS push-and-shove router, bus-trunk router, diff-pair meanders, fiber-weave, back-drill, copper pour, package delay, HDI, RF synthesizer, Fab Profiles, minimal-perturbation PCB ECO | `AG-P4-01` | `AG-P4-01`, `AG-P4-02` |
| 20 | `signal-em3d` | GPU 3D PEEC/FDTD/FEM full-wave EM solver, 224G PAM4 SerDes channel/COM/bathtub analyzer, causal passive Vector Fitting (`I-04`) | `AG-P4-02` | `AG-P4-02` |
| 21 | `signal-magnetics` | Custom transformer & planar magnetics designer (Steinmetz/iGSE core loss, Dowell proximity loss, 2D FEA leakage/fringing) | `AG-P4-03` | `AG-P4-03` |
| 22 | `signal-power-integrity` | `Z_target` PDN optimizer, 3D geodesic HV creepage/clearance, heavy-copper busbar IR-drop, hot-loop/Miller analyzer, Bode compensator, BMS pack, FOC motor, airflow/TIM thermal, power sequencing | `AG-P4-03` | `AG-P4-03` |
| 23 | `signal-emc` | Conducted/radiated CISPR/FCC pre-compliance, EMI filter + Middlebrook synthesizer, ESD/EFT/Surge tracer, shield resonance, acoustic piezo, ISO 26262 FMEDA, FTA, WCCA, Rad-Hard SEU/SET, derating | `AG-P4-04` | `AG-P4-04` |
| 24 | `signal-aging` | 10–15 yr mission-profile reliability & aging solver (NBTI/PBTI, HCI, TDDB, Black's Law electromigration, thermal solder fatigue) (`I-05`) | `AG-P4-04` | `AG-P4-04` |
| 25 | `signal-dfm` | PCB panelization + MLCC flex-crack guard, DFA reflow/tombstone/stencil, solder fatigue/vibration, conformal coating, live GD&T drawings, should-cost, ICT/flying-probe optimizer, MSL reflow | `AG-P4-05` | `AG-P4-05` |
| 26 | `signal-sftp-egress` | Ephemeral one-shot write-only `russh` SFTP pod (socket-pinned IP, host-key pinned, `.tmp.part` → remote SHA-256 read-back → atomic `rename` + `ML-DSA` `.sig`) | `AG-P0-01` | `AG-P4-07` |
| 27 | `signal-mcad` | Parametric B-Rep solid/surface kernel (`CAD_MCAD`), 2D geometric constraint solver, sheet-metal flat-pattern unfolder, live ECAD↔MCAD anchors, STEP AP242/IGES/JT/glTF/3MF (`19-*`) | `AG-P0-01` | `AG-P5-01` |
| 28 | `signal-harness` | Multi-board 3D assembly & collision solver, rigid-flex 3D kinematic folding & copper strain, 1:1 wire-harness nailboard/ampacity | `AG-P5-02` | `AG-P5-02` |
| 29 | `signal-cam` | Subtractive 2.5D/3-axis/3+2/5-axis adaptive trochoidal CNC toolpaths + gouge collision (`CAM_CNC`) AND direct-B-Rep additive 3D printing slicer + tree supports + warp compensation (`PRINT_3D`) (`19-*`) | `AG-P0-01` | `AG-P5-03`, `AG-P5-04` |
| 30 | `signal-plm` | Deterministic `eBOM` → `mBOM` → `sBOM` compiler, semantic BOM/geometry redline diff, Clear-to-Build (`CTB`) & should-cost evaluator (`19-*`) | `AG-P0-01` | `AG-P5-05` |
| 31 | `signal-ic`, `signal-security-hw`, `signal-fpga`, `signal-cosim`, `signal-twin`, `signal-pod` | Analog/Digital RTL-to-GDSII/OASIS (`gm/Id`, common-centroid, `.lib`, BSPDN, floorplan, CTS, STA, metal ECO, UCIe chiplets + `*.halo.gds.sig`, litho/CMP, Pelgrom/LVF, UPF `I-07`, DFT/ATPG `I-08`, IEEE 1735, Baseband CPO, 4K Cryo), HW Security (DPA/TVLA/IFT), FPGA Studio, Cycle-Accurate MCU/BIOS/UEFI Co-Sim, Lab Digital Twin (`I-06`), and main `signal-pod` gRPC/WebTransport server + Zero-Leak GPU encoder | `AG-P0-01` | `AG-P1-02`, `AG-P4-06`, `AG-P6-01`..`AG-P6-06`, `AG-P7-02`, `AG-P8-01` |

### 2.2 Tier 2 Control Plane (`services/*` Python `3.15+` FastAPI/Granian + Envoy Bridge + Huddle SFU) & AI (`ai/*`)

| # | Service / Component | Runtime | Owner Lane | Scaffolded In | Full Implementation Task(s) |
|---|---|---|---|---|---|
| 1 | `services/api-gateway` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P0-04`, `CU-P8-03` |
| 2 | `services/identity-org` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P2-01` |
| 3 | `services/project-catalog` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P2-03` |
| 4 | `services/parts-registry` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P2-04` |
| 5 | `services/plm-ops` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P5-01`, `CU-P5-02` |
| 6 | `services/collab-hub` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P7-01`..`CU-P7-03` |
| 7 | `services/vcs-connector` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P2-02` |
| 8 | `services/vcs-auditor` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P2-02` |
| 9 | `services/session-broker` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P2-03` |
| 10 | `services/sftp-egress` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-04` | `CU-P4-05` |
| 11 | `services/audit-service` | Python `3.15+` FastAPI + Granian | Lane B (Cursor) | `CU-P0-05` | `CU-P0-05`, `CU-P8-04` |
| 12 | `signal-engine-bridge` | Envoy Zero-Copy HTTP/3 WebTransport/gRPC/Arrow Relay (`pool-api`) | Lane A (Antigravity) | `AG-P0-04` | `AG-P0-04`, `AG-P2-04` |
| 13 | `signal-huddle-sfu` | Zero-Leak WebRTC Audio/Video + Viewport-Follow SFU (`pool-api`) | Lane A (Antigravity) | `AG-P7-01` | `AG-P7-01` |
| 14 | `ai/datasheet-ingest` | Python `3.15+` Dual-VLM + Multimodal Quarantine | Lane A (Antigravity) | `AG-P3-01` | `AG-P3-01` |
| 15 | `ai/rag-service` | Python `3.15+` 3-Tier `pgvector` + Graph + Mirrored BOM | Lane A (Antigravity) | `AG-P3-02` | `AG-P3-02` |
| 16 | `ai/llm-gateway` | Python `3.15+` GBNF Router (Ollama/MLX/`deterministic_mock`/Cloud) | Lane B (Cursor) | `CU-P3-01` | `CU-P3-01` |
| 17 | `ai/orchestrator` | Python `3.15+` Neuro-Symbolic Planner + Merge-Time Proof Re-Verifier | Lane B (Cursor) | `CU-P3-02` | `CU-P3-02` |

### 2.3 Angular `v22+` ONLY Application & `23` Lazy Libraries (`web/**` — Cursor Lane B)

| # | Angular App / Library (`web/*`) | Role | Scaffolded In | Full Implementation Task(s) |
|---|---|---|---|---|
| 0 | `web/apps/signal-web` | Zoneless Signals IDE Shell, Route `COOP`/`COEP`, Omni-Palette `Cmd+K`, Multi-Monitor, SpaceMouse, Progressive Profiles, What's New Drawer | `CU-P0-03` | `CU-P0-03`, `CU-P1-05`, `CU-P3-05`, `CU-P8-03` |
| 1 | `web/libs/design-system` | Centralized `--sig-*` tokens, Dark/High-Contrast/Okabe-Ito themes, `.sig-*` CSS primitives (NO component-local CSS rule) | `CU-P0-01` | `CU-P0-01` |
| 2 | `web/libs/ui-kit` | Standardized `<sig-*>` shared HTML component ecosystem (`sig-button`, `sig-table`, `sig-drawer`, `sig-split-pane`, `sig-si-input`, etc.) | `CU-P0-02` | `CU-P0-02` |
| 3 | `web/libs/canvas-core` | Dual-Plane `Zstd-QOI`/WebP + Hardware `HEVC 4:4:4`/Dual-Luma receiver, `HudOverlayPrimitive` 120 FPS renderer, DLP watermarks/guards | `CU-P0-03` | `CU-P1-01`, `CU-P4-06` |
| 4 | `web/libs/schematic` | Smart Schematic & Symbol Workbench (`FEAT-001`..`FEAT-010`, `FEAT-101`) | `CU-P0-03` | `CU-P1-02`, `CU-P3-03` |
| 5 | `web/libs/sim-viewer` | Arrow-streamed Waveform Viewer, Virtual Bench Instruments, Formula DSL, Monte Carlo, Jitter/Bathtub, What-If, Protocol Track, Pareto (`FEAT-053`, `FEAT-081`..`FEAT-086`, `FEAT-089`, `FEAT-090`, `FEAT-102`, `FEAT-103`, `FEAT-106`) | `CU-P0-03` | `CU-P1-03`, `CU-P1-04`, `CU-P3-03`, `CU-P3-04`, `CU-P6-05` |
| 6 | `web/libs/pcb` | High-Speed PCB Layout, PNS Router, Constraint Matrix, Fiber-Weave, Diff-Pair Meanders, Back-Drill, RF Wizard, 3D EM Field Animator, TDR Cross-Probe, X-Ray (`FEAT-011`..`FEAT-019`, `FEAT-087`, `FEAT-089`, `FEAT-095`, `FEAT-096`) | `CU-P0-03` | `CU-P4-01` |
| 7 | `web/libs/magnetics` | Custom Transformer & Planar Magnetics Winding Studio (`FEAT-021`) | `CU-P0-03` | `CU-P4-02` |
| 8 | `web/libs/power-studio` | PDN `Z_target`, 3D HV Geodesic Creepage, Busbar IR-Drop, Hot-Loop/Miller, Bode Compensator, BMS Pack, FOC Motor, Airflow Thermal, Power Sequencing Gantt (`FEAT-022`..`FEAT-030`) | `CU-P0-03` | `CU-P4-02` |
| 9 | `web/libs/emc-lab` | CISPR/FCC Spectrum Studio, EMI Filter Designer, ESD/EFT/Surge Injector, Shield Aperture Heatmap, Acoustic Piezo Risk (`FEAT-031`..`FEAT-034`, `FEAT-039`) | `CU-P0-03` | `CU-P4-03` |
| 10 | `web/libs/safety-wcca` | ISO 26262 FMEDA Workbench, Fault-Tree (FTA) Canvas, WCCA Report Studio, Rad-Hard SEU/SET Injector, Derating Policy Dashboard (`FEAT-035`..`FEAT-038`, `FEAT-040`) | `CU-P0-03` | `CU-P4-03` |
| 11 | `web/libs/ic` | Analog/Digital IC & Chiplet Workbench (`gm/Id`, Common-Centroid, Liberty `.lib`, BSPDN Dual-Side, SoC Floorplan, CTS, STA Waterfall, UCIe Interposer, Litho/CMP, HW Security `FEAT-041`..`FEAT-050`) | `CU-P0-03` | `CU-P6-01` |
| 12 | `web/libs/fpga-studio` | FPGA Die Floorplan, SDC/XDC Constraint Matrix, Graphical FSM Editor, Fixed-Point DSP Filter Studio (`FEAT-051`, `FEAT-052`, `FEAT-057`, `FEAT-060`) | `CU-P0-03` | `CU-P6-02` |
| 13 | `web/libs/code-ide` | Monaco/LSP Firmware & BIOS/UEFI IDE, Memory-Map Explorer, SAT Pinmux Matrix, Battery Energy Ledger, RTL IFT Security (`FEAT-050`, `FEAT-054`..`FEAT-056`) | `CU-P0-03` | `CU-P6-03` |
| 14 | `web/libs/bringup-lab` | Lab Digital Twin Correlation, Cold-Boot/POST Trace, Virtual HIL Fault Bench, Factory Interactive Board-Map, 2D/3D Shmoo Plot Studio (`FEAT-058`, `FEAT-059`, `FEAT-070`, `FEAT-088`, `I-06`) | `CU-P0-03` | `CU-P6-04` |
| 15 | `web/libs/harness-3d` | Multi-Board 3D Assembly, Rigid-Flex 3D Kinematic Folding Slider, 1:1 Wire-Harness Nailboard & Ampacity Studio (`FEAT-020`, `FEAT-061`, `FEAT-062`) | `CU-P0-03` | `CU-P5-03` |
| 16 | `web/libs/dfm-panel` | Panel Composer, DFA Checklist, Solder Fatigue/Vibration, Conformal Coating Mask, Live GD&T Drawing Sheets, Should-Cost Pill, Testability Heatmap, MSL Reflow (`FEAT-063`..`FEAT-069`, `FEAT-080`) | `CU-P0-03` | `CU-P4-04` |
| 17 | `web/libs/library-studio` | Universal Parts Studio (`18-*`): IPC-7351 Footprint & 3D STEP Wizard, Multi-Unit Symbol Splitter, SPICE Curve-Fitter, Rust/WASM Module SDK IDE, Padstack Cross-Section Editor, Golden Snippets, Datasheet-to-Testbench (`FEAT-071`..`FEAT-074`, `FEAT-076`, `FEAT-079`, `FEAT-107`) | `CU-P0-03` | `CU-P2-05`, `CU-P3-05` |
| 18 | `web/libs/specs` | Executable PRD/HFS Block Editor, Mirrored Multi-Distributor BOM Matrix, Provenance Lineage Badges, Datasheet Revision Diff Drawer, AI BOM Feeder Consolidator (`FEAT-077`, `FEAT-078`, `FEAT-108`) | `CU-P0-03` | `CU-P2-04`, `CU-P3-05` |
| 19 | `web/libs/ai-copilot` | Neuro-Symbolic AI Agent Drawer, Speculative Diff Preview, Formal Proof Certificates, Multimodal Whiteboard/Datasheet Upload | `CU-P0-03` | `CU-P3-03`..`CU-P3-05` |
| 20 | `web/libs/visual-diff` | Multi-Domain Onion-Skin Visual Semantic Diff (Schematic/PCB/3D/Waveform/BOM) & Git/Jujutsu History Time-Scrubber (`FEAT-097`) | `CU-P0-03` | `CU-P2-06` |
| 21 | `web/libs/review` | Spatial Canvas Redlining, Live Follow-Me Laser Pointer, EVT/DVT/PVT Stage-Gate Checklist, 200-Point Red-Team Report, Live Review Deck, External Partner Guest Portal (`FEAT-094`, `FEAT-098`, `FEAT-104`, `FEAT-109`, `FEAT-119`) | `CU-P0-03` | `CU-P2-06`, `CU-P3-04`, `CU-P3-05` |
| 22 | `web/libs/sftp-release` | Step-Up 2FA (WebAuthn/TOTP) SFTP Release Wizard, Atomic `.tmp.part` Progress & `ML-DSA` Signed Manifest Viewer (`07-*`) | `CU-P0-03` | `CU-P4-05` |
| 23 | `web/libs/academy` | Breadboard ↔ Schematic ↔ PCB Twin, Guided Labs & LTI Auto-Grader, Visual Smoke Sandbox, Resistor/SMD Quick-Calc, Open-Silicon Shuttle Wizard (`FEAT-111`..`FEAT-115`) | `CU-P0-03` | `CU-P1-06`, `CU-P2-06`, `CU-P8-01` |
| 24 | `web/libs/admin` | Org/Project Admin Access Review, AVL Where-Used Console, Compute FinOps & Budget Caps, Policy-as-Code Studio, Portfolio Blast-Radius Graph (`FEAT-075`, `FEAT-116`..`FEAT-118`) | `CU-P0-03` | `CU-P2-06`, `CU-P8-02` |
| 25 | `web/libs/mcad-studio` | Parametric B-Rep 3D Solid/Surface & Sheet-Metal CAD Workbench, Live ECAD↔MCAD Anchor Inspector (`19-*`) | `CU-P0-03` | `CU-P5-03` |
| 26 | `web/libs/cam-print` | Subtractive 2.5D–5-Axis CNC Toolpath Simulator & Additive Direct-B-Rep 3D Printing Build-Plate Slicer Studio (`19-*`) | `CU-P0-03` | `CU-P5-04` |
| 27 | `web/libs/plm-ops` | Multi-BOM (`eBOM`/`mBOM`/`sBOM`), ECN/ECO/MCN/PCN/Deviation/NCR/CAPA/CTB Control Tower & 3D Interactive Assembly Work Instructions (`19-*`) | `CU-P0-03` | `CU-P5-05` |
| 28 | `web/libs/collab-hub` | Zero-Leak 5-Team Collaboration Space, `@entity` Threads, WebRTC Huddles, Live Project Status Control Tower & Automated Weekly/Monthly/Stage-Gate Review Studio (`21-*`) | `CU-P0-03` | `CU-P7-04` |

*(Note: `design-system` and `ui-kit` are foundational UI platform libraries; `canvas-core` + the 23 domain workbenches above make up the complete modular `web/libs/*` suite.)*

---

## 3. All `120` Extended Dual UI + Rust Features (`FEAT-001`–`FEAT-120`) → Exact Task Assignment

| Feature ID | Feature Name | Phase | Contract Task | Antigravity Rust Task (`engine/`) | Cursor UI/API Task (`web/`, `services/`) |
|---|---|---|---|---|---|
| **`FEAT-001`** | Live Schematic Auto-Beautifier & Orthogonal Wire Untangler | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::beautify`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-002`** | Semantic Bus Slice, Struct-Bundle & Protocol-Aware Fanout Harness | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::bus_bundle`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-003`** | Interactive On-Canvas Potentiometer, Switch & Fault-Toggle Controls | `Phase 1` | `C-P1-01` | `AG-P1-05` (`signal-sim::interactive_step`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-004`** | Animated Schematic Power-Rail & Current-River Heatmap Overlay | `Phase 1` | `C-P1-01` | `AG-P1-05` (`signal-sim::branch_telemetry`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-005`** | Cross-Sheet Net-Jump Portal Minimap & Signal-Path Breadcrumb Tracer | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::hierarchy_index`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-006`** | Multi-Channel Block Parameterization & Live Matrix Instantiation (`Repeat(N)`) | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::multi_channel`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-007`** | Schematic Variant Diff Overlay & Interactive Stuffing-Option Curtain | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::variants`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-008`** | Interactive Pin-Swapping & Gate-Swapping Negotiator | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::swap_equivalence`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-009`** | Geographical Auto-Numbering Re-Annotator & Rev-History RefDes Map | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::reannotate`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-010`** | Live Math-Linked Parametric Equation Blocks on Canvas | `Phase 1` | `C-P1-01` | `AG-P1-01` (`signal-core::expr_solver`) | `CU-P1-02` (`libs/schematic`) |
| **`FEAT-011`** | Fiber-Weave Skew Mitigation & Zig-Zag / Panel-Rotation Planner | `Phase 4` | `C-P4-01` | `AG-P4-01` (`signal-pcb::fiber_weave`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-012`** | Interactive Diff-Pair Phase-Tuning & Coupled-Arc Meander Composer | `Phase 4` | `C-P4-01` | `AG-P4-01` (`signal-pcb::meander_tuner`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-013`** | Automated Back-Drill Stub Optimizer & Via S-Parameter Tuner | `Phase 4` | `C-P4-01` | `AG-P4-02` (`signal-em3d::via_stub`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-014`** | Copper-Pour Island Detector, Acid-Trap Eliminator & Teardrop Sculptor | `Phase 4` | `C-P4-01` | `AG-P4-01` (`signal-pcb::copper_hygiene`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-015`** | Die-to-Ball Package Flight-Time (`ps`) Pin-Delay Compensator | `Phase 4` | `C-P4-01` | `AG-P4-01` (`signal-pcb::pkg_flight_time`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-016`** | Return-Path Reference-Plane Split & Ground-Slot Crossing Guardian | `Phase 4` | `C-P4-01` | `AG-P4-02` (`signal-em3d::return_path`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-017`** | Multi-Trace Bundled Bus-Trunk Push-and-Shove Router | `Phase 4` | `C-P4-01` | `AG-P4-01` (`signal-pcb::bus_pns`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-018`** | HDI Microvia Stack Reliability & Aspect-Ratio Live Enforcer | `Phase 4` | `C-P4-01` | `AG-P4-01` (`signal-pcb::hdi_stackup`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-019`** | Planar RF Microstrip, Coupler, Hairpin Filter & Patch Antenna Synthesizer | `Phase 4` | `C-P4-01` | `AG-P4-02` (`signal-em3d::rf_synth`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-020`** | Rigid-Flex 3D Bend-Zone Kinematic Folding Simulator & Copper-Strain Verifier | `Phase 5` | `C-P5-01` | `AG-P5-02` (`signal-harness::flex_kinematics`) | `CU-P5-03` (`libs/harness-3d`) |
| **`FEAT-021`** | Custom Transformer & Planar Magnetics Winding Designer | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-magnetics`) | `CU-P4-02` (`libs/magnetics`) |
| **`FEAT-022`** | Interactive PDN Target-Impedance (`Z_target(f)`) Decoupling Optimizer | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::pdn`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-023`** | 3D Surface Creepage & Clearance High-Voltage Geodesic Verifier | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::creepage_3d`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-024`** | Heavy-Copper Busbar IR-Drop, Necking & Via Current-Density Mapper | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::dc_ir`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-025`** | Switching Hot-Loop Inductance Minimizer & SiC/GaN Miller-Turn-On Analyzer | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::hot_loop`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-026`** | Interactive Control-Loop Bode / Nyquist & Analog/Digital Compensator Synthesizer | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::loop_comp`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-027`** | Battery Pack & BMS Cell-Balancing, Aging & Thermal-Runaway Co-Simulator | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::battery_bms`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-028`** | Motor Drive, FOC Control & SVPWM Electro-Mechanical Co-Simulator | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::motor_drive`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-029`** | Heatsink, Forced-Airflow (CFM), TIM & Cold-Plate Thermal Network Studio | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::thermal_cfd`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-030`** | Power-Sequencing Gantt, Inrush, Hot-Swap & Back-Powering Stress Verifier | `Phase 4` | `C-P4-01` | `AG-P4-03` (`signal-power-integrity::sequencing`) | `CU-P4-02` (`libs/power-studio`) |
| **`FEAT-031`** | Conducted & Radiated Pre-Compliance Simulator (`CISPR 11/25/32`, `FCC`, `MIL-STD-461`) | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::precompliance`) | `CU-P4-03` (`libs/emc-lab`) |
| **`FEAT-032`** | EMI Input Filter (CM/DM Choke + X/Y-Cap) & Middlebrook Stability Synthesizer | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::filter_synth`) | `CU-P4-03` (`libs/emc-lab`) |
| **`FEAT-033`** | ESD (`IEC 61000-4-2`), EFT (`-4-4`) & Surge/Lightning (`-4-5`, `DO-160`) Clamping Tracer | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::transient_immunity`) | `CU-P4-03` (`libs/emc-lab`) |
| **`FEAT-034`** | Shield-Can, Gasket Aperture & Chassis Grounding Resonance Analyzer | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::shield_cavity`) | `CU-P4-03` (`libs/emc-lab`) |
| **`FEAT-035`** | Automated ISO 26262 / IEC 61508 FMEDA & `SPFM`/`LFM`/`PMHF` Workbench | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::fmeda`) | `CU-P4-03` (`libs/safety-wcca`) |
| **`FEAT-036`** | Interactive Fault-Tree Analysis (FTA) & Cut-Set Traceability Diagrammer | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::fta_bdd`) | `CU-P4-03` (`libs/safety-wcca`) |
| **`FEAT-037`** | Automated Worst-Case Circuit Analysis (WCCA / EVA / RSS) Engine | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::wcca`) | `CU-P4-03` (`libs/safety-wcca`) |
| **`FEAT-038`** | Rad-Hard Single-Event Upset (SEU) & Transient (SET) Heavy-Ion Injector | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::rad_hard`) | `CU-P4-03` (`libs/safety-wcca`) |
| **`FEAT-039`** | Acoustic Noise (MLCC Piezo-Singing & Magnetostriction) Risk Flagging | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::piezo_acoustic`) | `CU-P4-03` (`libs/emc-lab`) |
| **`FEAT-040`** | Org-Configurable Component Stress & Derating Policy Dashboard | `Phase 4` | `C-P4-01` | `AG-P4-04` (`signal-emc::derating_rules`) | `CU-P4-03` (`libs/safety-wcca`) |
| **`FEAT-041`** | Interactive `gm/Id` Lookup-Table & Analog Design-Space Explorer | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::gmid_lut`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-042`** | Automated Common-Centroid, Interdigitated & Symmetric Analog Layout Generator | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::analog_placer`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-043`** | Custom Standard-Cell & SRAM Bitcell Layout-to-Liberty (`.lib`) Characterizer | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::lib_char`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-044`** | Backside Power Delivery (BSPDN / PowerVia) & Nano-TSV Co-Design Workbench | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::bspdn`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-045`** | Interactive SoC Floorplan, NoC Topology & Macro-Channel Congestion Sculptor | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::floorplan_noc`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-046`** | Clock-Tree Synthesis (CTS) Interactive Skew, H-Tree & Clock-Mesh Studio | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::cts_mesh`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-047`** | Interactive STA Critical-Path Waterfall & Instant ECO Fix Previewer | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::sta_eco`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-048`** | 2.5D/3D-IC Chiplet Bump-Map, Micro-Bump Escape & UCIe Lane Negotiator | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::chiplet_ucie`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-049`** | Lithography / OPC Aerial-Image & CMP Metal-Density Hotspot Visualizer | `Phase 6` | `C-P6-01` | `AG-P6-01` (`signal-ic::litho_cmp`) | `CU-P6-01` (`libs/ic`) |
| **`FEAT-050`** | Hardware Security Auditor: DPA/TVLA Side-Channel, Fault-Injection & RTL IFT Trojan Verifier | `Phase 6` | `C-P6-01` | `AG-P6-03` (`signal-security-hw`) | `CU-P6-01` & `CU-P6-03` |
| **`FEAT-051`** | Interactive FPGA Die Floorplan, I/O Bank-Voltage & SerDes Transceiver Planner | `Phase 6` | `C-P6-02` | `AG-P6-04` (`signal-fpga::io_planner`) | `CU-P6-02` (`libs/fpga-studio`) |
| **`FEAT-052`** | Interactive SDC/XDC Timing-Constraint Wizard & Formal Waiver Verifier | `Phase 6` | `C-P6-02` | `AG-P6-04` (`signal-fpga::sdc_verifier`) | `CU-P6-02` (`libs/fpga-studio`) |
| **`FEAT-053`** | Live Bus-Protocol Transaction & Waveform Decoder (`AXI4`, `SPI`, `I2C`, `CAN-FD`, `USB`, `PCIe`) | `Phase 6` | `C-P6-02` | `AG-P6-05` (`signal-cosim::protocol_decode`) | `CU-P6-05` (`libs/sim-viewer`) |
| **`FEAT-054`** | Visual Interrupt-Vector, DMA Channel & Memory-Map Address-Collision Explorer | `Phase 6` | `C-P6-02` | `AG-P6-05` (`signal-cosim::memmap_svd`) | `CU-P6-03` (`libs/code-ide`) |
| **`FEAT-055`** | Interactive SAT-Solved MCU Pinmux Conflict Resolver & Alternate-Function Matrix | `Phase 6` | `C-P6-02` | `AG-P6-05` (`signal-cosim::pinmux_sat`) | `CU-P6-03` (`libs/code-ide`) |
| **`FEAT-056`** | Firmware Power-State & Battery-Life Energy Profiler | `Phase 6` | `C-P6-02` | `AG-P6-05` (`signal-cosim::energy_profiler`) | `CU-P6-03` (`libs/code-ide`) |
| **`FEAT-057`** | Graphical FSM & Waveform Timing-Diagram Bidirectional Code Generator | `Phase 6` | `C-P6-02` | `AG-P6-04` (`signal-fpga::fsm_codegen`) | `CU-P6-02` (`libs/fpga-studio`) |
| **`FEAT-058`** | Automated Cold-Boot, UEFI/ACPI/Device-Tree & POST Sequence Verifier | `Phase 6` | `C-P6-02` | `AG-P6-05` (`signal-cosim::boot_verifier`) | `CU-P6-04` (`libs/bringup-lab`) |
| **`FEAT-059`** | Live Virtual HIL Fault-Recovery & Watchdog Firmware Stress Bench | `Phase 6` | `C-P6-02` | `AG-P6-05` (`signal-cosim::hil_watchdog`) | `CU-P6-04` (`libs/bringup-lab`) |
| **`FEAT-060`** | Fixed-Point DSP & Digital Filter Co-Designer (FIR/IIR/CIC/FFT → RTL & C/Rust) | `Phase 6` | `C-P6-02` | `AG-P6-04` (`signal-fpga::dsp_fixedpoint`) | `CU-P6-02` (`libs/fpga-studio`) |
| **`FEAT-061`** | Multi-Board 3D Assembly & Enclosure Clearance / Interference Studio | `Phase 5` | `C-P5-01` | `AG-P5-02` (`signal-harness::assembly_3d`) | `CU-P5-03` (`libs/harness-3d`) |
| **`FEAT-062`** | Wire-Harness 1:1 Nailboard (Formboard), Splice & Ampacity Designer | `Phase 5` | `C-P5-01` | `AG-P5-02` (`signal-harness::nailboard`) | `CU-P5-03` (`libs/harness-3d`) |
| **`FEAT-063`** | Automated PCB Panelization, V-Score, Mouse-Bite & MLCC Flex-Crack Guard | `Phase 4` | `C-P4-02` | `AG-P4-05` (`signal-dfm::panelizer`) | `CU-P4-04` (`libs/dfm-panel`) |
| **`FEAT-064`** | Automated DFA (Reflow Shadowing, Wave-Solder, Tombstone & Stencil Ratio) | `Phase 4` | `C-P4-02` | `AG-P4-05` (`signal-dfm::dfa_solder`) | `CU-P4-04` (`libs/dfm-panel`) |
| **`FEAT-065`** | Solder-Joint Thermo-Mechanical Fatigue & Drop-Shock Vibration Predictor | `Phase 4` | `C-P4-02` | `AG-P4-05` (`signal-dfm::mechanics_fatigue`) | `CU-P4-04` (`libs/dfm-panel`) |
| **`FEAT-066`** | Selective-Solder, Conformal-Coating & Potting Keepout Mask Editor | `Phase 4` | `C-P4-02` | `AG-P4-05` (`signal-dfm::coating_keepout`) | `CU-P4-04` (`libs/dfm-panel`) |
| **`FEAT-067`** | Dynamic Fabrication & Assembly Drawing Studio (Live GD&T & Drill Charts) | `Phase 4` | `C-P4-02` | `AG-P4-05` (`signal-dfm::drawing_gen`) | `CU-P4-04` (`libs/dfm-panel`) |
| **`FEAT-068`** | Real-Time DFM Should-Cost & First-Pass Yield Estimator HUD | `Phase 4` | `C-P4-02` | `AG-P4-05` (`signal-dfm::should_cost`) | `CU-P4-04` (`libs/dfm-panel`) |
| **`FEAT-069`** | Interactive Test-Point Optimizer & ICT / Flying-Probe Coverage Heatmap | `Phase 4` | `C-P4-02` | `AG-P4-05` (`signal-dfm::testpoint_opt`) | `CU-P4-04` (`libs/dfm-panel`) |
| **`FEAT-070`** | Factory Bring-Up Interactive Board-Map & Guided Debug Overlay | `Phase 6` | `C-P6-02` | `AG-P6-06` (`signal-twin::board_debug`) | `CU-P6-04` (`libs/bringup-lab`) |
| **`FEAT-071`** | Visual IPC-7351 / JEDEC Parametric Footprint & Watertight 3D STEP Generator | `Phase 2` | `C-P2-02` | `AG-P2-05` (`signal-parts::ipc7351_step`) | `CU-P2-05` (`libs/library-studio`) |
| **`FEAT-072`** | Automated Multi-Unit Symbol Splitter & Pin-Grouping Generator | `Phase 2` | `C-P2-02` | `AG-P2-05` (`signal-parts::symbol_splitter`) | `CU-P2-05` (`libs/library-studio`) |
| **`FEAT-073`** | Interactive SPICE / `ElectronicModule` Curve-Fitter & Parameter Extractor | `Phase 2` | `C-P2-02` | `AG-P2-06` (`signal-module-sdk::curve_fitter`) | `CU-P2-05` (`libs/library-studio`) |
| **`FEAT-074`** | Visual Rust/WASM Custom Component Sandbox Editor & Live Test-Rig | `Phase 2` | `C-P2-02` | `AG-P2-06` (`signal-module-sdk::wasm_sandbox`) | `CU-P2-05` (`libs/library-studio`) |
| **`FEAT-075`** | Enterprise AVL Governance & Cross-Project "Where-Used" Impact Analyzer | `Phase 2` | `C-P2-02` | `AG-P2-05` (`signal-parts`) | `CU-P2-04` & `CU-P2-06` (`libs/admin`) |
| **`FEAT-076`** | Interactive Padstack Designer & Custom Thermal-Relief / Anti-Pad Sculptor | `Phase 2` | `C-P2-02` | `AG-P2-05` (`signal-parts::padstack`) | `CU-P2-05` (`libs/library-studio`) |
| **`FEAT-077`** | Counterfeit / Gray-Market Risk & Authorized-Distributor Lineage Verifier | `Phase 2` | `C-P2-02` | `AG-P2-05` (`signal-parts::lineage`) | `CU-P2-04` (`libs/specs`) |
| **`FEAT-078`** | Automated Datasheet Revision Diff Tracker (Rev A → Rev B Errata Alerter) | `Phase 3` | `C-P3-01` | `AG-P3-01` (`ai/datasheet-ingest::revision_diff`) | `CU-P3-05` (`libs/specs`) |
| **`FEAT-079`** | Design-Reuse Verified Subcircuit & Pre-Routed Layout Block Vault ("Golden Snippets") | `Phase 2` | `C-P2-02` | `AG-P2-06` (`signal-graph::snippet_stamp`) | `CU-P2-05` (`libs/library-studio`) |
| **`FEAT-080`** | Package Thermal/Reflow Profile & JEDEC MSL Floor-Life Compliance Checker | `Phase 4` | `C-P4-02` | `AG-P4-05` (`signal-dfm::msl_reflow`) | `CU-P4-04` (`libs/dfm-panel`) |
| **`FEAT-081`** | Virtual Bench Instruments (Oscilloscope, Logic Analyzer, Spectrum & VNA Smith Chart) | `Phase 1` | `C-P1-02` | `AG-P1-05` (`signal-sim::virtual_instruments`) | `CU-P1-03` (`libs/sim-viewer`) |
| **`FEAT-082`** | GPU-Accelerated Multi-Trace Waveform Expression Engine & Calculator | `Phase 1` | `C-P1-02` | `AG-P1-05` (`signal-sim::wave_expr`) | `CU-P1-03` (`libs/sim-viewer`) |
| **`FEAT-083`** | Monte-Carlo Scatter, Tornado Sensitivity & Yield-Corner Pareto Explorer | `Phase 1` | `C-P1-02` | `AG-P1-05` (`signal-sim::yield_analytics`) | `CU-P1-04` (`libs/sim-viewer`) |
| **`FEAT-084`** | Phase-Noise, Jitter Decomposition (`RJ`/`DJ`/`PJ`/`DDJ`) & PAM4 Bathtub BER Studio | `Phase 1` | `C-P1-02` | `AG-P1-05` (`signal-sim::jitter_ber`) | `CU-P1-04` (`libs/sim-viewer`) |
| **`FEAT-085`** | Live Parameter-Sweep "What-If Scrubber" Sliders with Ghost Curve Overlay | `Phase 1` | `C-P1-02` | `AG-P1-06` (`signal-diff-sim::fast_sweep`) | `CU-P1-04` (`libs/sim-viewer`) |
| **`FEAT-086`** | Hardware Regression Test-Suite Runner & Golden-Envelope Comparator | `Phase 1` | `C-P1-02` | `AG-P1-05` (`signal-sim::regression_runner`) | `CU-P1-04` (`libs/sim-viewer`) |
| **`FEAT-087`** | 3D Electromagnetic Field-Line, Surface-Current & Far-Field Antenna Lobe Animator | `Phase 4` | `C-P4-01` | `AG-P4-02` (`signal-em3d::field_stream`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-088`** | Automated 2D/3D Shmoo-Plot Margin Mapper (Simulated PVT vs. Lab ATE) | `Phase 6` | `C-P6-02` | `AG-P6-06` (`signal-twin::shmoo_runner`) | `CU-P6-04` (`libs/bringup-lab`) |
| **`FEAT-089`** | Interactive TDR/TDT Impedance-Profile-to-Physical-Trace Distance Mapper | `Phase 4` | `C-P4-01` | `AG-P4-02` (`signal-em3d::tdr_peeling`) | `CU-P4-01` (`libs/pcb` & `libs/sim-viewer`) |
| **`FEAT-090`** | Real-World Signal Stimulus & Zero-Download WebAudio Audition Bridge | `Phase 1` | `C-P1-02` | `AG-P1-05` (`signal-sim::stimulus_gen`) | `CU-P1-04` (`libs/sim-viewer`) |
| **`FEAT-091`** | Omni-Command Palette (`Cmd+K`) with Deterministic CLI Action Grammar | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::command_dsl`) | `CU-P1-05` (`apps/signal-web`) |
| **`FEAT-092`** | Multi-Monitor Pop-Out Synchronized Workspaces (Zero-Leak Compliant) | `Phase 1` | `C-P1-01` | `AG-P1-02` (`signal-pod::viewport_mux`) | `CU-P1-05` (`apps/signal-web`) |
| **`FEAT-093`** | Context-Aware Radial Pie-Menu & 6-DOF SpaceMouse (`WebHID`) Navigation | `Phase 1` | `C-P1-01` | `AG-P1-02` (`signal-geom::camera_6dof`) | `CU-P1-05` (`apps/signal-web`) |
| **`FEAT-094`** | Live "Follow Mode" Spotlight, Ephemeral Laser Pointer & Guided PR Tour | `Phase 2` | `C-P2-01` | `AG-P2-03` (`signal-graph::ephemeral_presence`) | `CU-P2-06` (`libs/review`) |
| **`FEAT-095`** | Interactive Constraint-Manager Spreadsheet with Live Canvas Highlighting | `Phase 4` | `C-P4-01` | `AG-P4-01` (`signal-graph::constraint_matrix`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-096`** | Viewport Bookmarks, Layer-Stack Presets & Single-Key "X-Ray Net Isolation" | `Phase 4` | `C-P4-01` | `AG-P4-06` (`signal-pod::render_mask`) | `CU-P4-01` (`libs/pcb`) |
| **`FEAT-097`** | Visual Git/Jujutsu History Time-Scrubber & Animated Design-Evolution Replay | `Phase 2` | `C-P2-01` | `AG-P2-01` (`signal-git-sync::history_scrubber`) | `CU-P2-06` (`libs/visual-diff`) |
| **`FEAT-098`** | Automated Hardware Stage-Gate Checklist (`EVT`/`DVT`/`PVT`/`Tape-Out`) | `Phase 2` | `C-P2-01` | `AG-P2-01` (`signal-pod::stage_gate_evaluator`) | `CU-P2-06` (`libs/review`) |
| **`FEAT-099`** | Smart Selection Filter Bar & Topological Bulk-Property Query Editor | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::query_engine`) | `CU-P1-05` (`apps/signal-web`) |
| **`FEAT-100`** | WCAG 2.2 AA Keyboard-Only CAD Wiring, Screen-Reader Netlist & One-Hand Mode | `Phase 1` | `C-P1-01` | `AG-P1-03` (`signal-graph::a11y_tree`) | `CU-P1-05` (`apps/signal-web`) |
| **`FEAT-101`** | AI "Explain This Circuit" Functional Topology Decomposer & Annotator | `Phase 3` | `C-P3-01` | `AG-P3-03` (`signal-graph::subgraph_matcher`) | `CU-P3-03` (`libs/schematic`) |
| **`FEAT-102`** | AI Counterfactual "What Breaks First?" Operating-Envelope Stress Explorer | `Phase 3` | `C-P3-01` | `AG-P1-06` & `AG-P3-03` (`signal-diff-sim::stress_boundary`) | `CU-P3-03` (`libs/sim-viewer`) |
| **`FEAT-103`** | AI Interactive Multi-Objective Pareto Frontier Explorer (Cost vs. Eff vs. Area vs. $\text{kgCO}_2\text{e}$) | `Phase 3` | `C-P3-01` | `AG-P1-06` & `AG-P3-04` (`signal-diff-sim::pareto_nsga3`) | `CU-P3-04` (`libs/sim-viewer`) |
| **`FEAT-104`** | AI Senior Principal "Pre-Fab / Pre-Tapeout Red-Team" 200-Point Heuristic Auditor | `Phase 3` | `C-P3-01` | `AG-P3-03` (`signal-formal::redteam_deck`) | `CU-P3-04` (`libs/review`) |
| **`FEAT-105`** | AI Natural-Language Constraint & Rule-Deck Compiler (NL → DRC / UPF / SDC / SVA) | `Phase 3` | `C-P3-01` | `AG-P3-03` (`signal-formal::constraint_ast`) | `CU-P3-04` (`apps/signal-web`) |
| **`FEAT-106`** | AI Waveform Anomaly Root-Cause Debugger (Oscillation / Ringing → Layout Fix) | `Phase 3` | `C-P3-01` | `AG-P3-03` (`signal-sim::anomaly_attribution`) | `CU-P3-04` (`libs/sim-viewer`) |
| **`FEAT-107`** | AI Automated Datasheet-to-Testbench & Typical-Application Reconstructor | `Phase 3` | `C-P3-01` | `AG-P3-01` (`ai/datasheet-ingest::ref_circuit`) | `CU-P3-05` (`libs/library-studio`) |
| **`FEAT-108`** | AI Smart BOM-Line-Item Consolidator & SMT Feeder-Slot Minimizer | `Phase 3` | `C-P3-01` | `AG-P3-02` (`ai/rag-service::bom_consolidator`) | `CU-P3-05` (`libs/specs`) |
| **`FEAT-109`** | In-App Live Interactive Design-Review Deck & Compliance Briefing Composer | `Phase 3` & `7` | `C-P3-01`, `C-P7-01` | `AG-P7-02` (`signal-pod::review_snapshot`) | `CU-P3-05` & `CU-P7-04` |
| **`FEAT-110`** | Quarantined Multi-Language Spec, Review & Datasheet Translator | `Phase 3` | `C-P3-01` | `AG-P3-01` (`ai/datasheet-ingest`) | `CU-P3-05` (`apps/signal-web`) |
| **`FEAT-111`** | Interactive Breadboard ↔ Schematic ↔ PCB Twin Visualizer | `Phase 2` | `C-P2-01` | `AG-P2-03` (`signal-graph::breadboard_map`) | `CU-P2-06` (`libs/academy`) |
| **`FEAT-112`** | Guided Interactive Circuit Labs, Auto-Grading Sandbox & LMS Bridge | `Phase 8` | `C-P8-01` | `AG-P8-02` (`signal-sim::lab_autograder`) | `CU-P8-01` (`libs/academy`) |
| **`FEAT-113`** | Gamified Safe "Learn-by-Breaking" Visual Smoke & Over-Current Sandbox | `Phase 1` | `C-P1-02` | `AG-P1-04` (`signal-component::soa_failure_event`) | `CU-P1-06` (`libs/academy`) |
| **`FEAT-114`** | Visual Resistor Color-Band, SMD Code & Engineering Quick-Calculator HUD | `Phase 1` | `C-P1-02` | `AG-P1-01` (`signal-core::eng_calculators`) | `CU-P1-06` (`libs/academy`) |
| **`FEAT-115`** | Open-Silicon Multi-Project Wafer (`TinyTapeout` / `ChipIgnite`) Guided Wizard | `Phase 8` | `C-P8-01` | `AG-P8-02` (`signal-ic::mpw_shuttle`) | `CU-P8-01` (`libs/academy`) |
| **`FEAT-116`** | Enterprise Compute FinOps, Engineering Velocity & Budget-Cap Console | `Phase 8` | `C-P8-01` | `AG-P8-01` (`signal-operator::metering`) | `CU-P8-02` (`libs/admin`) |
| **`FEAT-117`** | Organization Policy-as-Code & Custom Rust/WASM Rule-Pack Studio | `Phase 8` | `C-P8-01` | `AG-P8-01` (`signal-pod::wasm_policy_runner`) | `CU-P8-02` (`libs/admin`) |
| **`FEAT-118`** | Cross-Project Portfolio & Shared-IP Blast-Radius Dependency Graph | `Phase 8` | `C-P8-01` | `AG-P8-01` (`signal-git-sync`) | `CU-P8-02` (`libs/admin`) |
| **`FEAT-119`** | Zero-Leak Time-Boxed External Partner / CM / Auditor Guest Review Portal | `Phase 2` | `C-P2-01` | `AG-P2-03` (`signal-pod::readonly_view_filter`) | `CU-P2-06` (`libs/review`) |
| **`FEAT-120`** | Developing-Nation Offline-Resilient Campus Edge Appliance (`Signal Campus Edge`) | `Phase 8` | `C-P8-01` | `AG-P8-03` (`infra/campus-edge/`) | `CU-P8-03` (`apps/signal-web`) |

---

## 4. All `12` Innovations (`I-01`–`I-12`) & All `148` Audit Flaw Guardrails (`F-001`–`F-148`) → Task Map

### 4.1 Twelve Futuristic AI-EDA Innovations (`I-01`–`I-12`)

| ID | Innovation | Phase | Antigravity Task | Cursor Task |
|---|---|---|---|---|
| **`I-01`** | End-to-End Differentiable Circuit, Thermal & Timing Physics (`signal-diff-sim`) | `Phase 1` | `AG-P1-06` | `CU-P1-04`, `CU-P3-04` |
| **`I-02`** | Physics-Informed Neural Operators (`signal-surrogate`) + Exact Rust Sign-Off | `Phase 3` | `AG-P3-04` | `CU-P3-04` |
| **`I-03`** | Neuro-Symbolic Formal Verification & Equivalence Proving (`signal-formal`) | `Phase 3` | `AG-P3-03` | `CU-P3-04` |
| **`I-04`** | 3D Full-Wave EM, 224G SerDes & Co-Packaged Optics (`signal-em3d`) | `Phase 4` & `6` | `AG-P4-02`, `AG-P6-02` | `CU-P4-01`, `CU-P6-01` |
| **`I-05`** | Mission-Profile Silicon & Component Aging Engine (`signal-aging`) | `Phase 4` | `AG-P4-04` | `CU-P4-03` |
| **`I-06`** | Post-Silicon & Lab Bench Digital Twin Correlation Loop (`signal-twin`) | `Phase 6` | `AG-P6-06` | `CU-P6-04` |
| **`I-07`** | IEEE 1801 Unified Power Format (UPF) Multi-Voltage Architecture | `Phase 6` | `AG-P6-01` | `CU-P6-01` |
| **`I-08`** | Unified DFT, Scan/ATPG, IEEE 1149.1/1687 JTAG & ICT Test Synthesis | `Phase 4` & `6` | `AG-P4-05`, `AG-P6-01` | `CU-P4-04`, `CU-P6-01` |
| **`I-09`** | Privacy-Preserving Live Supply-Chain, Lead-Time & Zero-Spin BOM Rescue | `Phase 3` | `AG-P3-02` | `CU-P3-05` |
| **`I-10`** | Sandboxed Clean-Room Legacy CAD & Multimodal Whiteboard Migration (`signal-import`) | `Phase 3` | `AG-P3-05` | `CU-P3-03` |
| **`I-11`** | Hybrid Post-Quantum Cryptography (PQC) & Confidential GPU Computing | `Phase 1`, `2`, `8` | `AG-P1-01`, `AG-P2-04`, `AG-P8-04` | `CU-P0-05`, `CU-P8-04` |
| **`I-12`** | Two-Phase Cross-Repo `Super-Commit` & Padmé-Blinded Convergent Vault (`signal-git-sync`) | `Phase 2` | `AG-P2-01`, `AG-P2-02` | `CU-P2-02` |

### 4.2 Complete `148` Resolved Audit Flaws (`F-001`–`F-148`) → Implementing & Verifying Tasks

| Audit Category / Domain | Flaw IDs Covered | Implementing & Verifying Tasks |
|---|---|---|
| **Cat A — Storage, Network & Topology Alignment** | `F-001`–`F-010` | `AG-P0-04`, `AG-P2-01`, `AG-P2-02`, `AG-P6-01` (`signal-federation-broker` zero lateral pod traffic), `CU-P2-02`, `CU-P2-03` |
| **Cat B — Multi-Repo BYO-Git, Rate Limits & Key Lifecycle** | `F-011`–`F-022` | `C-P2-01`, `AG-P2-01` (2PC `Super-Commit`, `provider_repo_id`, blobless LFS, Padmé `AES-256-GCM-SIV`), `CU-P2-02` (`<= 40/min` lazy pacer, `vcs-auditor`) |
| **Cat C — Pure-Rust SPICE, Geometry & Multi-Physics** | `F-023`–`F-038` | `AG-P1-01` (`0.5-ULP` `core_math`), `AG-P1-02` (`i128`/Shewchuk/Euler), `AG-P1-04`..`AG-P1-06` (Ruiz/`q(v)`/`Φ(i)`/TR-BDF2/PSS/Saltation), `AG-P3-03` (`signal-formal`), `AG-P4-02`, `AG-P4-04`, `AG-P6-01` |
| **Cat D — K8s Runtime, CRDT Consensus & Scaling** | `F-039`–`F-051` | `AG-P1-03` (topological CRDT, causal undo, CoW), `AG-P2-03`, `AG-P2-04` (`signal-operator` Leases, warm pool, memory governor, live handoff) |
| **Cat E — AI Agents, Datasheet RAG & Supply Chain** | `F-052`–`F-065` | `AG-P2-05` (3-way pin verifier), `AG-P3-01`..`AG-P3-05` (Dual-VLM, errata, `signal-surrogate`, mirrored BOM, legacy importers), `CU-P3-01`..`CU-P3-05` (`GBNF`, quiescent guardian) |
| **Cat F — Angular `v22+` UX, Zero-Leak Streaming & FastAPI** | `F-066`–`F-077` | `AG-P1-01` (SI parser `m` vs `Meg`), `AG-P4-06` (composited GPU atlas), `CU-P0-01`..`CU-P0-04`, `CU-P1-01`..`CU-P1-05` (`turns:443`/WebTransport, virtual scroll, Okabe-Ito, volatile 60s ring buffer, cloud clipboard), `CU-P2-06`, `CU-P4-05` |
| **Cat G — Zero-Trust AppSec, PQC, DLP & SFTP** | `F-078`–`F-092` | `AG-P0-01` (`cargo vet`/`deny`), `AG-P1-01` (`SecretBox`/PQC), `AG-P2-02` (socket-pinned anti-SSRF), `AG-P3-02` (partitioned `pgvector`), `AG-P3-05` (bomb caps), `AG-P4-06` (Tardos/Constellation), `AG-P4-07` (`russh` atomic SFTP), `CU-P0-04` (uniform `404`), `CU-P0-05` (RFC-3161 WORM), `CU-P2-01` (2-of-3 Break-Glass, `expires_at`) |
| **Cat H — Identity, Sovereignty, Legal & FinOps** | `F-093`–`F-100` | `CU-P0-04` (`/org/:org_slug` isolation), `CU-P0-05` (GDPR pseudonymous `actor_hmac`), `CU-P2-01`..`CU-P2-03` (Dual SSO+PKCE, `USER_NAMESPACE`, zero-bypass `local-guest-0001`, 21 CFR Part 11 `ML-DSA`, SPDX license gate), `CU-P8-02` (budget caps) |
| **Cat I — Contracts, Local Mac Dev, Multi-IDE & Lifecycle** | `F-101`–`F-112` | `C-P0-01`..`C-P0-03` (`<= 1 MB` Arrow chunks, SI suffixes, build-time codegen), `AG-P0-01`, `AG-P0-03`, `AG-P0-05` (`dev:lite < 1.5 GB`, native `arm64`), `AG-P1-05` (`benchmarks/`), `AG-P6-06` (`signal-twin`), `CU-P0-04` (`Idempotency-Key`), `CU-P5-05` (Digital Product Passport carbon/PFAS) |
| **Domain J — Hybrid Adjoints, Foundry IP & Solver Microarch** | `F-113`–`F-116`, `F-134`–`F-137`, `F-148` | `AG-P1-04` (AOT Cranelift Verilog-A, child-MicroVM `OSDI`/`IBIS-AMI`), `AG-P1-05` (`KLU`↔`cuDSS` crossover), `AG-P1-06` (Filippov Saltation + `Revolve`), `AG-P4-01` (Fab Profiles, PCB ECO), `AG-P4-07` / `CU-P4-05` (Lab/MCAD SFTP egress), `AG-P6-01`, `AG-P6-02` (Metal IC ECO, Pelgrom/LVF, IEEE 1735, Baseband CPO, 4K Cryo) |
| **Domain K — Padmé GCM-SIV, H100 vs B200 CC & Quorum Audit** | `F-117`–`F-118`, `F-140`–`F-141`, `F-147` | `AG-P1-01` (2-level `HKDF`), `AG-P2-01` (Padmé + path-blinded `blobs/*.sigv` + `.signal/vault.merkle.sig` `AES-256-GCM-SIV` + BYOK Continuity WAL), `AG-P2-04` (Exclusive `H100 CC` vs `B200 TEE-I/O` MIG), `CU-P0-05` (Fail-Closed 3x Quorum Tier-0 Audit Outbox) |
| **Domain L — External Git Fencing, WASM Cells & Node Leases** | `F-119`–`F-125`, `F-138`–`F-139`, `F-142` | `AG-P1-03` (CRDT power-rail short rejection & dependent undo), `AG-P2-01`, `AG-P2-02` (Dual-Gate fencing, `<= 40/min` pacer, blobless clones + `64 KB` LFS, external push auto-rebase), `AG-P2-03` (`< 8 MB` `signal-cell-wasm`), `AG-P2-04` (Host-Aggregated Leases, `< 150 ms` Pre-Copy migration), `AG-P6-01` (`*.halo.gds.sig`), `CU-P2-03` (In-Jurisdiction Paired-Region DR) |
| **Domain M — Browser Codecs, COOP/PKCE, Sharded RAG & `bigint`** | `F-126`–`F-133`, `F-143`–`F-146` | `C-P0-01` (`[jstype = JS_STRING]` / `bigint` + `contracts/transcripts/*.jsonl`), `AG-P0-01` (`signal-proto` + `lavapipe` WGSL CI), `AG-P3-01`, `AG-P3-02` (3-Tier `<= 500` partitions `pgvector`, Multimodal VLM quarantine, `C(V,T,f)` curves, padded k-anonymous BOM batches), `AG-P4-06` (Constellation watermarks), `CU-P0-03` (Route `COOP`/`COEP` + `__Host-` PKCE), `CU-P1-01` (`Zstd-QOI`/WebP + `HEVC 4:4:4`/Dual-Luma), `CU-P3-02` (Merge-time proof delta re-verifier) |
