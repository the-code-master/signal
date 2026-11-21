# 15 — Application Security & Complete Project Isolation Architecture

> **Scope:** Defines Signal's end-to-end **Application Security (AppSec)** and **Complete Project Isolation** architecture—complementing [`07-security-encryption-dlp-and-sftp.md`](./07-security-encryption-dlp-and-sftp.md) (Encryption, DLP & 2FA SFTP) to engineer **OWASP ASVS Level 3** security across browser, API, database, compute, kernel, network, AI/RAG, and CI/CD supply chain so that **every single Project operates as a mathematically, cryptographically, and physically isolated security cell**.

---

## 1. Core Security Guarantee: "1 Project = 1 Isolated Security Cell"

No single control achieves complete security—true enterprise & semiconductor-grade security comes from **non-bypassable Defense-in-Depth** where every layer assumes adjacent layers could be attacked.

**Signal's Project Isolation Invariant:**
> *Even if an adversary compromises a user's browser session, uploads a weaponized datasheet/PDK/WASM plugin, triggers an AI prompt injection, exploits an application query bug, OR achieves arbitrary code execution inside one Project Pod—there MUST be **ZERO network, kernel, memory, cache, queue, database, or cryptographic path** to read, decrypt, pivot into, or corrupt **ANY other Project** (even another Project inside the same Organization).*

```mermaid
flowchart TB
    subgraph Ring1["Ring 1: Edge & Zero-Trust Browser AppSec (Angular)"]
        WAF["Cloud WAF + DDoS + Hybrid PQC TLS 1.3 + Strict CSP (Nonces) + Trusted Types"]
        Browser["Zero-Persistence Angular Shell (No IP in Storage/Cache, HttpOnly/SameSite Cookies)"]
    end

    subgraph Ring2["Ring 2: Auth, Live Provider Gate & API AppSec (Node.js BFF)"]
        Auth["Dual SSO + GitHub/GitLab/Bitbucket OAuth PKCE + Replay-Proof Step-Up 2FA"]
        API["Contract Validation + Uniform-404 BOLA Guard + Dirty-Hint Webhook Verifier"]
    end

    subgraph Ring3["Ring 3: Data-Plane, Partitioned Vector & Message-Bus Isolation"]
        PG["PostgreSQL RLS (NOBYPASSRLS) + Per-Org Partitioned pgvector HNSW"]
        Redis["Scoped Keys + TOTP Nonce Burn-List + K8s Linearizable Lease Fencing"]
        NATS["NATS JWT/NKey Per-Project Subject ACLs (signal.org.project.>)"]
    end

    subgraph Ring4["Ring 4: Compute, Confidential Silicon & Memory Isolation (K8s)"]
        subgraph CellA["Isolated Security Cell: Project A"]
            MicroVMA["Kata / gVisor MicroVM or SEV-SNP/TDX Confidential VM + MIG GPU"]
            PodA["Rust signal-pod (Read-Only Rootfs, RAM tmpfs, ZeroizeOnDrop, SecretBox)"]
            WASMA["Wasmtime Fuel/Memory-Metered Sandbox (Plugins / PCELLs)"]
            SVIDA["SPIFFE SVID: spiffe://signal/orgA/projA (ML-KEM-768 DEK_A Only)"]
        end
        subgraph CellB["Isolated Security Cell: Project B"]
            MicroVMB["Kata / gVisor MicroVM or SEV-SNP/TDX Confidential VM + MIG GPU"]
            PodB["Rust signal-pod (Read-Only Rootfs, RAM tmpfs, ZeroizeOnDrop, SecretBox)"]
            WASMB["Wasmtime Fuel/Memory-Metered Sandbox"]
            SVIDB["SPIFFE SVID: spiffe://signal/orgA/projB (ML-KEM-768 DEK_B Only)"]
        end
    end

    subgraph Ring5["Ring 5: eBPF Network, Socket-Pinned Egress & Runtime Sensor"]
        Cilium["Cilium eBPF Default-Deny (Cell A <-> Cell B 100% DROPPED!)"]
        Egress["signal-egress-proxy (Socket-Level IP Pinning + Custom Root CA + Write-Only SFTP)"]
        Falco["Runtime eBPF Threat Sensor (Tetragon / Falco -> Instant Pod Quarantine)"]
    end

    subgraph Ring6["Ring 6: Dual-LLM Quarantine & Partitioned Datasheet RAG"]
        AIRBAC["Quarantined Reader LLM (Zero Tools) -> GBNF Schema -> Scoped Planner Agent"]
        RAGISO["Physically Partitioned Global vs Per-Org NDA pgvector Tables"]
    end

    WAF --> Browser --> Auth --> API
    API --> PG
    API --> Redis
    API --> NATS
    API --> Cilium
    Cilium --> CellA
    Cilium --> CellB
    CellA x--x|"HARD BLOCKED (eBPF + MicroVM/SEV-SNP + MIG + Distinct PQC DEK)"| CellB
    CellA --> Egress
    CellB --> Egress
    Falco -.-> CellA
    Falco -.-> CellB
    API --> AIRBAC --> RAGISO
```

