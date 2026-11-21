# 10 — Canonical API & Data Contracts

> **Scope:** Defines the contract-first interfaces connecting Angular, Node.js Control Plane, Kubernetes Rust Engine Pods, AI Agents, Universal Datasheet RAG, and Authorized SFTP Egress. Locking these contracts first eliminates cross-service drift and prevents deadlocks during parallel Antigravity + Cursor development.

---

## 1. Contract-First Governance, Unit Safety & Streaming Rules

All inter-service boundaries live in `contracts/` and are versioned (`v1`, `v2`). **Generated Rust (`build.rs`) and TypeScript (`pnpm codegen`) stubs are produced deterministically at build time and NEVER committed to Git** (preventing noisy parallel-branch merge conflicts).

### Wire, `bigint` Precision & Stateful Protocol Invariants
1. **Mandatory SI-Unit-Suffixed Fields + 64-Bit `bigint` Precision (`Zero 2^53 Rounding!`):** Numeric physical fields MUST carry explicit unit suffixes (`_nm`, `_ps`, `_fs`, `_uv`, `_na`, `_uw`, `_mkelvin`, `_hz`, `_mohm`). Because JavaScript `Number.MAX_SAFE_INTEGER` (`2^53 - 1`) silently corrupts 64-bit `i64` nanometer/femtosecond/epoch values, **100% of `int64`/`uint64`/`sint64`/`fixed64` fields enforce `jstype = JS_STRING` on JSON/REST and ES2020 `bigint` (`longAsBigint: true`) in TypeScript**, verified via `contracts/fixtures/`.
2. **Bounded Chunked Streaming (`<= 1 MB`) + Apache Arrow Zero-Copy Framing:** Waveforms, layout diffs, and SFTP archives stream in `<= 1 MB` AEAD/CRC32C-framed chunks with credit-based backpressure.
3. **Executable Stateful Protocol Transcripts (`contracts/transcripts/*.jsonl`):** Beyond static type stubs, multi-step request/response/error state-machine transcripts verify exact call ordering (`HydrateProject -> MOUNTED -> ApplyTransaction`), idempotent retry semantics (`Idempotency-Key`), and `N-1` rolling-upgrade handshakes across both Antigravity (`engine/`) and Cursor (`web/`/`services/`) in CI!

