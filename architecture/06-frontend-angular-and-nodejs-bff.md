# 06 — Frontend (Angular) & Node.js Platform BFF Architecture

> **Scope:** Defines the web application architecture for Signal—built with **Angular** on the frontend and **Node.js (TypeScript)** for the Backend-for-Frontend (BFF) and SaaS Control Plane—delivering desktop-grade EDA ergonomics with zero local design files.

---

## 1. Frontend Architecture (Angular 19+ Web IDE)

Signal's UI must feel as fast, keyboard-driven, and precise as a native desktop EDA suite while running entirely inside a browser with zero local file persistence.

### 1.1 Architectural Principles
- **Modern Angular Core:** Standalone Components, **Angular Signals** for fine-grained reactive UI state (zero heavy zone-thrashing during 60 FPS cursor/probe updates), strict TypeScript (`strict: true`), and lazy-loaded domain workbench modules.
- **Zero-Persistence Client Rule:** Design data, schematics, netlists, layouts, waveforms, and specs are **NEVER** written to `localStorage`, `sessionStorage`, `IndexedDB`, Service Worker caches, or downloadable Blob URLs. Only non-sensitive UI preferences (dark/light theme, panel layout geometry, keyboard bindings) may persist.
- **Unified Multi-Domain IDE Shell:** One cohesive workspace shell hosting every product discipline without switching apps:
  - **Left Dock:** Unified Project Explorer (PRD, HFS, Schematics, PCB, IC, FPGA, Firmware, BIOS, Simulations, Releases), Component/Datasheet Browser, Net/Hierarchy Navigator, Layer Manager.
  - **Center Multi-Tab Dockable Canvas:** Split-screen cross-probing (e.g., Schematic on left + PCB 2D/3D on right + Live Waveform on bottom + Firmware pinmux code side-by-side—clicking a net highlights it across ALL open panes simultaneously!).
  - **Right Dock:** Context-sensitive Property & Rust Module Parameter Inspector (tolerances, TCR, parasitics, SOA limits) + **AI Agent Copilot Panel** (chat, speculative diff reviewer, autonomous verification progress, datasheet citations).
  - **Bottom Dock:** Real-time ERC/DRC/LVS/SOA Diagnostics Console, Simulation Output, Traceability Matrix, and Terminal/Build logs (read-only streamed from cloud worker pods).

```mermaid
flowchart TB
    subgraph Shell["Angular IDE Shell (apps/signal-web)"]
        TopBar["Command Palette (Cmd+K), Collaborator Avatars, Project/Rev Switcher, SFTP Release Gate"]
        Left["Unified Project Tree & Datasheet RAG Search"]
        Right["AI Agent Copilot & Rust Module Property Inspector"]
        Bottom["Live ERC/DRC/SOA Diagnostics & Traceability Matrix"]

        subgraph Workbenches["Lazy-Loaded Domain Workbenches (libs/*)"]
            W1["Schematic & Block Diagram Canvas"]
            W2["PCB 2D/3D, Harness & Stackup Workbench"]
            W3["IC Analog/Digital Mask & 3D-IC Workbench"]
            W4["GPU Waveform, Digital Twin & PINN Studio"]
            W5["PRD & HFS Structured Spec Editor"]
            W6["FPGA, Firmware & BIOS Code IDE (Monaco)"]
            W7["Visual Semantic Diff & Spatial Review Redlining"]
            W8["Admin, Access Review, BYO-Git Security & 2FA SFTP Console"]
        end
    end

    subgraph Transport["Zero-Trust Client Transport Layer"]
        REST["Typed REST/OpenAPI Client (URL-Scoped /org/:org_slug + Idempotency-Key)"]
        WSS["Envoy WebTransport / WSS Multiplexer + Volatile <=60s RAM Reconnect Buffer"]
        Viewport["Composited GPU Atlas (YUV 4:4:4 Lossless Tiles + Motion Stream / WebGPU)"]
        DLP["Client DLP Guard (No-Print, Watermark Overlay, Focus Curtain, Cloud Clipboard)"]
    end

    Workbenches <--> Transport
    DLP -.-> Shell
```

### 1.2 Dual-Mode High-Performance Viewport & Firewall-Resilient Transport

