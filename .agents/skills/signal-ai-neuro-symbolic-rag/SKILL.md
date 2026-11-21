---
name: signal-ai-neuro-symbolic-rag
description: Builds Signal's neuro-symbolic AI agents, GBNF-constrained llm-gateway adapters, Multimodal VLM + Dual-LLM Zero-Tool Quarantine extractors, Dual-VLM condition-parameterized Datasheet Hybrid RAG, and Merge-Time Proof Re-Verification under ai/. Use whenever working on AI orchestration, RAG retrieval, datasheet ingestion, or LLM routing.
---

# Signal Neuro-Symbolic AI & Partitioned Datasheet RAG (`ai/**`)

Governing specs: `architecture/05-ai-agent-and-datasheet-rag.md`, `architecture/15-*` (Sec 4).

## 1. Core Neuro-Symbolic Discipline
- **LLM for Topology, Rust for Physics:** NEVER rely on LLM token math for resistor/capacitor sizing, loop compensation, transistor `(W/L)`, or DRC margins. Delegate numerical sizing to `signal-diff-sim` (Filippov Saltation + `Revolve` + Two-Stage PSS/HB) and verification to `signal-sim` / `signal-formal`.
- **K-Hop Cone-of-Influence Slicing:** Slice prompts strictly to the active K-hop subcircuit cone (`< 16k` tokens even on 100B-transistor SoCs) and enforce hard per-task token, iteration, and wall-clock budgets.
- **Speculative Branches & Merge-Time Proof Re-Verification:** Agents write ONLY to `jj` CoW branches and attach a `DesignProofBundle` bound to `base_graph_merkle_root`. Before human "Approve & Merge", automatically run `< 1 s` incremental cone-of-influence ERC/DRC/SOA re-verification on the post-rebase tree.

## 2. Multimodal VLM + Dual-LLM Prompt-Injection Quarantine
- **100% Untrusted Input Isolation:** External PDFs, silicon errata, CAD notes, commit messages, foreign-language datasheets, AND **VLM-processed whiteboard photos / datasheet curve images** MUST enter the **Zero-Tool Quarantine Tier** first.
- Quarantine models hold **ZERO tool-calling authority** and emit strictly `GBNF`-constrained typed JSON/Protobuf primitives consumed by the Privileged Planner Agent.
- Every Privileged Agent tool call carries the invoking user's non-elevated per-repo RBAC token with ZERO bindings to `sftp-egress`, `vcs-connector` credentials, or admin APIs.

## 3. Universal Datasheet Ingestion & 3-Tier `pgvector` Hybrid RAG
- Key every record by `orderable_mpn + package_code + die_revision` with co-linked Silicon Errata and **condition-parameterized curves** (`C(V_bias, T, f)`, copper-coupled `R_thetaJC/JB`) + 3-way Symbol/Footprint/3D-STEP verification (`BRONZE`/`SILVER`/`GOLD` badges).
- Enforce **3-Tier `pgvector` Sharding**: `datasheet_chunks_global` + sharded dedicated partitions (`<= 500` orgs/cluster) for `ENTERPRISE_ISOLATED` NDA pools + 64 fixed hash-bucket partitions (`NOBYPASSRLS` + `hnsw.iterative_scan = strict_order`) for `MAKER`/`PRO`.
- Serve supply-chain stock/lead-time/carbon from **Signal-Mirrored Bulk Catalogs** + **`4×` chaff-padded proxy batches** for delta checks (zero BOM set-intersection leakage).

## 4. Model-Agnostic `ai/llm-gateway`
- Enforce `GBNF` schema-constrained decoding + Dynamic Tool Subset Routing (5–8 tools/step) across Local Mac Ollama/MLX, `deterministic_mock` offline CI fallback, VPC Confidential GPU (`H100 CC` / `B200 TEE-I/O` MIG), and zero-retention cloud APIs.
- Zero customer IP training retention; scrub all telemetry spans through `SecretBox` netlist redactors.
