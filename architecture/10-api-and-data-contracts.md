# 10 — Canonical API & Data Contracts

> **Scope:** Defines the contract-first interfaces connecting Angular, Node.js Control Plane, Kubernetes Rust Engine Pods, AI Agents, Universal Datasheet RAG, and Authorized SFTP Egress. Locking these contracts first eliminates cross-service drift and prevents deadlocks during parallel Antigravity + Cursor development.

---

## 1. Contract-First Governance Rule

All inter-service boundaries live in `contracts/` (once Phase 0 begins) and are versioned (`v1`, `v2`). Code generators produce strongly typed clients/servers for **Rust (`tonic`/`prost`)**, **TypeScript/Node.js (`ts-proto` / OpenAPI)**, and **Angular** from a single canonical source.

```text
contracts/
├── proto/signal/v1/
│   ├── session.proto          # Pod assignment, sticky attachment, presence, leases
│   ├── design_graph.proto     # Transactional graph mutations, CoW branches, traceability
│   ├── component.proto        # ElectronicModule manifests, ports, tolerances, SOA
│   ├── simulation.proto       # SPICE/Mixed-Signal/Monte-Carlo/Thermal jobs & waveforms
│   ├── layout_verify.proto    # PCB/IC geometry queries, routing, DRC/LVS/PEX/STA
│   ├── render_stream.proto    # GPU viewport framing, WebRTC signaling, input events
│   ├── agent_tools.proto      # Typed AI Agent tool calls executed on the Rust Engine
│   └── sftp_release.proto     # Verified in-memory release streaming to SFTP egress
├── openapi/
│   └── control-plane.v1.yaml  # Auth, Enterprise Onboarding, Projects, Templates, 2FA SFTP
└── schemas/
    ├── datasheet-record.schema.json
    ├── project-template.schema.json
    └── sftp-release-manifest.schema.json
```

---

## 2. Core Rust `ElectronicModule` Interface Specification (Conceptual Blueprint)

Every passive, active, IC, RF, digital, and hierarchical subcircuit module in `signal-component` conforms to this conceptual contract (planning-level specification, not final implementation code):

### Module Capability Contract
- **`Metadata & Identity`:** `module_id`, `version`, `part_number`, `datasheet_ref_id`, `category` (`Passive`, `Semiconductor`, `AnalogIC`, `Digital`, `RF`, `Subcircuit`).
- **`Typed Ports (`ports() -> &[PortDescriptor]`):`**
  - Each port defines: `port_id`, `name` (e.g., `in`, `out`, `gate`, `drain`, `source`, `vdd`, `gnd`, `thermal_j`), `domain` (`Electrical`, `Thermal`, `DigitalLogic`, `RFPort`, `Mechanical`), `direction_hint` (`Bidirectional`, `Input`, `Output`, `Power`).