---

## 2. Project-to-Project Isolation Matrix (Every Layer Audited)

This matrix governs BOTH **Cross-Organization Isolation** (Company A vs. Company B) AND **Intra-Organization Project Isolation** (Project X vs. Project Y inside the same company, plus Sub-Project Repo isolation inside Project X):

| Layer | Isolation Mechanism | What Happens If Layer Above Is Breached |
|---|---|---|
| **Browser / Angular UI** | Strict origin isolation, `Cross-Origin-Opener-Policy: same-origin`, `Cross-Origin-Embedder-Policy: require-corp`, CSP with per-request cryptographic nonces, Angular Trusted Types (XSS immunity), URL-scoped `/org/:org_slug/project/:proj_slug`, zero design data in `localStorage`/`IndexedDB`/SW cache (volatile JS RAM closure only for `<= 60 s` reconnect buffer). | Compromised extension, multi-tab context, or stolen laptop disk cannot scrape cross-origin tokens or pull cached design files. |
| **Authentication & Sessions** | Dual-Identity SAML/OIDC + OAuth 2.0 PKCE (`provider_user_id`), `HttpOnly; Secure; SameSite=Strict` cookies, short-lived signed JWTs (`<= 10 min`), live Git Org + Repo (`provider_repo_id`) + contractor `expires_at` verification on every project attach, instant Redis revocation + pod eviction. | Stolen, expired, or revoked user loses access across all active Project Pods within `< 2 s`; never gains access to unlisted projects. |
| **Node.js Control Plane / BFF** | Zero-trust BOLA/IDOR enforcement: every route extracts `(org_id, project_id, sub_project_id)` and checks live `REPO_ACCESS_ENTRY` before touching any resource. **Uniform `404 Not Found` Anti-Enumeration:** unauthorized AND non-existent resource UUIDs return identical constant-time `404` responses while logging an internal security anomaly. | Attackers probing `/v1/projects/<uuid>` learn ZERO information about whether another customer's or sibling team's project exists. |
| **PostgreSQL & `pgvector`** | **Mandatory Row-Level Security (RLS)** (`NOBYPASSRLS`) on every tenant/project table (`SET LOCAL app.org_id = ...; SET LOCAL app.user_id = ...;`). **Physically Partitioned `pgvector` HNSW Indexes:** each Enterprise Org's private NDA datasheets reside in a separate physical table partition (`datasheet_chunks_org_<hash>`) with its own isolated HNSW graph. | Eliminates missing-`WHERE` SQL leaks AND eliminates shared-HNSW vector recall starvation or neighbor-timing inference side channels! |
| **Kubernetes Lease, Redis & NATS** | Pod-per-Project single-writer exclusivity enforced by **linearizable Kubernetes `coordination.k8s.io/v1 Lease` (etcd Raft `resourceVersion`)** with monotonic `fencing_epoch`. Redis keys strictly namespaced (`org:{org}:proj:{proj}:*`). NATS enforces per-project **JWT/NKey subject ACLs** (`signal.{org}.{proj}.>`). | Network partitions or Redis failovers can never spawn split-brain writable pods; Pod A can never sniff or inject jobs on Pod B's NATS bus. |
| **Hybrid PQC Cryptography & KMS** | **Unique DEK per Project** wrapped via **Hybrid Post-Quantum `ML-KEM-768 + X25519`** and signed with **`ML-DSA-65 + Ed25519`** (`AAD = org_id \|\| project_id \|\| sub_project_id \|\| provider_repo_id \|\| entity_id`). Vault/KMS releases `DEK_A` ONLY to `spiffe://signal/orgA/projA` (after hardware attestation in `CONFIDENTIAL_SILICON` tier). | Defeats both cross-project ciphertext substitution AND long-horizon "Harvest Now, Decrypt Later" quantum adversaries on 20-year chip IP! |
| **Compute, Kernel & GPU (`CONFIDENTIAL_SILICON`)** | **1 Dedicated Pod per Loaded Project** (never reused; destroyed after RAM `zeroize`). Runs inside **Kata / Firecracker MicroVM** or **gVisor**; defense/frontier silicon runs in **`CONFIDENTIAL_SILICON` (AMD SEV-SNP / Intel TDX encrypted-RAM VMs + NVIDIA H100/B200 Confidential Computing GPUs)**. Shared GPUs MUST use **Hardware-Isolated NVIDIA MIG (`1g.10gb`/`2g.20gb`)**—time-sliced multi-project GPUs are forbidden! | Neither a Linux kernel escape, a GPU VRAM residue probe, nor even a malicious cloud host root administrator (`CONFIDENTIAL_SILICON`) can read Project Pod RAM or GPU tensors! |
| **Inside Multi-Repo Project Pod** | Rust `signal-engine` enforces each connected user's signed **per-repo (`provider_repo_id`) Sub-Project RBAC map** (`ev-inverter-pcb: write`, `ev-inverter-ic-analog: none`) on graph queries, viewport layers, Cloud Clipboard pastes, and AI context. | An external PCB contractor connected to Project A's pod receives ZERO nodes, polygons, net names, or clipboard data from Project A's IC or BIOS repos. |
| **Network (Cilium eBPF + Brokered Federation)** | Default-deny Cilium NetworkPolicies: **ALL Pod-to-Pod lateral traffic between Project Pods is 100% DROPPED at kernel eBPF level.** Even `SILICON_SCALE` hierarchical multi-pod projects federate strictly via `ML-DSA`-signed immutable Abstract Views (`*.lef.sig`, `*.lib.sig`) in Git + stateless control-plane `signal-federation-broker`—zero lateral sockets! | Zero lateral scanning, zero east-west movement, zero direct internet exfiltration from inside any Project Pod. |
| **Ephemeral Burst & SFTP Workers** | SPICE/EM/DRC/LVS/Formal/Firmware workers are **single-job, single-project, zero-egress microVM pods**. SFTP manufacturing pushes run inside **disposable single-target write-only `russh` pods** (`SSH_FXP_READ`/`OPENDIR` disabled; atomic `.tmp.part` → SHA-256 verify → `rename`). | Malicious firmware `Makefile`, Verilog testbench, or compromised fab SFTP server cannot read back sibling fab files, pivot laterally, or touch persistent disk. |
| **BYO-Git Storage** | Separate prefixed repositories per Project Type, **entity-chunked convergent `AES-256-GCM-SIV` `ENCRYPTED_VAULT`** with `(kek_id, dek_version)` headers, non-bypassable **`MUST_ENCRYPT` taint tag**, Two-Phase `Super-Commit`, audited periodically by `services/vcs-auditor`. | Compromise or `git clone` of one repo yields unreadable ciphertext without breaking Git deduplication or exposing sibling repos. |
| **AI Agents & Dual-LLM Quarantine** | Agents inherit calling user's exact per-repo RBAC, run **Dual-LLM Quarantine** (zero-tool Reader LLM → `GBNF` JSON → Planner Agent), write strictly to anonymous `jj` / CoW branches, and cannot invoke SFTP/VCS-credential/admin APIs. | Indirect prompt injection hidden in a PDF datasheet, errata note, or commit message is structurally incapable of calling tools or leaking IP. |