```text
contracts/
├── proto/signal/v1/
│   ├── session.proto          # Host-Lease/Epoch hydration, N-1 capability handshake, eviction, FinOps metering
│   ├── design_graph.proto     # Idempotent GraphOps, Dependency-Causal CRDT, CoW branches, UPF, ECN, Variants
│   ├── vcs_sync.proto         # Multi-repo BYO-Git lazy materialization, History Scrubber & Dual-Fenced Super-Commit
│   ├── component.proto        # ElectronicModule manifests, multi-physics/optical/cryo ports, SOA, Aging, Curves
│   ├── universal_parts.proto  # 3-Tier Universal Parts, Master Pin Table, IEEE/IEC Symbols, Padstacks & 3D STEP
│   ├── simulation.proto       # KLU/cuDSS SPICE, Saltation+Revolve Diff-Sim, PINNs, Virtual Bench & Arrow waveforms
│   ├── formal_verify.proto    # SAT/SMT/IC3 formal property proofs, RTL-to-Gate LEC, CDC/UPF & FTA cut-sets
│   ├── em_thermal.proto       # 2D BEM + GPU 3D Full-Wave EM, 224G SerDes, Baseband CPO, TDR & thermal FVM
│   ├── magnetics_power.proto  # Planar/bobbin iGSE magnetics, PDN Z_target, 3D creepage, hot-loop, BMS/FOC
│   ├── emc_safety.proto       # CISPR/FCC emissions, ESD/surge, ISO 26262 FMEDA/WCCA, Rad-Hard SEU, Derating
│   ├── dfm_harness.proto      # Multi-board 3D mate, rigid-flex bend, 1:1 nailboard, panels, fatigue, should-cost
│   ├── mcad_cam_print.proto   # Parametric B-Rep CAD, sheet-metal, CNC CAM toolpaths & direct-B-Rep 3D slicing
│   ├── plm_ops.proto          # Multi-view eBOM/mBOM/sBOM, ECO/ECN, MCN, PCN, Deviations, NCR/CAPA & CTB
│   ├── fpga_firmware.proto    # FPGA SLR/SSO floorplan, SAT pinmux, protocol decoders, battery energy & DSP/FSM
│   ├── hw_security.proto      # DPA/TVLA side-channel leakage, laser fault-injection & RTL GLIFT Trojan audits
│   ├── digital_twin.proto     # Lab oscilloscope / VNA / ATE Shmoo measurement ingestion & board-map bring-up
│   ├── import_sandbox.proto   # Clean-room legacy CAD (KiCad/Altium/OrCAD/GDSII), sketch & datasheet reconstruction
│   ├── layout_verify.proto    # Exact i128/Euler geometry, glass-weave, Fab Profiles, Metal/PCB ECO, 20um Halo, OPC
│   ├── render_stream.proto    # Dual-Plane Zstd-QOI/WebP dirty tiles, HudOverlayPrimitive, 6-DOF & Constellation
│   ├── agent_tools.proto      # RBAC-scoped Neuro-Symbolic AI tools, Pareto explorer, Red-Team & NL rule compiler
│   ├── academy_campus.proto   # Breadboard twin, deterministic lab grader, TinyTapeout wizard & Campus Edge sync
│   └── sftp_release.proto     # Fail-closed PQC-signed RAM-to-SFTP fab/CNC/3D-print/ECN/MCN/turnkey CM release
├── openapi/
│   └── control-plane.v1.yaml  # URL-scoped (/org/:slug) Auth, Access Review, BYO-Git, Parts/AVL, PLM-Ops & SFTP
├── schemas/
│   ├── datasheet-record.schema.json
│   ├── universal-part-manifest.schema.json
│   ├── universal-padstack.schema.json
│   ├── universal-symbol.schema.json
│   ├── universal-footprint.schema.json
│   ├── org-avl-overlay.schema.json
│   ├── parts-lockfile.schema.json
│   ├── project-template.schema.json
│   ├── fab-capability-profile.schema.json
│   ├── mcad-feature-tree.schema.json
│   ├── cam-print-job.schema.json
│   ├── mbom-structure.schema.json
│   ├── ecn-change-order.schema.json
│   ├── mcn-change-notice.schema.json
│   ├── pcn-triage.schema.json
│   ├── deviation-waiver.schema.json
│   ├── ncr-capa.schema.json
│   ├── upf-power-intent.schema.json
│   ├── winding-magnetics.schema.json
│   ├── emc-safety-policy.schema.json
│   ├── dfm-panel-plan.schema.json
│   ├── hw-security-audit.schema.json
│   ├── super-commit-manifest.schema.json
│   └── sftp-release-manifest.schema.json
├── fixtures/                  # Cross-IDE golden bigint/SI-unit serialization vectors
└── transcripts/               # Replayable multi-step client<->server state-machine conformance traces
```

---

## 2. Core Rust `ElectronicModule` Interface Specification (Conceptual Blueprint)

Every passive, active, IC, RF, optical, cryo-quantum, digital, and hierarchical subcircuit module in `signal-component` conforms to this conceptual contract:

- **`Metadata & Provenance`:** `module_id`, `version`, `orderable_mpn`, `package_code`, `datasheet_ref_id`, `verification_badge` (`BRONZE_PARSED` | `SILVER_SIM_MATCHED` | `GOLD_ATTESTED`), `spdx_license`, `must_encrypt_taint: bool`.
- **`Typed Multi-Physics Ports (`ports() -> &[PortDescriptor]`):`** `(port_id, name, PhysicalDomain: Electrical|Thermal|DigitalLogic|RFWave|OpticalEnvelope|Mechanical|QuantumFlux, direction_hint)`.
- **`Condition-Parameterized Distributions & Aging (`ParameterSpec`):`** Nominal SI value, tolerance PDF (`Gaussian`, `Uniform`, `LogNormal`, `PelgromSpatialMismatch`), `tc1`/`tc2`, **multi-dimensional operating-condition curves (`C(V_bias, T, f)`, `R_thetaJC/JB` coupled to routed PCB copper area)**, `4 K–10 mK` cryo regularization, and 15-year aging coefficients.
- **`Charge/Flux-Conserving & Saltation-Differentiable Acausal MNA Kernel:`**
  - `stamp_dc(ctx, state, equilibrated_matrix)` — stamps conductance & analytical Newton-Raphson Jacobian (`C^2`-smooth limited).
  - `stamp_ac(ctx, freq_hz, complex_matrix)` — stamps small-signal admittance & adjoint noise sources.
  - `stamp_transient(ctx, dt, method, state, matrix)` — stamps **exact charge `q(v)` and flux `Φ(i)` conservation equations** (`dq/dt`, `dΦ/dt`).
  - `stamp_adjoint_and_saltation(ctx, adjoint_rhs, switching_surface)` — provides exact analytical derivatives $\partial f / \partial p$ AND **Filippov Saltation Event-Jump Matrices** across switching instants for **`signal-diff-sim`**.
