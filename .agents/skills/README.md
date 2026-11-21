# Signal — Cross-IDE Agent Skills Library (`.agents/skills/`)

> **Open Standard Compatibility:** All skills in this directory follow the **Agent Skills Open Standard (`SKILL.md`)** and are natively discovered by **both Antigravity and Cursor** using **progressive disclosure** (only compact YAML metadata loads at startup; full instructions load on demand when a task matches).

---

## Available Skills (17)

| Skill Name | Scope / Lane | Purpose |
|---|---|---|
| [`signal-session-start`](./signal-session-start/SKILL.md) | Both | Mandatory session-start ritual, peer journal read, disjoint scope check, and branching |
| [`signal-token-efficient-editing`](./signal-token-efficient-editing/SKILL.md) | Both | Minimal-token context discovery, 2–3 sentence replies, anti-hallucination, `<= 300 LOC` modularity |
| [`signal-contract-first`](./signal-contract-first/SKILL.md) | Shared (Serialized) | Authoring `contracts/` (`signal/v1/*`, SI suffixes, `bigint`, Arrow, `fixtures/`, `transcripts/`) |
| [`signal-rust-crate-architect`](./signal-rust-crate-architect/SKILL.md) | Lane A (`engine/`) | Scaffolding Rust 2024 crates (`#![forbid(unsafe_code)]`, `signal-proto`, `thiserror`, `SecretBox`) |
| [`signal-rust-physics-and-geometry`](./signal-rust-physics-and-geometry/SKILL.md) | Lane A (`engine/`) | Implementing `ElectronicModule`, Saltation/`Revolve` SPICE, `i128`/Shewchuk & `0.5-ULP` determinism |
| [`signal-universal-parts-registry`](./signal-universal-parts-registry/SKILL.md) | Both | Universal Datasheet/Component repo, Master Pin Table, IEEE/IEC Symbols, Parametric Padstacks & 3-Tier AVL |
| [`signal-cad-cam-plm-ops`](./signal-cad-cam-plm-ops/SKILL.md) | Both | Integrated Mechanical CAD, CNC CAM, 3D Printing, 3D Visualization, eBOM/mBOM, ECN/ECO/MCN/PCN/Deviations & Ops |
| [`signal-angular-workbench`](./signal-angular-workbench/SKILL.md) | Lane B (`web/`) | Building Angular 19/20+ Zoneless Signals standalone workbenches (`web/libs/*`), `bigint`, WCAG |
| [`signal-zero-leak-canvas-and-dlp`](./signal-zero-leak-canvas-and-dlp/SKILL.md) | Both (`web/` + `engine/`) | Dual-Plane `Zstd-QOI`/WebP streaming, `HudOverlayPrimitive`, Multi-Monitor & Constellation DLP |
| [`signal-nodejs-bff-control-plane`](./signal-nodejs-bff-control-plane/SKILL.md) | Lane B (`services/`) | Stateless Node.js 22 services, `@Require*` uniform `404`, RLS, PKCE cookies & Fail-Closed Outbox |
| [`signal-byo-git-and-vault`](./signal-byo-git-and-vault/SKILL.md) | Both | Multi-provider BYO-Git, lazy repo pacer, Padmé-blinded `AES-256-GCM-SIV` & Dual-Fenced `Super-Commit` |
| [`signal-security-isolation-audit`](./signal-security-isolation-audit/SKILL.md) | Both | OWASP ASVS L3 & "1 Project = 1 Security Cell" audit (PQC, H100/B200, eBPF, sandboxes, SFTP) |
| [`signal-ai-neuro-symbolic-rag`](./signal-ai-neuro-symbolic-rag/SKILL.md) | Both (`ai/`) | Neuro-symbolic agents, Multimodal + Dual-LLM Quarantine, `GBNF`, 3-Tier `pgvector` & Proof re-check |
| [`signal-local-mac-dev`](./signal-local-mac-dev/SKILL.md) | Both (`infra/local/`) | Apple Silicon `dev:lite`/`dev:full`, `lavapipe` WGSL CI, `local-git-mock` & `dev-guest-auth` guards |
| [`signal-release-deploy-topology`](./signal-release-deploy-topology/SKILL.md) | Both (`infra/`, `releases/`) | Platform SemVer & Auto Release Notes, monorepo `justfile`/Docker/Helm/GitOps, and 3-tier K8s (`1–4` extensible Rust nodes) |
| [`signal-verification-and-pr-gate`](./signal-verification-and-pr-gate/SKILL.md) | Both | Full Definition-of-Done verification, supply-chain vetting, journal update & clean PR creation |
| [`signal-cross-ide-handoff`](./signal-cross-ide-handoff/SKILL.md) | Both (`coordination/`) | Immutable Antigravity ↔ Cursor handoff packets, blocker escalation & `> 24h` stale-claim takeover |