---

## 3. Application-Layer Hardening Standards (OWASP ASVS Level 3)

### 3.1 Angular Frontend Hardening (`web/`)
- **Strict Content Security Policy (CSP):** Per-request cryptographic `nonce` for scripts, `object-src 'none'`, `base-uri 'none'`, `frame-ancestors 'none'` (clickjacking immunity), `connect-src` locked strictly to Signal Envoy/TURN origins (`turns:443`, HTTP/3 WebTransport, WSS).
- **XSS Elimination:** Angular **Trusted Types** enforcement enabled globally; `bypassSecurityTrust*` APIs banned by ESLint CI rule; all PRD/HFS/Review Markdown rendered via strict DOMPurify allowlist.
- **Session, Multi-Tab Scoping & CSRF Protection:** Auth tokens live exclusively in `HttpOnly; Secure; SameSite=Strict` cookies; active org/project context is bound explicitly to URL route parameters (`/org/:org_slug/project/:proj_slug`) and gRPC metadata rather than mutable shared cookies; state-changing requests require double-submit CSRF token + `Origin` validation.
- **Subresource Integrity (SRI) & Lockfile Pinning:** All build bundles verified with SRI hashes; zero external third-party CDN scripts or telemetry trackers loaded in the Signal IDE.

### 3.2 Node.js Control Plane, Quorum Governance & Socket-Pinned Anti-SSRF (`services/`)
- **Fail-Closed Authorization Middleware & Uniform `404`:** Every route MUST declare an explicit authorization policy decorator (`@RequireProjectRepoRole`, `@RequireOrgAdmin`, `@RequireStepUp2FA`). Routes missing an explicit policy decorator **fail CI build automatically**. Unauthorized resource lookups return constant-time `404 Not Found`.
- **Strict Input & Schema Validation:** Every request body, query param, and path param validated against build-time-generated OpenAPI/Protobuf schemas with strict byte-length limits, SI-unit integer bounds, Unicode NFC normalization, and unknown-field stripping.
- **Untrusted "Dirty-Hint" Webhook Policy:** Incoming GitHub/GitLab/Bitbucket webhooks validate constant-time `HMAC-SHA256` signatures + 5-minute replay windows, and are treated strictly as **untrusted dirty hints**—`vcs-connector` / `identity-org` ALWAYS re-queries the provider's authoritative HTTPS API over `signal-egress-proxy` before mutating RBAC or evicting pods!
- **Socket-Level Anti-SSRF & Pinned Enterprise PKI (`signal-egress-proxy` & `sftp-egress`):**
  1. Enforce `https://` (Git) or `ssh/sftp` (SFTP) schemes only; disable cross-host HTTP redirects.
  2. **Defeat `TTL=0` DNS Rebinding:** Custom resolver resolves DNS, validates every returned IPv4/IPv6 address against **HARD BLOCKED** CIDRs (`127.0.0.0/8`, `::1`, `169.254.0.0/16`, `fd00:ec2::254`, `100.64.0.0/10`, and RFC1918/K8s pod CIDRs unless bound to an attested Enterprise PrivateLink/VPC endpoint), and **binds the outbound TCP socket directly to the verified IP on EVERY `connect()`** while preserving TLS SNI.
  3. **Custom Enterprise Root CA Pinning:** Self-hosted Git servers using private enterprise PKIs verify against Step-Up 2FA-uploaded Custom Root CA `PEM` bundles pinned by SPKI SHA-256 fingerprint—never disabling TLS verification.