- **`Directed Signal-Flow & 3-Speed Co-Sim Kernel:`** `on_port_event(ctx, port_id, event) -> Vec<ScheduledPortEvent>`.
- **`Autonomous Thermal, SOA, Aging & Fault Kernel:`** `compute_dissipation`, `evaluate_soa`, `evaluate_aging(mission_hours, stress_history) -> AgedParamShift`, and `apply_fault` (for automated ISO-26262 FMEDA).

---

## 3. Control Plane REST API Surface (`control-plane.v1.yaml`)

*(All mutating routes require `Idempotency-Key` + explicit fail-closed auth decorator; unauthorized resource lookups return uniform `404 Not Found`; Tier-0 mutations enforce synchronous 3x quorum audit pre-commit.)*

| Domain | Endpoints (Representative Contract) | Auth / Security Gate |
|---|---|---|
| **Identity & Dual-Bound Auth** | `GET/POST /v1/auth/oauth/{github\|gitlab\|bitbucket}` (top-level PKCE redirect, `SameSite=Lax` -> `__Host-` `Strict` upgrade), `POST /v1/auth/sso/{org_slug}/bind-git`, `POST /v1/auth/2fa/enroll`, `POST /v1/auth/2fa/verify-stepup`, `POST /v1/auth/break-glass/quorum` | Public / PKCE + SAML Dual-Bind / Step-Up Passkey or Burn-Listed TOTP |
| **Enterprise Onboarding** | `POST /v1/enterprises/apply`, `POST /v1/enterprises/{id}/verify-domain`, `POST /v1/admin/enterprises/{id}/approve` (pins paired `jurisdiction_region`), `PATCH /v1/orgs/{org_slug}/security-policy` | Corporate Email / Platform Approver / Enterprise Super Admin (2FA) |
| **Projects, Lazy Repos, ECN & Import** | `POST /v1/orgs/{org_slug}/projects` (sync `<slug>-meta`), `GET /v1/orgs/{org_slug}/projects`, `POST .../subprojects` (lazy/paced `<slug>-<type>`), `POST .../ecn` (formal ECO sign-off), `POST .../import` (clean-room CAD/sketch) | JWT + Live Git Org/Repo User RBAC + SPDX Policy Gate |
| **Templates & Fab Profiles** | `GET /v1/templates`, `POST /v1/orgs/{org_slug}/templates`, `POST /v1/orgs/{org_slug}/fab-profiles` (`*.fabprofile.toml`), `POST .../projects/from-template/{template_id}` | JWT + Template Scope RBAC |
| **BYO-Git Settings (2FA)** | `PUT /v1/orgs/{org_slug}/vcs/connection` (`GITHUB\|GITLAB\|BITBUCKET`, `ORG_NAMESPACE\|USER_NAMESPACE`, optional pinned Root CA, `ENCRYPTED_VAULT`/`PLAIN_CANONICAL`), `POST /v1/orgs/{org_slug}/vcs/connection/verify` | **Mandatory Fresh Step-Up 2FA** + Socket Anti-SSRF + Sync Audit |
| **Access Review (Org & Project Admin)** | `GET /v1/orgs/{org_slug}/access-review`, `GET /v1/orgs/{org_slug}/projects/{id}/access-review`, `POST .../access-review/attest` (21 CFR Part 11 PQC sign-off), `DELETE .../members/{provider_user_id}` | **Org Admin** or **Project Admin** (**Step-Up 2FA** + Sync Audit to revoke) |
| **Multi-Provider Security Auditor** | `GET /v1/orgs/{org_slug}/vcs/security-audit/latest`, `POST /v1/orgs/{org_slug}/vcs/security-audit/run`, `POST .../findings/{id}/remediate` | Security Officer / Admin RBAC (**Step-Up 2FA** for remediation) |
| **Active Sessions & Cloud Clipboard** | `POST /v1/orgs/{org_slug}/projects/{id}/session/attach` (checks mTLS device posture + issues `60 s` Ed25519 Envoy stream ticket), `POST /v1/orgs/{org_slug}/clipboard/copy-ticket` | JWT + Live Per-Repo RBAC + Sovereign ABAC + Org DLP Policy |
| **Authorized SFTP (2FA)** | `POST /v1/orgs/{org_slug}/sftp-targets`, `PUT .../sftp-targets/{id}`, `POST .../sftp-targets/{id}/test`, `POST /v1/orgs/{org_slug}/projects/{id}/releases/push-sftp` (fab, OSAT, regulatory lab, or MCAD STEP) | **Mandatory Fresh Step-Up 2FA** + Sync Quorum Audit (+ Optional 4-Eyes & 21 CFR Part 11) |
| **Audit & Digital Product Passport** | `GET /v1/orgs/{org_slug}/audit-events`, `GET /v1/orgs/{org_slug}/projects/{id}/compliance-and-sustainability` | Security Officer / Auditor RBAC |