To satisfy both **extreme Enterprise Zero-Leak security** and **razor-sharp, 60–120 FPS desktop CAD precision**, Signal's viewport adapter supports two organization-selectable policies:

| Aspect | Mode A: Enterprise Zero-Leak Dual-Plane Stream (Default for Enterprise) | Mode B: Standard Ephemeral WebGPU (Local Dev / `EXPLORER` / `MAKER`) |
|---|---|---|
| **Where Rendering Happens** | Server-side GPU (`signal-render` `wgpu` inside the Project MicroVM) compositing all open split-screen panes into **one unified Multi-Pane GPU Atlas** (preventing encoder session exhaustion!). | Browser GPU (`WebGPU` / `WebGL2`) rendering ephemeral viewport-clipped display primitives (or ultra-compact binary vector deltas on 3G links). |
| **CAD Sharpness & Battery-Safe Decode (`Zstd-QOI` + HW Video)** | **Zero Chroma Blur & Zero Software-AV1 CPU Drain:** **Plane 1** transmits **`Zstd + QOI` (`> 400 MB/s` GPU shader decode on 100% of browsers including M1/M2 Safari & Chromebooks!) / Lossless `WebP` `RGBA8` dirty-rect tiles** so 1px wires and pin labels are pixel-perfect; **Plane 2** probes `WebCodecs` hardware support (`HEVC 4:4:4` on Mac → **Dual-Luma Hardware `AV1/H.264 4:2:0` Plane Split** on NVDEC/Intel GPUs) *only* during continuous 3D orbit/pan, snapping to Plane 1 within **`80 ms`** of motion stop! | Native crisp local vector/mesh rasterization. |
| **Firewall Traversal & Reconnect** | Automatic **HTTP/3 WebTransport (Baseline including Safari 26.4+) ↔ TURN-over-TLS 443 (`turns:`) ↔ HTTP/2 WSS Binary Tile Fallback** (100% UDP-blocked corporate proxy traversal!). Uncommitted `GraphOp`s buffer in a **volatile JS RAM ring-buffer (`<= 60 s`, zero disk/IndexedDB!)** to survive Wi-Fi micro-drops seamlessly. | WebTransport / WSS with identical volatile RAM reconnect buffer. |
| **Input Latency & Memory Caps** | **120 FPS client-side predictive crosshair, selection box, pan matrix & ghost-wire overlay** + V8 heap protection (`cdk-virtual-scroll` trees, windowed Monaco buffers, `<= 4,096`-point decimated waveform envelopes, ES2020 `bigint` for 64-bit `_nm`/`_ps` coordinates). | Native 60–120 FPS local pan/zoom with server-authoritative edits. |

### 1.3 Accessibility, Cross-Origin Isolation & Workbench Highlights
- **Route-Partitioned `COOP`/`COEP` & OAuth/SAML-Safe Cookie Upgrade:** Workspace routes (`/org/:org_slug/project/*`) enforce `Cross-Origin-Opener-Policy: same-origin` + `Cross-Origin-Embedder-Policy: require-corp` (`crossOriginIsolated = true` enabling high-res timers & `SharedArrayBuffer` workers), whereas top-level `/auth/*` PKCE/SAML redirect routes use `5 min` `SameSite=Lax` state cookies immediately upgraded on `/auth/callback` into a host-locked `__Host-signal_session` (`HttpOnly; Secure; SameSite=Strict`) cookie + `Sec-Fetch-Site` guards—eliminating `window.opener = null` popup and cross-site IdP redirect breakages!
- **Accessibility & Engineer Ergonomics:** Built-in **Color-Blind-Safe Okabe-Ito / High-Contrast Palettes + Procedural Layer Stipple/Hatch Patterns**, switchable **KiCad / Altium / Virtuoso / Vim single-key hotkey maps**, CLI coordinate bar, **unambiguous SI unit confirmation pills (`10 mΩ` vs `10 MΩ`)**, and **Server-Side Encrypted Cloud Clipboard**.
- **Visual Semantic Diff & Spatial Review (`libs/visual-diff`, `libs/review`):** Side-by-side / onion-skin graphical PR & ECO diffs across schematics, PCBs, waveforms, and register maps + **pin/coordinate-anchored review threads & live audio/cursor huddles**.
- **PRD/HFS (`libs/specs`), Waveform/Digital-Twin Studio (`libs/sim-viewer`), Embedded Code/HDL IDE (`libs/code-ide`), and Admin/Access-Review/VCS-Security Console (`libs/admin`):** All lazy-loaded inside the same unified shell with **Progressive UI Profiles (`EXPLORER` / `MAKER` / `PRO` / `SILICON_SCALE`)**.