- **Replay-Proof Step-Up 2FA & Multi-Admin Quorum:**
  - Fresh WebAuthn/FIDO2 (preferred) or TOTP assertions (`<= 5 min` TTL) required for BYO-Git credentials, Access Review revocations, SFTP targets/releases, DLP/BYOK edits, and Admin promotions. Every used `(user_id, totp_time_step)` is atomically burned in Redis (`SET NX EX 90`) to defeat same-window TOTP replay!
  - Enterprise orgs require **≥ 2 enrolled hardware-key Org Super Admins** and enforce **2-of-3 M-of-N Multi-Admin Quorum** for Emergency Break-Glass recovery.

### 3.3 Rust Engine, Untrusted File Parser Caps, WASM Sandbox & `SecretBox<T>` (`engine/`)
EDA platforms ingest notoriously complex external files (PDF datasheets, legacy KiCad/Altium/ODB++/IPC-2581 archives in `signal-import`, STEP 3D models, GDSII/OASIS, STDF/VCD silicon telemetry in `signal-twin`, user-authored PCELLs/modules). Signal eliminates memory-corruption and resource-exhaustion surfaces:
- **Pure Safe Rust Core:** `#![forbid(unsafe_code)]` enforced at crate level across `signal-core`, `signal-graph`, `signal-component`, `signal-git-sync`, `signal-pod`, `signal-import`, `signal-twin`, and `signal-operator`. Audited SIMD/GPU FFI leaves require `// SAFETY:` proofs + Miri/fuzz coverage.
- **Hardened Untrusted File Parser Sandboxes & Bomb Caps:** All external uploads execute inside **ephemeral, zero-network, memory-and-CPU-capped microVM/WASM parser sandboxes** enforcing:
  - Max decompression ratio `<= 20:1` and strict unpacked byte ceilings (zip/gzip/zstd bomb immunity).
  - **Zero XML DTD / External Entity (`XXE`) expansion** on IPC-2581, ODB++, SVG, and `.sdf`/`.lib` XML wrappers.
  - Max cell-instantiation DAG depth `<= 64` and cycle rejection on GDSII/OASIS/STEP geometry graphs (exponential DAG bomb immunity).