> **Explicitly Excluded Endpoints:** There are **ZERO** `/export`, `/download`, `/raw-file`, or `/print` endpoints anywhere in the API surface.

---

## 4. Rust Project Pod gRPC Contracts (`proto/signal/v1/`)

- **`ProjectSessionService`:** `HydrateProject` (blobless fetch + 1-shot KMS DEK unwrap), `AttachCollaborator` (`N-1` capability + hardware codec probe + per-repo RBAC map), `EvictCollaborator`, `ThreePhaseLiveMigrate`, `SuperCommitNow`, `HibernateAndZeroize`.
- **`DesignGraphService`:** `ApplyTransaction` (idempotent + power-short contention guard), `UndoActorLastTransaction` (read-after-write causal check), `CreateSpeculativeBranch`, `DiffBranches` (`<= 1 MB` chunked visual diff), `MergeBranch` (verifies `base_graph_merkle_root` + runs `< 1 s` incremental post-rebase delta check on AI proof bundle!), `ApplyEcoPatch`, `QueryTraceability`.
- **`SimulationService`, `FormalVerifyService`, `EmThermalService` & `DigitalTwinService`:** `RunAnalysis` (DC, AC, Transient, Noise, Pelgrom MonteCarlo, LVF/PVT, Aging, 3SpeedFWCoSim), `RunSaltationAdjointOptimization` (`signal-diff-sim` PSS/Revolve), `QueryPinnPreview` (`signal-surrogate`), `RunFormalProof` (SAT/SMT/IC3/LEC/UPF), `RunEm3dAndBasebandCpo`, `CorrelateLabMeasurement` (`signal-twin`), `QueryWaveformViewport` (Arrow binary chunks).
- **`AgentToolsService`:** `QueryConeOfInfluence`, `PlaceComponentModule`, `ConnectPorts`, `RunAdjointParetoSizing`, `RunAnalyticalPlacer`, `RunFabAwareDrcLvsPex`, `SynthesizeMetalOnlyEco`, `SyncFirmwarePinmux`, `CompileNlConstraintRule`, `ConsolidateBomSkus`, `RunPrincipalRedTeamAudit`, `GenerateReleaseChecklist`.
- **`ExtendedDomainServices` (`17-*`):**
  - `MagneticsPowerService`: `SynthesizeTransformerWinding`, `OptimizePdnTargetImpedance`, `Verify3dGeodesicCreepage`, `SolveDcIrJouleMap`, `ExtractHotLoopInductance`, `SynthesizeCompensator`, `SimulateBmsPackOrFocMotor`, `VerifyPowerSequence`.
  - `EmcSafetyService`: `RunPreComplianceEmissions`, `SynthesizeEmiFilter`, `InjectEsdSurgePulse`, `RunAutomatedFmeda`, `SolveFaultTreeCutSets`, `RunAdjointWcca`, `InjectRadHardSeu`, `AuditComponentDerating`.
  - `DfmHarnessService`: `CheckMultiBoardClearance`, `EvaluateRigidFlexBend`, `GenerateHarnessNailboard`, `ComposeFabPanel`, `RunDfaAndSolderFatigue`, `EstimateLiveShouldCost`, `ComputeIctTestCoverage`, `GenerateIpc7351FootprintStep`.
  - `FpgaSecurityService`: `ValidateFpgaFloorplanAndSso`, `SolveSatPinmux`, `DecodeBusProtocolStream`, `ProfileFirmwareBatteryLife`, `SynthesizeFsmOrDsp`, `AuditHwSideChannelAndIft`, `GradeAcademyLab`.
  - `UniversalPartsService` (`18-*`): `ResolveUniversalPart` (3-Tier Global + Org AVL + Project Lockfile), `AdaptPadstackToFabProfile`, `VerifyMasterPinBijection`, `VerifyThreeWayStepPadAlignment`, `ComputePartRevisionDiff`, `PinPartInLockfile`.
  - `McadCamPrintService` (`19-*`): `SolveSketchConstraints`, `EvaluateBRepFeatureTree`, `UnfoldSheetMetal`, `SyncEcadMcadAnchors`, `GenerateCncToolpathAndSim`, `SliceBRepFor3dPrint`, `StreamUnified3dScene`.
  - `PlmOpsService` (`19-*`): `CompileMultiViewBom` (`eBOM`/`mBOM`/`sBOM`/`Costed`), `AnalyzeChangeBlastRadius`, `TransitionChangeOrder` (`ECR`/`ECO`/`ECN`/`MCN`/`PCN`/`Deviation`/`NCR`), `VerifyDeviationGate`, `ComputeClearToBuild`.

