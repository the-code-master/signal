# 10 — Canonical API & Data Contracts

> **Scope:** Defines the contract-first interfaces connecting Angular, Node.js Control Plane, Kubernetes Rust Engine Pods, AI Agents, Universal Datasheet RAG, and Authorized SFTP Egress. Locking these contracts first eliminates cross-service drift and prevents deadlocks during parallel Antigravity + Cursor development.

---

## 1. Contract-First Governance, Unit Safety & Streaming Rules

All inter-service boundaries live in `contracts/` and are versioned (`v1`, `v2`). **Generated Rust (`build.rs`) and TypeScript (`pnpm codegen`) stubs are produced deterministically at build time and NEVER committed to Git** (preventing noisy parallel-branch merge conflicts).

### Wire & Semantic Safety Invariants
1. **Mandatory SI-Unit-Suffixed Field Names:** Numeric physical fields MUST carry explicit unit suffixes (`_nm`, `_ps`, `_uv`, `_na`, `_uw`, `_mkelvin`, `_hz`, `_mohm` — NEVER bare `voltage` or `width`!) backed by shared golden conformance vectors in `contracts/fixtures/`.
2. **Bounded Chunked Streaming (`<= 1 MB`) + Apache Arrow Zero-Copy Framing:** Waveforms, layout diffs, and SFTP archives stream in `<= 1 MB` AEAD/CRC32C-framed chunks with credit-based backpressure (never hitting gRPC's 4 MB unary message wall).
3. **Universal Idempotency & `N-1` Rolling Upgrade Handshake:** Every mutating REST/gRPC call requires a client `Idempotency-Key` (UUIDv7, 24h deduplication TTL); `AttachCollaborator` negotiates `(protocol_version, capabilities)` so `N-1` active Project Pods stay seamlessly compatible during rolling web deploys.

```text
contracts/
├── proto/signal/v1/
│   ├── session.proto          # Linearizable Lease pod hydration, N-1 capability handshake, eviction
│   ├── design_graph.proto     # Idempotent GraphOps, Topological CRDT, CoW branches, UPF & traceability
│   ├── vcs_sync.proto         # Multi-repo BYO-Git mount status & Two-Phase Super-Commit lockstep
│   ├── component.proto        # ElectronicModule manifests, multi-physics ports, tolerances, SOA, Aging
│   ├── simulation.proto       # Equilibrated SPICE, Adjoint Diff-Sim, PINN Surrogates & Arrow waveforms
│   ├── formal_verify.proto    # SAT/SMT/IC3 formal property proofs, RTL-to-Gate LEC & CDC/UPF checks
│   ├── em_thermal.proto       # 2D BEM + GPU 3D Full-Wave EM, 224G SerDes, CPO & multi-rate thermal
│   ├── digital_twin.proto     # Lab oscilloscope / VNA / ATE measurement ingestion & model calibration
│   ├── import_sandbox.proto   # Clean-room legacy CAD (KiCad/Altium/OrCAD/GDSII) & sketch import jobs
│   ├── layout_verify.proto    # Exact i128/arc geometry, placer/router, DRC/LVS/PEX/STA, DFT/JTAG
│   ├── render_stream.proto    # Composited YUV 4:4:4 lossless dirty-tiles + motion stream & Tardos watermarks
│   ├── agent_tools.proto      # RBAC-scoped Neuro-Symbolic AI tools & K-Hop cone-of-influence queries
│   └── sftp_release.proto     # PQC-signed (ML-DSA-65 + Ed25519) atomic RAM-to-SFTP release streaming
├── openapi/
│   └── control-plane.v1.yaml  # URL-scoped (/org/:slug) Auth, Access Review, BYO-Git, Auditor & 2FA SFTP
├── schemas/
│   ├── datasheet-record.schema.json
│   ├── project-template.schema.json
│   ├── upf-power-intent.schema.json
│   ├── super-commit-manifest.schema.json
│   └── sftp-release-manifest.schema.json
└── fixtures/                  # Cross-IDE golden serialization & unit-conversion test vectors
```

---

## 2. Core Rust `ElectronicModule` Interface Specification (Conceptual Blueprint)

Every passive, active, IC, RF, optical, digital, and hierarchical subcircuit module in `signal-component` conforms to this conceptual contract:

- **`Metadata & Provenance`:** `module_id`, `version`, `orderable_mpn`, `package_code`, `datasheet_ref_id`, `verification_badge` (`BRONZE_PARSED` | `SILVER_SIM_MATCHED` | `GOLD_ATTESTED`), `spdx_license`, `must_encrypt_taint: bool`.
- **`Typed Multi-Physics Ports (`ports() -> &[PortDescriptor]`):`** `(port_id, name, PhysicalDomain: Electrical|Thermal|DigitalLogic|RFWave|OpticalMode|Mechanical|QuantumFlux, direction_hint)`.
- **`Parameters, Distributions & Aging (`ParameterSpec`):`** Nominal SI value, tolerance PDF (`Gaussian`, `Uniform`, `LogNormal`, `CorrelatedMismatchGroup`), `tc1`/`tc2`, digitized DC-bias/saturation curves, and 15-year mission-profile aging coefficients.
- **`Charge/Flux-Conserving Acausal MNA Kernel:`**
  - `stamp_dc(ctx, state, equilibrated_matrix)` — stamps conductance & analytical Newton-Raphson Jacobian (`C^2`-smooth limited).
  - `stamp_ac(ctx, freq_hz, complex_matrix)` — stamps small-signal admittance & adjoint noise sources.
  - `stamp_transient(ctx, dt, method, state, matrix)` — stamps **exact charge `q(v)` and flux `Φ(i)` conservation equations** (`dq/dt`, `dΦ/dt`).
  - `stamp_adjoint(ctx, adjoint_rhs)` — provides exact analytical parameter derivatives $\partial f / \partial p$ for **`signal-diff-sim`**.
- **`Directed Signal-Flow & 3-Speed Co-Sim Kernel:`** `on_port_event(ctx, port_id, event) -> Vec<ScheduledPortEvent>`.
- **`Autonomous Thermal, SOA, Aging & Fault Kernel:`** `compute_dissipation`, `evaluate_soa`, `evaluate_aging(mission_hours, stress_history) -> AgedParamShift`, and `apply_fault` (for automated ISO-26262 FMEDA).

---

## 3. Control Plane REST API Surface (`control-plane.v1.yaml`)

*(All mutating routes require `Idempotency-Key` + explicit fail-closed auth decorator; unauthorized resource lookups return uniform `404 Not Found`.)*

| Domain | Endpoints (Representative Contract) | Auth / Security Gate |
|---|---|---|
| **Identity & Dual-Bound Auth** | `POST /v1/auth/oauth/{github\|gitlab\|bitbucket}` (PKCE), `POST /v1/auth/sso/{org_slug}/bind-git`, `POST /v1/auth/2fa/enroll`, `POST /v1/auth/2fa/verify-stepup`, `POST /v1/auth/break-glass/quorum` | Public / PKCE + SAML Dual-Bind / Step-Up Passkey or Burn-Listed TOTP |
| **Enterprise Onboarding** | `POST /v1/enterprises/apply`, `POST /v1/enterprises/{id}/verify-domain`, `POST /v1/admin/enterprises/{id}/approve` (pins `jurisdiction_region`), `PATCH /v1/orgs/{org_slug}/security-policy` | Corporate Email / Platform Approver / Enterprise Super Admin (2FA) |
| **Projects, Repos & Import** | `POST /v1/orgs/{org_slug}/projects` (`<slug>-meta`), `GET /v1/orgs/{org_slug}/projects`, `POST /v1/orgs/{org_slug}/projects/{id}/subprojects` (`<slug>-<type>`), `POST /v1/orgs/{org_slug}/projects/{id}/import` (clean-room legacy CAD/sketch) | JWT + Live Git Org/Repo User RBAC + SPDX Policy Gate |
| **Project Templates** | `GET /v1/templates`, `POST /v1/orgs/{org_slug}/templates`, `POST /v1/orgs/{org_slug}/projects/from-template/{template_id}` | JWT + Template Scope RBAC |
| **BYO-Git Settings (2FA)** | `PUT /v1/orgs/{org_slug}/vcs/connection` (`GITHUB\|GITLAB\|BITBUCKET`, `ORG_NAMESPACE\|USER_NAMESPACE`, optional pinned Root CA, `ENCRYPTED_VAULT`/`PLAIN_CANONICAL`), `POST /v1/orgs/{org_slug}/vcs/connection/verify` | **Mandatory Fresh Step-Up 2FA** + Socket Anti-SSRF Check |
| **Access Review (Org & Project Admin)** | `GET /v1/orgs/{org_slug}/access-review`, `GET /v1/orgs/{org_slug}/projects/{id}/access-review`, `POST /v1/orgs/{org_slug}/projects/{id}/access-review/attest` (21 CFR Part 11 PQC sign-off), `DELETE .../members/{provider_user_id}` | **Org Admin** or **Project Admin** (**Step-Up 2FA** to revoke or set `expires_at`) |
| **Multi-Provider Security Auditor** | `GET /v1/orgs/{org_slug}/vcs/security-audit/latest`, `POST /v1/orgs/{org_slug}/vcs/security-audit/run`, `POST .../findings/{id}/remediate` | Security Officer / Admin RBAC (**Step-Up 2FA** for remediation) |
| **Active Sessions & Cloud Clipboard** | `POST /v1/orgs/{org_slug}/projects/{id}/session/attach` (returns Ed25519 Envoy stream ticket), `POST /v1/orgs/{org_slug}/clipboard/copy-ticket` | JWT + Live Per-Repo RBAC + Sovereign ABAC + Org DLP Policy |
| **Authorized SFTP (2FA)** | `POST /v1/orgs/{org_slug}/sftp-targets`, `PUT .../sftp-targets/{id}`, `POST .../sftp-targets/{id}/test`, `POST /v1/orgs/{org_slug}/projects/{id}/releases/push-sftp` | **Mandatory Fresh Step-Up 2FA** (+ Optional 4-Eyes & 21 CFR Part 11 Sign-Off) |
| **Audit & Digital Product Passport** | `GET /v1/orgs/{org_slug}/audit-events`, `GET /v1/orgs/{org_slug}/projects/{id}/compliance-and-sustainability` | Security Officer / Auditor RBAC |

> **Explicitly Excluded Endpoints:** There are **ZERO** `/export`, `/download`, `/raw-file`, or `/print` endpoints anywhere in the API surface.

---

## 4. Rust Project Pod gRPC Contracts (`proto/signal/v1/`)

- **`ProjectSessionService`:** `HydrateProject`, `AttachCollaborator` (`N-1` capability negotiation + per-repo RBAC map), `EvictCollaborator`, `LiveMigrateHandoff`, `SuperCommitNow`, `HibernateAndZeroize`.
- **`DesignGraphService`:** `ApplyTransaction` (idempotent + selective actor ID), `UndoActorLastTransaction`, `CreateSpeculativeBranch` (port-encapsulated live-rebasing CoW), `DiffBranches` (`<= 1 MB` chunked visual diff), `MergeBranch` (attaches Immutable AI Design Proof Bundle), `QueryTraceability`.
- **`SimulationService`, `FormalVerifyService`, `EmThermalService` & `DigitalTwinService`:** `RunAnalysis` (DC, AC, Transient, Noise, MonteCarlo, PVT, Aging, 3SpeedFWCoSim), `RunAdjointOptimization` (`signal-diff-sim`), `QueryPinnPreview` (`signal-surrogate`), `RunFormalProof` (SAT/SMT/IC3/LEC/UPF), `RunEm3dExtraction`, `CorrelateLabMeasurement` (`signal-twin`), `QueryWaveformViewport` (Arrow binary chunks).
- **`AgentToolsService`:** `QueryConeOfInfluence`, `PlaceComponentModule`, `ConnectPorts`, `RunAdjointParetoSizing`, `RunAnalyticalPlacer`, `RunDrcLvsPex`, `SyncFirmwarePinmux`, `GenerateReleaseChecklist`.

---

## 5. Canonical Data Schemas (`contracts/schemas/`)

### 5.1 Universal Datasheet Record Schema (`datasheet-record.schema.json`)
- `component_uid`, `orderable_mpn`, `package_code`, `die_revision`, `manufacturer`, `tenant_scope` (`GLOBAL_VERIFIED` | `org_id`)
- `verification_badge` (`BRONZE_PARSED` | `SILVER_SIM_MATCHED` | `GOLD_ATTESTED`), `spdx_license`
- `lifecycle_status` (`ACTIVE`, `NRND`, `EOL`, `OBSOLETE`), `pcn_alerts[]`, `errata_refs[]` (co-linked silicon errata!), `qualifications[]`
- `absolute_max_ratings` (explicit SI-suffixed bounds), `electrical_params[]` (`min`, `typ`, `max`, `unit`, `page_bbox_citation`), `pins[]` (3D STEP & IPC-7351 verified), `curves[]`, `register_map_ref`
- `sustainability_passport`: `{ embodied_co2e_grams, rohs_compliant, reach_compliant, pfas_free, conflict_mineral_free }`
- `rust_module_binding` & `rag_chunks_ref`

### 5.2 Cross-Repo `Super-Commit` & PQC SFTP Release Manifests
- **`super-commit-manifest.schema.json` (`<project>-meta/repos.lock.json`):** `super_commit_id`, `fencing_token`, `timestamp_utc`, `repo_commits: { "<provider_repo_id>": { repo_name, commit_sha, tree_merkle_root, dek_version } }`.
- **`sftp-release-manifest.schema.json`:** `release_id`, `org_id`, `project_id`, `super_commit_id`, `bundle_type`, `verification_attestation` (`erc: 0`, `drc: 0`, `lvs_clean: true`, `formal_lec_clean: true`, `soa_aging_clean: true`), `artifacts[]` (`sha256`), `pgp_encrypted`, `approvals[]` (`user_uuid`, `meaning: AUTHOR|REVIEWER|QA_APPROVER`, `mfa_method`, `ml_dsa_65_ed25519_sig`), `sftp_target_id`, `atomic_rename_verified_sha256`.
