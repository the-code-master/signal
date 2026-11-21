# 05 — AI-Agent-First Platform & Universal Datasheet RAG

> **Scope:** Defines Signal's multi-agent orchestration engine, deterministic closed-loop verification against the Rust physics core, the **Universal Datasheet Database & Hybrid RAG** pipeline, and the GPU/LLM routing layer (supporting Enterprise VPC GPUs, cloud frontier models, and **local Mac LLM default** for development).

---

## 1. AI-Agent-First Engineering Paradigm

Most "AI in CAD" tools fail because LLMs cannot reliably do raw electrical math, recall pinouts from memory without hallucinating, or keep schematics/layouts geometrically consistent.

**Signal solves this with a Grounded, Tool-Native, Closed-Loop Architecture:**
1. **Never Trust LLM Memory for Component Specs:** Every part number, pinout, absolute maximum rating, thermal impedance, and application circuit is retrieved from Signal's **Universal Datasheet Database via Hybrid RAG** with exact page/table citations.
2. **Never Trust LLM Math for Circuit Verification:** Every generated circuit, trace impedance, timing budget, or transistor sizing is executed through the **Rust `signal-engine`** (SPICE, ERC, DRC, LVS, Thermal, SI/PI). Physics is the final arbiter.
3. **Never Mutate Live User Work Unsafely:** Agents operate on zero-copy **in-memory speculative branches** inside the Project Pod, iterate autonomously until all PRD/HFS specs and DRC/SOA rules pass, and present reviewable visual diffs with simulation proof waveforms.

```mermaid
flowchart TB
    User["Engineer Prompt / Canvas Selection / Quiescent Guardian"] --> Sup["Supervisor Agent (K-Hop Cone Slicer & Budget Guard)"]

    subgraph Specialists["Neuro-Symbolic Domain Specialist Agents"]
        A1["PRD, HFS & Whiteboard Agent"]
        A2["Schematic & Adjoint Synthesis Agent"]
        A3["Simulation, Formal & Aging Agent"]
        A4["Multimodal PCB, 3D EM & SI/PI Agent"]
        A5["IC, Analog Sizing & FPGA Agent"]
        A6["Firmware, BIOS & Bring-Up Agent"]
        A7["BOM, Supply-Chain & Sustainability Agent"]
    end

    Sup <--> Specialists

    subgraph Grounding["Dual-LLM Quarantine, Partitioned RAG & Adjoint Physics"]
        Quar["Quarantined Reader LLM (Zero Tools -> GBNF Facts)"]
        RAG["Partitioned Hybrid RAG (SQL + Per-Org pgvector HNSW + Graph)"]
        DSDB["Dual-VLM Universal Datasheet, Errata & Supply-Chain DB"]
        Tools["Typed Protobuf Engine Tool Bridge (gRPC)"]
        Branch["Speculative jj CoW Branch inside Rust Project Pod"]
        Sim["Rust Adjoint signal-diff-sim / SPICE / Formal / 3D EM / DRC"]
    end

    subgraph Inference["Grammar-Constrained (GBNF) LLM & GPU Gateway"]
        Router["signal-llm-gateway"]
        Local["Local Dev Default: Mac Ollama / MLX / llama.cpp"]
        VPC["Enterprise Private: Confidential GPU vLLM / TensorRT-LLM"]
        Cloud["Zero-Retention Cloud Frontier APIs"]
    end

    Specialists <--> RAG
    RAG <--> Quar
    Quar <--> DSDB
    Specialists <--> Tools
    Tools <--> Branch
    Branch <--> Sim
    Sim -->|"Exact Gradients, Proofs, Waveforms & Violations"| Specialists
    Sup <--> Router
    Specialists <--> Router
    Quar --> Router
    Router --> Local
    Router --> VPC
    Router --> Cloud
```

---

## 2. Multi-Agent Team & Capabilities