---

## 5. Canonical Data Schemas (`contracts/schemas/`)

### 5.1 Universal Datasheet Record Schema (`datasheet-record.schema.json`)
- `component_uid`, `orderable_mpn`, `package_code`, `die_revision`, `manufacturer`, `tenant_scope` (`GLOBAL_VERIFIED` | `org_id`)
- `verification_badge` (`BRONZE_PARSED` | `SILVER_SIM_MATCHED` | `GOLD_ATTESTED`), `spdx_license`
- `lifecycle_status` (`ACTIVE`, `NRND`, `EOL`, `OBSOLETE`), `pcn_alerts[]`, `errata_refs[]`, `qualifications[]`
- `absolute_max_ratings` (explicit SI-suffixed `bigint`-safe bounds), `electrical_params[]` (`min`, `typ`, `max`, `unit`, **`test_conditions: { v_bias_uv, temp_mkelvin, freq_hz, coupon_spec }`**, `page_bbox_citation`), `thermal_network: { r_theta_jc_mk_per_mw, r_theta_jb_mk_per_mw }`, `pins[]` (3D STEP & IPC-7351 verified), `condition_curves[]`, `register_map_ref`
- `sustainability_passport`: `{ embodied_co2e_grams, rohs_compliant, reach_compliant, pfas_free, conflict_mineral_free }`
- `rust_module_binding` & `rag_chunks_ref`

### 5.2 Cross-Repo `Super-Commit`, Vault Merkle & PQC SFTP Release Manifests
- **`super-commit-manifest.schema.json` (`<project>-meta/.signal/repos.lock.json`):** `super_commit_id`, `fencing_epoch` (`bigint`), `timestamp_utc`, `repo_commits: { "<provider_repo_id>": { commit_sha, vault_merkle_root, dek_version } }`, `ml_dsa_65_ed25519_sig`.
- **`sftp-release-manifest.schema.json`:** `release_id`, `org_id`, `project_id`, `super_commit_id`, `bundle_type` (`PCB_FAB` | `IC_GDSII` | `REGULATORY_DOSSIER` | `MCAD_STEP`), `fab_profile_id`, `verification_attestation` (`erc: 0`, `drc: 0`, `lvs_clean: true`, `formal_lec_clean: true`, `soa_aging_clean: true`), `artifacts[]` (`sha256`), `approvals[]` (`actor_pseudo_id`, `meaning: AUTHOR|REVIEWER|QA_APPROVER`, `mfa_method`, `ml_dsa_65_ed25519_sig`), `sftp_target_id`, `atomic_rename_verified_sha256`.