- **`Parameters & Statistical Distributions (`ParameterSpec`):**
  - Nominal SI value, manufacturing tolerance (`±%` / `±abs`), PDF (`Gaussian`, `Uniform`, `LogNormal`, `CorrelatedGroup`), temperature coefficients (`tc1`, `tc2`), DC-bias/voltage derating table, aging drift law, parasitic model toggles (`ideal`, `lumped_parasitics`, `full_rf`).
- **`Acausal Physics Kernel (MNA SPICE Stamping):`**
  - `stamp_dc(ctx, instance_state, matrix_builder)` — stamps linear conductance & nonlinear Newton-Raphson Jacobian + RHS residuals into the coupled circuit matrix.
  - `stamp_ac(ctx, freq_hz, complex_matrix_builder)` — stamps small-signal linearized admittance/capacitance/inductance & noise current sources.
  - `stamp_transient(ctx, dt, integration_method, state, matrix_builder)` — stamps dynamic companion models (`C*dv/dt`, `L*di/dt`, nonlinear charge/flux conservation).
- **`Directed Signal-Flow Kernel (Digital / Behavioral / Co-Sim):`**
  - `on_port_event(ctx, port_id, event_value) -> Vec<ScheduledPortEvent>` — propagates logic states, timing delays, control signals, or virtual MCU peripheral events.
- **`Autonomous Thermal, Stress & Fault Kernel:`**
  - `compute_dissipation(port_voltages, port_currents) -> Watts`
  - `evaluate_soa(port_voltages, port_currents, tj_kelvin) -> Vec<StressViolation>` (checks `V_max`, `I_max`, `P_derated`, `T_j_max`, `dV/dt`, `ESD` bounds).
  - `apply_fault(fault_mode: FaultInjection)` (`None`, `OpenPin(port)`, `ShortPins(p1, p2)`, `DriftMultiplier(f64)`).

**Why this contract is powerful:** Connecting `R1.out` to `R2.in` binds both modules to the same net node in `signal-graph`. Every solver (DC, AC, Transient, Noise, Thermal, Monte Carlo, Fault Injection) interacts ONLY with this uniform trait—meaning a resistor, a GaN FET, a BSIM-CMG FinFET, a digital NAND gate, or an entire hierarchical power stage plugs into the engine seamlessly!

---

## 3. Control Plane REST API Surface (`control-plane.v1.yaml`)

| Domain | Endpoints (Representative Contract) | Auth / Security Gate |
|---|---|---|
| **Identity & Auth** | `POST /v1/auth/register`, `POST /v1/auth/login`, `POST /v1/auth/sso/{org_slug}`, `POST /v1/auth/2fa/enroll`, `POST /v1/auth/2fa/verify-stepup` | Public / JWT / Step-Up 2FA |
| **Enterprise Onboarding** | `POST /v1/enterprises/apply`, `POST /v1/enterprises/{id}/verify-domain`, `POST /v1/admin/enterprises/{id}/approve`, `PATCH /v1/enterprises/{id}/security-policy` | Corporate Email / Signal Platform Approver / Enterprise Super Admin |
| **Projects & Sub-Projects** | `POST /v1/projects`, `GET /v1/projects`, `PATCH /v1/projects/{id}/metadata`, `POST /v1/projects/{id}/subprojects` (`PRD`, `HFS`, `SCHEMATIC`, `PCB`, `IC_*`, `FPGA`, `FIRMWARE`, `BIOS`), `GET /v1/projects/{id}/traceability` | JWT + Project RBAC |
| **Project Templates** | `GET /v1/templates`, `POST /v1/templates` (`PERSONAL` / `ENTERPRISE`), `POST /v1/projects/from-template/{template_id}` | JWT + Template Scope RBAC |
| **BYO-GitHub Org Settings (2FA)** | `PUT /v1/vcs/github-org` (org name, Fine-Grained PAT or GitHub App install ID, `ENCRYPTED_VAULT`/`PLAIN_CANONICAL`), `POST /v1/vcs/github-org/verify`, `DELETE /v1/vcs/github-org` | **Mandatory Fresh Step-Up 2FA** (Enterprise Admin / Individual Owner) |
| **GitHub Org Security Auditor** | `GET /v1/vcs/security-audit/latest`, `POST /v1/vcs/security-audit/run`, `POST /v1/vcs/security-audit/findings/{id}/remediate` | Security Officer / Admin RBAC (**Step-Up 2FA** for remediation) |
| **Active Sessions** | `POST /v1/projects/{id}/session/attach` → returns sticky WebSocket/WebTransport & WebRTC session token bound to the single active Project Pod | JWT + Project RBAC + Org DLP Policy |
| **Authorized SFTP (2FA)** | `POST /v1/sftp-targets`, `PUT /v1/sftp-targets/{id}`, `DELETE /v1/sftp-targets/{id}`, `POST /v1/sftp-targets/{id}/test-connection`, `POST /v1/projects/{id}/releases/push-sftp` | **Mandatory Fresh Step-Up 2FA** (+ Optional 4-Eyes Second Approver) |
| **Audit & Compliance** | `GET /v1/orgs/{id}/audit-events`, `GET /v1/projects/{id}/compliance-report` | Security Officer / Auditor RBAC |

> **Explicitly Excluded Endpoints:** There are **ZERO** `/export`, `/download`, `/raw-file`, or `/print` endpoints anywhere in the API surface.

---

## 4. Rust Project Pod gRPC Contracts (`proto/signal/v1/`)

### 4.1 `ProjectSessionService` (Control Plane ↔ Rust Pod / Operator)
- `HydrateProject(project_id, org_id, fencing_token, kms_grant, dlp_policy) -> PodReadyAck`
- `AttachCollaborator(user_id, role, watermark_identity, viewport_mode) -> SessionStreamTicket`
- `CheckpointNow(reason) -> EncryptedSnapshotManifest`
- `HibernateAndZeroize(fencing_token) -> ZeroizeAttestation`

### 4.2 `DesignGraphService` (Client / AI Agent ↔ Rust Pod)
- `ApplyTransaction(branch_id, base_rev, ops[], idempotency_key) -> CommitResult { new_rev, instant_erc_diagnostics[] }`
- `CreateSpeculativeBranch(parent_rev, label, owner_agent_id) -> BranchId`
- `DiffBranches(base_branch, candidate_branch) -> SemanticDesignDiff`
- `MergeBranch(candidate_branch, approval_signature) -> CommitResult`
- `QueryTraceability(entity_id, depth) -> TraceabilitySubgraph`

### 4.3 `SimulationService` (Client / AI Agent ↔ Rust Pod ↔ Burst Workers)
- `RunAnalysis(branch_id, subproject_id, analysis_spec: DC|AC|Transient|Noise|MonteCarlo|Corner|Thermal|MixedSignal|FWCoSim) -> stream SimProgressEvent`
- `MeasureWaveform(sim_run_id, measurement_exprs[]) -> MeasurementResults`
- `GetSoaAudit(sim_run_id) -> Vec<ComponentStressReport>`
- `QueryWaveformViewport(sim_run_id, signal_ids[], x_min, x_max, pixel_width) -> MinMaxDecimatedSeries`

### 4.4 `AgentToolsService` (AI Orchestrator ↔ Rust Pod)
Exposes deterministic, schema-checked tools so LLMs operate safely:
- `InspectSubcircuit`, `PlaceComponentModule`, `ConnectPorts`, `UpdateModuleParameters`, `RunVerificationSuite`, `ProposePcbFloorplan`, `RunDrcLvs`, `SyncFirmwarePinmux`, `GenerateReleaseChecklist`.

---

## 5. Canonical Data Schemas (`contracts/schemas/`)

### 5.1 Universal Datasheet Record Schema (`datasheet-record.schema.json`)
Every indexed electronic part in the Universal Datasheet DB stores:
- `component_uid`, `mpn`, `manufacturer`, `tenant_scope` (`GLOBAL` or `org_id` for private NDA datasheets)
- `category`, `package`, `lifecycle_status` (`ACTIVE`, `NRND`, `EOL`, `OBSOLETE`), `qualifications` (`AEC-Q100`, `MIL-STD`, `RoHS`, `REACH`)
- `absolute_max_ratings`: typed SI bounds (`v_max`, `i_max`, `p_max`, `tj_min`, `tj_max`, `esd_hbm`)
- `electrical_params`: array of `{ name, symbol, conditions, min, typ, max, unit, page_citation }`
- `pins`: array of `{ pin_number, name, electrical_type, voltage_domain, description }`
- `curves`: digitized lookup tables (e.g., `mlcc_dc_bias`, `rds_on_vs_tj`, `eff_vs_iout`)
- `register_map_ref`: optional SystemRDL/JSON register map for smart ICs/MCUs
- `rust_module_binding`: `{ crate_module_id, default_params, verified_accuracy_score, spice_model_hash }`
- `rag_chunks_ref`: linked vector/BM25 document chunks with exact `(page, section, bbox)` citations

### 5.2 Authorized SFTP Release Manifest Schema (`sftp-release-manifest.schema.json`)
Generated inside the Rust Pod and signed before transmission:
- `release_id`, `org_id`, `project_id`, `project_revision`, `subproject_ids[]`
- `bundle_type` (`PCB_FAB_GERBER_X2`, `PCB_ODBPP`, `PCB_IPC2581`, `IC_GDSII`, `IC_OASIS`, `FIRMWARE_BIN`, `SPEC_PACKAGE`)
- `verification_attestation`: `{ erc_errors: 0, drc_errors: 0, lvs_clean: true, soa_clean: true, requirements_passed_pct }`
- `artifacts[]`: `{ filename, size_bytes, sha256 }`
- `pgp_encrypted`: `boolean`, `pgp_key_fingerprint`
- `approvals[]`: `{ user_id, role, mfa_method, signed_at_utc }`
- `sftp_target_id`, `pinned_host_fingerprint`, `transfer_status`, `remote_verified_sha256`