### 1.4 Extended Angular 19+ Workbench Architecture & UI Primitives (`FEAT-001`–`FEAT-120`)
Full dual UI+Rust feature mappings live in [`17-extended-feature-specifications-ui-and-rust.md`](./17-extended-feature-specifications-ui-and-rust.md). To scale cleanly across **120+ advanced capabilities** without bloating initial bundle size (`< 250 KB` gzipped initial shell), Angular organizes the frontend around **10 additional lazy-loaded domain workbench libraries** and **4 shared interactive UI primitives**:
- **Extended Lazy-Loaded Workbench Libraries (`web/libs/*`):**
  1. `libs/magnetics` — Bobbin/planar winding cross-section builder, core catalog picker, and `B_pk`/thermal rise gauges (`FEAT-021`).
  2. `libs/power-studio` — PDN `Z_target(f)` waterfall, 3D geodesic creepage inspector, DC IR-drop/hot-loop overlays, interactive Bode pole-zero dragger, BMS pack matrix, FOC phasor scope, thermal airflow studio, and power-sequencing Gantt (`FEAT-022`–`030`).
  3. `libs/emc-lab` & `libs/safety-wcca` — `CISPR`/`FCC` regulatory spectrum studio, EMI filter Middlebrook designer, ESD/surge click-injector, shield-can leakage map, ISO 26262 FMEDA matrix, interactive Fault-Tree canvas, WCCA margin studio, Rad-Hard pulse overlay, MLCC piezo-singing overlay, and org derating scoreboard (`FEAT-031`–`040`).
  4. `libs/fpga-studio` & `libs/bringup-lab` — Zoomable FPGA SLR/bank/SSO die floorplanner, SDC clock-matrix wizard, graphical FSM & fixed-point DSP editors, interactive Shmoo-plot margin explorer, boot-sequence/HIL fault runners, and zero-export factory board-map debugger (`FEAT-051`–`060`, `FEAT-070`, `FEAT-088`).
  5. `libs/harness-3d` & `libs/dfm-panel` — Multi-board 3D STEP mate + rigid-flex 0–100% bend-fold slider, 1:1 wire-harness nailboard canvas, multi-up panel composer with MLCC V-score flex-crack halos, DFA reflow/tombstone/coating tools, live GD&T drawing studio, should-cost/yield HUD pill, and ICT test-point coverage map (`FEAT-020`, `FEAT-061`–`069`).
  6. `libs/library-studio` & `libs/academy` — Universal 3-Tier Part/Datasheet/Master-Pin/Symbol/Padstack/IPC-7351 Footprint+3D STEP studio (`18-*`), multi-unit symbol splitter, nonlinear curve-fitter, Monaco Rust/WASM component sandbox, Golden Subcircuit snippet vault, Breadboard↔Schematic↔PCB twin, auto-graded classroom labs, safe visual smoke/over-current FX, resistor/SMD quick-calculators, and `TinyTapeout` wizard (`FEAT-071`–`080`, `FEAT-111`–`115`).
  7. `libs/mcad-studio`, `libs/cam-print` & `libs/plm-ops` (`19-*`) — Parametric 2D constraint sketcher + 3D B-Rep/sheet-metal enclosure modeler synced live to PCB anchors (`mcad-studio`); 2.5D/3/5-axis CNC CAM toolpath/gouge simulator & direct-B-Rep FDM/SLA/SLS/DMLS 3D-print build-plate slicer (`cam-print`); and native multi-view `eBOM`/`mBOM`/`sBOM` tree-grid, `ECR`/`ECO`/`ECN`, `MCN`, `PCN`, qty/time-bounded `Deviation`/`Waiver`, 8D `NCR`/`CAPA`, Clear-to-Build (`CTB`) shortage cockpit, and interactive 3D factory work instructions (`plm-ops`).