| Agent | Responsibilities | Core Rust Engine & RAG Tools Used |
|---|---|---|
| **Supervisor / Planner Agent** | Decomposes goals across domains, enforces org guardrails & **hard Token/Iteration/Wall-Clock budgets**, performs **K-Hop Cone-of-Influence graph slicing** (`< 16k` prompt tokens even on 100B-transistor chips), and compiles **Immutable AI Design Proof Bundles**. | `read_cone_slice`, `query_requirements`, `spawn_cow_branch`, `request_human_approval` |
| **PRD, HFS & Whiteboard Agent** | Drafts/audits requirements, parses whiteboard sketches (`signal-import`), builds UPF power trees, clock trees, pin budgets, and interface matrices; flags contradictory specs. | `create_requirement`, `link_traceability`, `build_upf_power_tree`, `ingest_whiteboard_sketch` |
| **Schematic & Neuro-Symbolic Synthesis Agent** | Selects `SILVER`/`GOLD`-verified, in-stock components via Hybrid RAG (co-checking Silicon Errata!), wires Rust `ElectronicModule`s, and delegates numerical component sizing to **`signal-diff-sim` Adjoint / Bayesian Pareto Optimizer** (eliminating LLM trial-and-error oscillation!). | `rag_parametric_parts`, `place_component_module`, `connect_ports`, `run_adjoint_pareto_opt`, `run_erc` |
| **Simulation, Formal & Aging Agent** | Runs DC/AC/Transient/Noise/Monte-Carlo/PVT/Thermal + **15-yr BTI/HCI/EM Aging (`signal-aging`)** + **Formal SVA/LEC proofs (`signal-formal`)** + `< 50 ms` PINN previews (`signal-surrogate`). | `configure_sim`, `run_simulation`, `run_formal_proof`, `check_soa_and_aging`, `correlate_digital_twin` |
| **Multimodal PCB, 3D EM & SI/PI Agent** | Combines **Vision-Canvas top/bottom/thermal snapshots** with deterministic Rust **Analytical Electro-Thermal Placer & Negotiated-Congestion Router** tools, 3D EM breakout checks (`signal-em3d`), and DRC/DFM auto-healing. | `inspect_viewport_vision`, `run_analytical_placer`, `route_critical_nets`, `run_em3d_check`, `run_drc_dfm` |
| **IC, Analog Sizing & FPGA Agent** | Performs `gm/Id` + **Adjoint Differentiable `(W/L)` sizing** across PVT/aging corners, evaluates WASM PCELLs, authors SystemVerilog/VHDL + formal SVA, verifies UPF power islands, and analyzes MMMC STA/DRC/LVS. | `size_transistors_adjoint`, `run_corner_sweep`, `generate_rtl_and_sva`, `sync_fpga_pinmux`, `run_lvs_drc_pex` |
| **Firmware, BIOS & Bring-Up Agent** | Generates pinmux configs, C/C++/Rust HALs, SystemRDL CSRs, Linux `.dts`, UEFI/ACPI tables, factory JTAG/POST diagnostics, and runs **3-Speed Variable-Fidelity Co-Sim** (`100+ MIPS` ISS → RTL → SPICE). | `read_netlist_pins`, `read_register_map`, `generate_code_patch`, `run_3speed_fw_cosim` |
| **BOM, Supply-Chain & Sustainability Agent** | Queries **k-anonymous live distributor stock/lead-time/pricing**, flags PCN/EOL & geo-fab risks, computes **Embodied Carbon ($\text{kgCO}_2\text{e}$) & PFAS/RoHS Digital Product Passports**, executes **Zero-Spin BOM Rescue**, and prepares 2FA SFTP checklists. | `audit_bom_supply_chain`, `find_dropin_alternates`, `compute_carbon_passport`, `prepare_sftp_bundle` |

---

## 3. Closed-Loop Neuro-Symbolic Design Flow (Topology by LLM + Sizing by Adjoint Physics)

```mermaid
sequenceDiagram
    participant Eng as Engineer
    participant Agent as Schematic & Sim Agents
    participant RAG as Verified Datasheet + Supply-Chain RAG
    participant Pod as Rust Project Pod (Live-Rebasing CoW Branch)

    Eng->>Agent: "Design 12V->3.3V 3A Buck, <15mV ripple, >90% eff, -40..125C, 10yr automotive"
    Agent->>RAG: Query (Vin>=18V, Iout>=4A, AEC-Q100, Trust>=SILVER, In-Stock) + Silicon Errata
    RAG-->>Agent: Top 3 Orderable MPNs (with exact Package Pinout + Errata + Digitized Curves)
    Agent->>Pod: Spawn Port-Encapsulated CoW Branch "ai/buck-3v3" (Live-Rebasing)
    Agent->>Pod: Wire Buck IC + Saturating Inductor + DC-Bias MLCCs + Type-III Comp Network
    Agent->>Pod: Invoke signal-diff-sim Adjoint Pareto Optimizer (200 gradient steps in 40ms!)
    Pod-->>Agent: Optimal L/C/R_comp Values Found -> Run Full SPICE + 100-Run Monte Carlo + 10yr Aging
    Pod-->>Agent: Worst-Case Ripple = 10.8mV (PASS), PM = 63 deg (PASS), Eff = 93.1% (PASS), 0 SOA/DRC Errors
    Agent-->>Eng: Present Visual Semantic Diff + Immutable Proof Bundle (Equations, Plots, Page/BBox Citations)
    Eng->>Pod: Click "Approve & Merge" -> Committed Atomically to Shared Project Graph
```

---

## 4. Universal Datasheet Database & Partitioned Hybrid RAG Architecture

### 4.1 High-Assurance Multi-Modal Ingestion & Trust Tiers (`ai/datasheet-ingest`)
Single-pass PDF OCR can misread `1.8 V` as `18 V` or swap QFN pad numbers—destroying hardware. Signal enforces a **Zero-Hallucination High-Assurance Ingestion Pipeline** inside zero-network parser sandboxes:

1. **Orderable MPN + Package + Die-Revision Normalization:** Every record is keyed by exact **Orderable MPN + Package Code (`LQFP48` vs `TFBGA100`) + Silicon Revision**, automatically co-linking **Manufacturer Silicon Errata PDFs, PCNs, and App Notes**.
2. **Dual-VLM Consensus + Physics Sanity Verification:**
   - Two independent layout/Vision-Language extractors parse **AbsMax Ratings**, **Electrical/Thermal Tables**, **Pinout Maps**, **SystemRDL Register Bitfields**, and **Digitized Curves** (`MLCC DC-bias`, `Rds_on vs Tj`, `SOA`). Any disagreement quarantines the field for steward review!
   - Automated **3-Way Symbol ↔ IPC-7351 Footprint ↔ 3D STEP Pin Intersection Check** + SPICE sanity testbench verifies that simulated module curves match extracted datasheet tables within tolerance.
3. **Cryptographic Datasheet Trust Badges (`VerificationBadge`):**
   - `BRONZE_PARSED`: Automated dual-extractor consensus passed; unverified SPICE curve.
   - `SILVER_SIM_MATCHED`: Automated physics, pin-pad 3D check, and SPICE curve-fit verified (`< 2%` RMS error).
   - `GOLD_ATTESTED`: Manufacturer-signed feed or human librarian sign-off (Enterprise templates can mandate `SILVER`/`GOLD` parts only!).

### 4.2 Partitioned Three-Tier Hybrid RAG + Live Supply-Chain (`ai/rag-service`)

| Tier | Store & Isolation | Role in Retrieval |
|---|---|---|
| **1. Structured Parametric & Supply-Chain Index** | PostgreSQL (`NOBYPASSRLS`) + **k-Anonymous Hashed Prefix Supply-Chain Cache** | Deterministic SI-normalized filtering on `V/I/R/Package/Temp/AEC-Q/TrustTier/AVL` + live stock, lead-time, pricing, and embodied-carbon/PFAS metrics without leaking customer BOMs to distributors! |
| **2. Partitioned Dense + Sparse Semantic Index** | **Physically Partitioned `pgvector` HNSW Indexes** (`GLOBAL_VERIFIED` partition vs. per-org `ORG_PRIVATE_NDA` partitions + `hnsw.iterative_scan`) + BM25 | Eliminates RLS ANN post-filter recall collapse and cross-tenant timing side channels while retrieving application notes, layout rules, and **Silicon Errata**. |
| **3. Component & Equivalence Knowledge Graph** | Relational/Graph adjacency tables | Pin-compatible drop-in alternates (**Zero-Spin BOM Rescue**), complementary gate-driver/FET pairs, and verified reference subcircuits. |

---

## 5. Grammar-Constrained & Quarantine-Hardened LLM Gateway (`ai/llm-gateway`)

| Capability Tier | Purpose | Production Cloud / Enterprise VPC | Local Mac Dev Default |
|---|---|---|---|
| **Tier 1: Fast / Quarantine Reader** | Autocomplete, ERC explanation, **Zero-Tool Untrusted Text/PDF Quarantine Summarizer** (defeats Indirect Prompt Injection!) | Quantized fast model on VPC vLLM | Local Apple Silicon Ollama / MLX (`qwen2.5-coder:7b` / `llama3.2`) |
| **Tier 2: Deep Neuro-Symbolic Reasoning** | Circuit topology synthesis, multi-domain debug, RTL/SVA/firmware generation | VPC Frontier Open-Weights on **Confidential H100/B200 GPUs** or Zero-Retention API | Local Mac Ollama / MLX (`qwen2.5-coder:32b` or best model fitting Unified Memory) |
| **Tier 3: Vision / Spatial Multimodal** | Dual-VLM datasheet extraction, whiteboard/sketch import, PCB Vision-Canvas critique | Vision-Language cluster on VPC GPUs | Local Mac Vision model (`qwen2.5-vl` / `llama3.2-vision`) |
| **Tier 4: Embeddings & Reranking** | Partitioned Datasheet & Errata RAG | GPU embedding + cross-encoder reranker (`bge-m3`) | Local Ollama / MLX (`nomic-embed-text` / `bge-m3`) |

### Reliability & Enterprise AI Privacy Guarantees
- **100% Valid Tool Calls via Grammar-Constrained Decoding (`GBNF`):** `ai/llm-gateway` enforces JSON-Schema / Protobuf state-machine decoding (`Outlines` / `llama.cpp GBNF`) + **Dynamic Tool Subset Routing (5–8 tools per specialist step)** so even 7B Local Mac models emit zero syntax errors.
- **Zero Model Training, Confidential GPU Inference & Telemetry Redaction:** Customer IP and private NDA datasheets are NEVER used for model training; VPC Confidential GPUs encrypt model weights and prompts in hardware VRAM; observability spans pass through `SecretBox` netlist scrubbers.