- **Sandboxed Custom Component Modules (`wasmtime`):** Third-party Verilog-A / Rust `ElectronicModule` plugins and IC PCELLs require valid `ML-DSA` publisher signatures and execute inside **`wasmtime`** with **zero WASI filesystem/network capabilities**, hard linear-memory caps (`<= 256 MB`), and deterministic CPU fuel limits.
- **Mandatory Memory Zeroization & `SecretBox<T>` Telemetry Hygiene:**
  - All decrypted DEKs, Git tokens, PDKs, and sensitive design arenas implement `zeroize::ZeroizeOnDrop`; K8s nodes enforce `swapoff -a` and `RLIMIT_CORE = 0`.
  - Every credential, DEK, customer net name, and confidential design string is wrapped in **`SecretBox<T>`** (Rust) / **`Redacted<T>`** (TypeScript) whose `Debug`, `Display`, and `Serialize` implementations emit `"[REDACTED]"`—guaranteeing zero sensitive IP ever leaks into panic backtraces, OpenTelemetry spans, or structured logs!

---

## 4. AI Agent & Universal Datasheet RAG Security (Dual-LLM & Partitioned RAG Defense)

AI-Agent-First EDA faces sophisticated threats such as **Indirect Prompt Injection** (hidden adversarial text inside an uploaded PDF datasheet, silicon errata, legacy schematic note, or Git commit message) and **Cross-Tenant Vector Side Channels**:

1. **Dual-LLM Quarantine Architecture (Structural Prompt-Injection Immunity):**
   - Untrusted external documents (PDF datasheets, errata, imported CAD notes, commit messages) are **NEVER** fed directly into a tool-calling agent's system/instruction stream.
   - Instead, a **Quarantined Reader Model (with ZERO tool-calling permissions and zero project state)** extracts strictly typed engineering facts enforced by **`GBNF` / JSON-Schema constrained token decoding**. The **Privileged Planner Agent** consumes only validated typed structs/numbers—making prose instructions hidden inside a PDF structurally incapable of triggering tool calls!
2. **Zero Ambient Authority & Hard Tool Allowlist:**
   - Every `AgentToolsService` call carries the **invoking user's scoped, non-elevated per-repo (`provider_repo_id`) RBAC token**.
   - AI Agents have **ZERO tool bindings** to `sftp-egress`, `vcs-connector` credentials, `identity-org` admin APIs, Access Review revocations, or outbound network sockets.
3. **Deterministic Physics & Speculative Approval Boundary:**
   - AI Agents write exclusively to anonymous `jj` / CoW speculative branches verified by deterministic Rust ERC/DRC/SPICE/Formal gates—never mutating `main` or live shared branches without human **Approve & Merge** (with visual schematic/layout diff).
4. **Physically Partitioned Multi-Tenant Datasheet RAG:**
   - Global verified datasheets reside in `datasheet_chunks_global`; each Enterprise tenant's private NDA datasheets/PDKs reside in a **dedicated per-org PostgreSQL partition (`datasheet_chunks_org_<hash>`) with its own physical `pgvector` HNSW index** plus mandatory RLS (`org_id`), preventing both cross-tenant retrieval leaks and HNSW graph-traversal timing inference.
5. **Anti-Exfiltration & Zero Training Guarantee:**
   - Enterprise tenants can mandate **VPC-Only / Confidential-GPU Inference** (vLLM/TensorRT-LLM inside private K8s) with zero external LLM egress and zero training retention.

---

## 5. Secrets Management, Workload Identity & Runtime Intrusion Detection

- **Zero Static Secrets in Code, Env Vars, or Git:**
  - All BYO-Git tokens, GitHub/GitLab/Bitbucket App keys, SFTP keys, and DB credentials live inside **HashiCorp Vault / Cloud HSM** protected by Hybrid PQC envelopes.
  - Workloads authenticate via **SPIFFE / SPIRE** cryptographic X.509 SVIDs (auto-rotated every 60 minutes, plus AMD SEV-SNP / Intel TDX hardware attestation quotes for `CONFIDENTIAL_SILICON` pods) and receive lease-bound, project-scoped credentials directly in `SecretBox` RAM.