- **Four Shared High-Precision UI Primitives (`apps/signal-web`):**
  1. **Zero-Leak `HudOverlayPrimitive` Layer:** Renders crisp DOM/Canvas2D/SVG interactive overlays (on-canvas potentiometers/switches, cursor-tracking meander/skew gauges, KaTeX math blocks, laser pointers, protocol waveform bubbles, TDR trace reticles) aligned via `bigint` nanometer screen-to-world projections over `signal-render`'s `Zstd-QOI`/WebP GPU atlas—never holding raw IP polygons in browser JS memory under Enterprise Zero-Leak mode!
  2. **Omni-Command Palette (`Cmd+K`), Radial Pie-Menu, SpaceMouse & In-App "What's New" Release Drawer (`20-*`):** Instant fuzzy search + typed parametric CLI commands (`FEAT-091`), 8-slice radial wheel (`FEAT-093`), `X`-key X-Ray layer/net focus (`FEAT-096`), topological query bulk editor (`FEAT-099`), plus automated In-App Release Notes (`GET /v1/platform/releases`) and non-disruptive **"Save & Reload"** prompts backed by `N-3` hashed chunk retention!
  3. **Zero-Leak Multi-Monitor Pop-Out & Live Review Deck:** Same-origin `BroadcastChannel` + `SharedWorker` coordinates undocked Waveform/3D/IDE monitors (`FEAT-092`) and interactive zero-export Design-Review slide decks (`FEAT-109`, `FEAT-119`) inheriting identical focus curtains, `COOP`/`COEP` isolation, and watermark streams.
  4. **WCAG 2.2 AA Keyboard-Only CAD & Live Telemetry Signals:** Full keyboard schematic wiring/nudging, ARIA netlist connectivity announcements (`FEAT-100`), virtual bench instrument skins (`FEAT-081`), what-if parameter scrubbers (`FEAT-085`), and multi-objective AI Pareto scatter clouds (`FEAT-103`).

---

## 2. Node.js Control Plane, Dedicated API Bridge & 3-Tier K8s Separation (`services/`)

Per [`20-versioning-release-monorepo-build-and-k8s-topology.md`](./20-versioning-release-monorepo-build-and-k8s-topology.md), Signal physically separates the **Angular + Node Web Pod (`pool-web`)**, the **Dedicated HA API & Engine Bridge Tier (`pool-api`)**, and the **`1–4` Baseline (Extensible to `N`) Rust Engine Worker Nodes (`pool-rust-engine`)**. Node.js (Node 22 LTS, strict TypeScript, Fastify/NestJS) owns SaaS control-plane orchestration, while **Envoy (`signal-engine-bridge`)** handles zero-copy high-rate binary viewport/pointer pass-through over SPIFFE mTLS (eliminating Node.js JSON/GC jitter on 120 Hz CAD streams).

### 2.1 Service Decomposition

