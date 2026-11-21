# Signal — AI-Agent-First Cloud EDA Platform

> **Signal** (by **Signal Design, Inc.**) is a next-generation, cloud-native, **AI-Agent-First Electronic Design Automation (EDA)** platform built on a pure **Rust** physics and EDA engine deployed in Kubernetes, paired with a **Node.js + Angular** zero-trust web workspace.

## Current Phase: Architecture & Planning

This repository is currently in the **complete architecture and planning phase** — no production application code has been written yet. Every architectural decision, subsystem design, security model, data contract, roadmap, and multi-AI-IDE collaboration protocol is documented inside [`architecture/`](./architecture/README.md).

## Documentation Map

Start with the **[Master Architecture Index](./architecture/README.md)**, or jump directly to any specification:

| # | Document | Focus |
|---|---|---|
| 00 | [Master Architecture Index](./architecture/README.md) | Executive overview, split BYO-Git/Platform pillars, system context, glossary |
| 01 | [Product Vision & Complete EDA Feature Catalog](./architecture/01-product-vision-and-features.md) | Unified workspace (PRD/HFS, Differentiable SPICE, Formal, 224G PCB, 3D EM/Aging, IC/UPF/CPO, FPGA, FW/BIOS, Digital Twin, Clean-Room Import, Carbon/Supply-Chain) |
| 02 | [System Architecture & Cloud Topology](./architecture/02-system-architecture.md) | End-to-end cloud topology, Envoy HTTP/3 WebTransport + `turns:443` relay, Sovereign Jurisdiction pinning, monorepo blueprint, SLAs |
| 03 | [Core Rust EDA & Modular Physics Engine](./architecture/03-rust-eda-engine.md) | `ElectronicModule`, Ruiz/charge-conserving MNA SPICE, Adjoint `signal-diff-sim`, PINN surrogates, Formal, 3D EM, `i128`/Euler geometry, `YUV 4:4:4` GPU rendering |
| 04 | [Kubernetes Runtime & Multi-User Collaboration](./architecture/04-kubernetes-runtime-and-collaboration.md) | Linearizable K8s Lease fencing, In-Place Vertical Resize + `< 150 ms` Live Migration, conflating CRDTs, Hardware MIG & `CONFIDENTIAL_SILICON` pods |
| 05 | [AI Agent Orchestration & Universal Datasheet RAG](./architecture/05-ai-agent-and-datasheet-rag.md) | Neuro-symbolic agents, Dual-LLM quarantine, `GBNF` decoding, Dual-VLM Datasheet & Silicon Errata DB, physically partitioned `pgvector` HNSW RAG |
| 06 | [Frontend (Angular) & Node.js Platform BFF](./architecture/06-frontend-angular-and-nodejs-bff.md) | Angular 19+ Signals IDE, `YUV 4:4:4` + lossless dirty-tile stream, Multi-Pane GPU Atlas, Visual Diff & Spatial Review, Encrypted Cloud Clipboard, Node.js BFF |
| 07 | [Zero-Trust Security, Encryption, DLP & Authorized SFTP](./architecture/07-security-encryption-dlp-and-sftp.md) | Hybrid Post-Quantum (`ML-KEM-768` + `ML-DSA-65`), convergent `AES-256-GCM-SIV`, Tardos watermarks, replay-proof 2FA, write-only `russh` SFTP, RFC-3161 WORM logs |
| 08 | [Identity, Provider-Synced Access Review, Multi-Type Projects & Templates](./architecture/08-identity-enterprise-and-project-model.md) | Dual SSO + OAuth PKCE, `ORG_NAMESPACE` & `USER_NAMESPACE`, 2-of-3 Quorum Break-Glass, uniform `404` guard, Access Review Console, SPDX & 21 CFR Part 11 sign-offs |
| 09 | [Local Development Environment (macOS + Local LLM)](./architecture/09-local-development-environment.md) | Mac Apple Silicon `dev:lite` (`< 1.5 GB RAM`) & `dev:full`, `GBNF` Local LLM default, zero-bypass `Guest @ Signal Design, Inc.` in `local-git-mock` |
| 10 | [Canonical API & Data Contracts](./architecture/10-api-and-data-contracts.md) | Build-time-only codegen, SI-unit-suffixed Protobuf (`_nm`, `_uv`, `_ps`), `<= 1 MB` Arrow streaming, `Super-Commit` & UPF schemas, conformance fixtures |
| 11 | [Phased Implementation Roadmap & Risk Register](./architecture/11-implementation-roadmap.md) | Milestone plan from Phase 0 foundation to GA, `benchmarks/` golden physics suites, verification gates, technical risk register |
| 12 | [Multi-AI-IDE Parallel Collaboration Protocol](./architecture/12-multi-ide-collaboration.md) | Deadlock-free workflow for Antigravity + Cursor via disjoint lanes, build-time codegen, 24h stale-claim expiry & `cargo vet` gates |
| 13 | [Version Control: BYO-Git (GitHub, GitLab & Bitbucket), Multi-Repo Projects & Security Auditor](./architecture/13-vcs-git-server-and-storage-analysis.md) | Unified `GitProviderAdapter`, immutable `provider_repo_id`, convergent `ENCRYPTED_VAULT`, Two-Phase `Super-Commit` (`repos.lock.json`), `vcs-auditor` |
| 14 | [Universal Scale, Global Accessibility & Future Expandability](./architecture/14-universal-scale-accessibility-and-expandability.md) | Zero-floor/infinite-ceiling design: developing-nation `EXPLORER` mode to 100B-transistor zero-lateral-movement GPU federation, CPO photonics & WASM SDK |
| 15 | [Application Security & Complete Project Isolation](./architecture/15-application-security-and-project-isolation.md) | OWASP ASVS L3 AppSec, "1 Project = 1 Security Cell" (`CONFIDENTIAL_SILICON`, Hardware MIG, Cilium eBPF, socket-pinned anti-SSRF, Dual-LLM, `SecretBox<T>`) |
| 16 | [Adversarial Architectural Audit (112 Flaws Resolved) & Futuristic AI-EDA Innovations](./architecture/16-architectural-audit-flaws-and-futuristic-innovations.md) | Complete red-team audit of **112 cross-layer flaws (`F-001`–`F-112`)** and **12 industry-first AI-EDA innovations (`I-01`–`I-12`)** integrated across `01`–`15` |

## Parallel AI Development (Antigravity + Cursor)

This repository is pre-configured for deadlock-free simultaneous development by **Antigravity** and **Cursor**:
- **Canonical Rulebook (Both IDEs):** [`AGENTS.md`](./AGENTS.md)
- **Cursor Always-Apply Rule:** [`.cursor/rules/signal-architecture.mdc`](./.cursor/rules/signal-architecture.mdc)
- **Collaboration Protocol:** [`architecture/12-multi-ide-collaboration.md`](./architecture/12-multi-ide-collaboration.md)
- **Task Board & Per-Agent Journals:** [`coordination/TASK_BOARD.md`](./coordination/TASK_BOARD.md) ([Antigravity](./coordination/agents/antigravity.md) | [Cursor](./coordination/agents/cursor.md))
