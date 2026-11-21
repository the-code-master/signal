# Signal — AI-Agent-First Cloud EDA Platform

> **Signal** (by **Signal Design, Inc.**) is a next-generation, cloud-native, **AI-Agent-First Electronic Design Automation (EDA)** platform built on a pure **Rust** physics and EDA engine deployed in Kubernetes, paired with a **Node.js + Angular** zero-trust web workspace.

## Current Phase: Architecture & Planning

This repository is currently in the **complete architecture and planning phase** — no production application code has been written yet. Every architectural decision, subsystem design, security model, data contract, roadmap, and multi-AI-IDE collaboration protocol is documented inside [`architecture/`](./architecture/README.md).

## Documentation Map

Start with the **[Master Architecture Index](./architecture/README.md)**, or jump directly to any specification:

| # | Document | Focus |
|---|---|---|
| 00 | [Master Architecture Index](./architecture/README.md) | Executive overview, architectural pillars, system context, glossary |
| 01 | [Product Vision & Complete EDA Feature Catalog](./architecture/01-product-vision-and-features.md) | Unified product workspace (PRD/HFS, Schematic, PCB, IC/ASIC, FPGA, Firmware, BIOS) & AI-first UX |
| 02 | [System Architecture & Cloud Topology](./architecture/02-system-architecture.md) | End-to-end architecture, tech stack, monorepo blueprint, data flows, SLAs |
| 03 | [Core Rust EDA & Modular Physics Engine](./architecture/03-rust-eda-engine.md) | Component-as-a-Rust-Module architecture, Acausal MNA + Signal-Flow hybrid SPICE, PCB/IC kernels, GPU rendering |
| 04 | [Kubernetes Runtime & Multi-User Collaboration](./architecture/04-kubernetes-runtime-and-collaboration.md) | Stateful pod-per-project lifecycle, sticky multi-user routing, CRDT sync, GPU compute bursting |
| 05 | [AI Agent Orchestration & Universal Datasheet RAG](./architecture/05-ai-agent-and-datasheet-rag.md) | Multi-agent closed-loop EDA workflows, Universal Datasheet DB, Hybrid RAG, LLM/GPU routing |
| 06 | [Frontend (Angular) & Node.js Platform BFF](./architecture/06-frontend-angular-and-nodejs-bff.md) | Angular unified IDE shell, Zero-Leak GPU streaming & WebGPU canvas, Node.js control plane |
| 07 | [Zero-Trust Security, Encryption, DLP & Authorized SFTP](./architecture/07-security-encryption-dlp-and-sftp.md) | Envelope encryption, no local copies, anti-export/screenshot/print, 2FA-gated SFTP-only delivery |
| 08 | [Identity, Enterprise Onboarding, Projects & Templates](./architecture/08-identity-enterprise-and-project-model.md) | Individual & approval-based Enterprise onboarding, RBAC/ABAC, multi-type projects, templates |
| 09 | [Local Development Environment (macOS + Local LLM)](./architecture/09-local-development-environment.md) | Mac Apple Silicon dev setup, default local LLM, local `Guest` / `Signal Design, Inc.` mode |
| 10 | [Canonical API & Data Contracts](./architecture/10-api-and-data-contracts.md) | Contract-first gRPC/Protobuf, REST/WebSocket, Rust component trait & datasheet schemas |
| 11 | [Phased Implementation Roadmap & Risk Register](./architecture/11-implementation-roadmap.md) | Milestone plan from Phase 0 foundation to GA, verification gates, technical risk register |
| 12 | [Multi-AI-IDE Parallel Collaboration Protocol](./architecture/12-multi-ide-collaboration.md) | Deadlock-free workflow for Antigravity + Cursor working in parallel via worktrees, lanes & contracts |
| 13 | [Version Control: Customer-Owned BYO-GitHub Org & Security Auditor](./architecture/13-vcs-git-server-and-storage-analysis.md) | Unified GitHub Org → Project Repo model (Fine-Grained PAT / GitHub App, zero Signal file custody, optional `ENCRYPTED_VAULT` mode, periodic Org vulnerability auditor) |
| 14 | [Universal Scale, Global Accessibility & Future Expandability](./architecture/14-universal-scale-accessibility-and-expandability.md) | Zero-floor/infinite-ceiling design: kid's hobby project in developing nations to 100B-transistor GPU/ASIC & future multi-physics plugins |

## Parallel AI Development (Antigravity + Cursor)

This repository is pre-configured for deadlock-free simultaneous development by **Antigravity** and **Cursor**:
- **Canonical Rulebook (Both IDEs):** [`AGENTS.md`](./AGENTS.md)
- **Cursor Always-Apply Rule:** [`.cursor/rules/signal-architecture.mdc`](./.cursor/rules/signal-architecture.mdc)
- **Collaboration Protocol:** [`architecture/12-multi-ide-collaboration.md`](./architecture/12-multi-ide-collaboration.md)
- **Task Board & Per-Agent Journals:** [`coordination/TASK_BOARD.md`](./coordination/TASK_BOARD.md) ([Antigravity](./coordination/agents/antigravity.md) | [Cursor](./coordination/agents/cursor.md))