| Service | Primary Responsibilities | Backing Stores |
|---|---|---|
| **`api-gateway` + `signal-engine-bridge` (Envoy)** | Hybrid PQC TLS 1.3, CSP nonces, route-partitioned `COOP`/`COEP`, `Cache-Control: no-store`, CSRF/`Sec-Fetch-*` guards, rate limiting, `@Require*` uniform-`404` decorators, Platform Version & Automated Release Notes feed (`/v1/platform/releases`), and zero-copy Ed25519-ticketed WebTransport/`turns:443`/WSS stream relay to multi-version Rust Engine Pods. | Redis |
| **`identity-org`** | Top-level redirect OAuth 2.0 PKCE (`GitHub`/`GitLab`/`Bitbucket`) + Enterprise SAML/OIDC dual-identity binding, approval-gated Enterprise onboarding, WebAuthn/TOTP Step-Up 2FA (Redis replay burn-list), ≥2 Super Admin 2-of-3 Quorum break-glass, and **Org & Project Admin Access Review Console** (with contractor `expires_at`). | PostgreSQL (RLS), Redis |
| **`project-catalog`** | Multi-repo Project & Sub-Project CRUD (including `CAD_MCAD`, `CAM_CNC`, `PRINT_3D`, `PLM_OPS`), extensible metadata, per-project LTS engine version pinning (`platform.lock.json`), **Signed Fab Capability Profiles**, **SPDX license compatibility gate**, Project Templates, `signal-federation-broker`, and In-Jurisdiction Paired-Region pinning. | PostgreSQL (RLS) |
| **`parts-registry` (`18-*`)** | **3-Tier Universal Component & Datasheet Registry** (`signal-universal-parts` Sharded Global Git + BLAKE3 CAS gateway, Encrypted Org-Private BYO-Git `<org>/signal-org-parts` AVL overlays, Project `parts.lock.json` pinning, and cross-project Where-Used scans). | PostgreSQL (RLS), BLAKE3 CAS, Redis |
| **`plm-ops` (`19-*`)** | **Native PLM, Change-Control & Supply-Chain Operations:** Multi-view `eBOM`/`mBOM`/`sBOM`/`Costed BOM`, Step-Up 2FA-signed `ECR`/`ECO`/`ECN`, `MCN`, `PCN`/`PDN` triage, hard qty/time-gated `Deviation`/`Waiver`, 8D `NCR`/`CAPA`, Clear-to-Build (`CTB`) shortage analytics, 3D factory work instructions, and `signal-egress-proxy` ERP/MES webhooks. | PostgreSQL (RLS), NATS, Redis |
| **`vcs-connector`** | **2FA-gated BYO-Git settings** (`GitProviderAdapter` + custom Root CA pinning), **lazy just-in-time `<project>-<type>` repo materialization + `<= 40/min` secondary-rate-limit pacer**, GraphQL/`ETag` roster sync, and HMAC/replay-guarded dirty-hint webhooks. | PostgreSQL (RLS), Vault/HSM, Redis |
| **`vcs-auditor`** | Scheduled (6h/daily) + webhook + pre-release **Multi-Provider Security Posture & Access-Drift Auditor** (public drift, rename drift, unreviewed users, overdue contractors, 2FA/SSO, Secret Scanning, CVEs, Posture Score `0–100`, 2FA remediation). | PostgreSQL (RLS), Vault/HSM, NATS |
| **`session-broker`** | Singleflight Host-Lease/Epoch + Redis sticky cell lookup across `1–4 -> N` extensible Rust worker nodes, attested device-posture check (`HIGH_SECURITY`), Ed25519 stream ticket issuance (`60 s` rolling heartbeat), instant user eviction, and **RBAC-verified cross-project encrypted cloud clipboard**. | Redis, K8s Operator gRPC, Project Pod gRPC |
| **`sftp-egress`** | **2FA-gated Authorized SFTP registry**, host-key pinning, socket-level anti-SSRF IP pinning, HSM `ED25519` keys, 4-eyes / 21 CFR Part 11 + active `Deviation` gate check, and disposable `russh` atomic uploads (PCB/IC/MCAD STEP/DXF/CNC G-code/3MF/ECN/MCN/turnkey CM bundles). | PostgreSQL (RLS), Vault/HSM, NATS |
| **`audit-service`** | **Fail-Closed Synchronous Quorum Outbox for Tier-0 Security Events** (`503` if unlogged!) + Pseudonymous (GDPR crypto-shreddable PII) **RFC 3161 + `ML-DSA-65` Merkle-Tree Transparency Log** committed to S3 Object Lock (`COMPLIANCE` WORM) and SIEMs. | 3-Replica JetStream/Postgres Outbox, WORM S3 |

### 2.2 Why 3-Tier Node.js + Envoy Bridge + Angular + Extensible Rust Nodes Is the Optimal Split
- **Shared `bigint`-Safe TypeScript Contracts:** Frontend (`web/`) and Control Plane (`services/`) share build-time generated TypeScript types (`longAsBigint: true`) and stateful protocol transcripts (`contracts/transcripts/`) with `engine/`.
- **Zero Physics Logic, Zero Web-to-Rust Direct Path & Zero Stream Bottleneck:** `signal-web` (`pool-web`) talks ONLY to `signal-api-gateway`; Rust (`pool-rust-engine`, `1–4` baseline extensible to `N`) owns 100% of physics, CAD/CAM, slicing, PLM compilation, and rendering; `signal-engine-bridge` (`pool-api`) relays binary viewport streams zero-copy; Node.js stays horizontally stateless, fast, and auditable.