- **Runtime eBPF Security Monitoring (Cilium Tetragon / Falco):**
  - Monitors every `signal-project-pod`, parser sandbox, and burst worker at the kernel eBPF level for impossible behaviors: spawning shells (`/bin/sh`), writing outside RAM `tmpfs`, opening unbrokered sockets, or touching `/proc` namespaces.
  - **Automated Self-Defense:** Any violation triggers **Instant Pod Quarantine** (`< 1 s` network freeze + pod termination, SPIFFE SVID & KMS lease revocation, scrubbed-RAM audit snapshot, and immediate SIEM alert).

---

## 6. Secure SDLC, Supply-Chain Integrity & Automated Security Gates

Security is verified automatically on **every single PR** (authored by **Antigravity**, **Cursor**, or human engineers):

| Gate | Tooling / Enforcement | Fails Build If... |
|---|---|---|
| **Static Security Analysis (SAST)** | `cargo clippy`, `Semgrep` (Signal custom rules), `CodeQL`, TypeScript strict checks | Any unapproved `unsafe`, banned sinks (`eval`, `child_process.exec`, `bypassSecurityTrust*`), raw unredacted secret logging bypassing `SecretBox<T>`, missing `@Require*` route decorator, non-uniform `403` enumeration leak, or `localStorage`/`IndexedDB` design persistence! |
| **Supply-Chain & Crate-Diff Vetting** | **`cargo vet`** (cryptographic human audit of third-party crate diffs), `cargo-deny`, `cargo-audit`, `pnpm audit` (`ignore-scripts=true`), CycloneDX SBOM | Any unvetted crate version diff, Critical/High CVE, disallowed SPDX license, lifecycle install script, or unpinned dependency! |
| **Build-Time Contract & SI-Unit Check** | `buf lint`, `buf breaking`, cross-language conformance fixture tests (`contracts/fixtures/`) | Hand-edited generated stubs committed to Git, un-suffixed physical units, or `i64`/`f64` serialization drift between Rust and TypeScript. |
| **Container, MicroVM & GPU Admission** | **SLSA Level 3** provenance + **Sigstore Cosign** (`ML-DSA`/ECDSA) + **Kyverno / OPA Gatekeeper** | Unsigned image, root user, writable rootfs, time-sliced non-MIG multi-tenant GPU profile, missing MicroVM/SEV-SNP runtimeClass, or missing Cilium default-deny policy. |
| **Production Guardrail CI Check** | Automated binary symbol & bundle verification | `dev-guest-auth` feature flag or `SIGNAL_ALLOW_LOCAL_GUEST` exists in any staging/production binary, OR any `/download`/`/export`/`/print` design endpoint is registered. |
| **Continuous Fuzzing & Isolation Suite** | `cargo-fuzz` (parsers/geometry/MNA) + Cross-Project Isolation chaos suite + semi-annual external red-team pen-tests | Any parser panic/OOM on malformed PDF/GDSII/STEP/ODB++, or any cross-project SQL/NATS/HNSW/eBPF/clipboard isolation breach! |

---

## 7. Incident Response, Cryptographic Kill Switches & GDPR-Safe WORM Forensics

Signal builds **four instant kill switches** directly into the Control Plane and KMS layer:
1. **User Session Eviction (`< 2 s`):** Revoking a user in GitHub/GitLab/Bitbucket or Signal Access Review immediately kills their JWT, terminates active WebTransport/WebRTC/WebSocket streams, and evicts them from all Project Pods.
2. **Project Cell Freeze (`< 5 s`):** Security Officer freezes a compromised Project → `signal-operator` revokes its K8s Lease, terminates its Project Pod, scrubs RAM, locks new sessions, and halts all SFTP releases.
3. **Enterprise BYOK / HYOK Cryptographic Kill Switch (`< 10 s`):** Enterprise revokes their customer-managed KMS/HSM key → every active pod loses decryption authority and terminates immediately, rendering all `ENCRYPTED_VAULT` chunks and continuity journals mathematically inaccessible.
4. **GDPR-Compliant Pseudonymous WORM Merkle Audit Reconstruction:** All identity, RBAC, Access Review, Pod Lease, `Super-Commit`, AI merge, DLP watermark, Tetragon/Falco, and 21 CFR Part 11 / SFTP events append to **immutable WORM storage anchored hourly to an external RFC-3161 Trusted Timestamping Authority (TSA)** and streamed to Enterprise SIEMs (Splunk, Sentinel, Datadog, Chronicle). To satisfy **GDPR Article 17 Right-to-Erasure** without ever mutating WORM hash chains, audit entries record pseudonymous `actor_pseudo_id = HMAC-SHA256(org_audit_salt, provider_user_id)` while storing the reversible PII mapping in an erasable identity vault table!
