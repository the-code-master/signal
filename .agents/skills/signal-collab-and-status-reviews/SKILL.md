---
name: signal-collab-and-status-reviews
description: Builds Signal's built-in Zero-Leak Cross-Team Collaboration Space (Design, Development, Software, Manufacturing, Operations channels, @entity-anchored threads, encrypted WebRTC/viewport huddles), live Project Status Control Tower (stage gates, RAG health, dependencies, action items, ADR/EDR decisions), and Automated Review Engine (Weekly Review, Monthly Executive Review, Stage-Gate Project Status Review) across services/collab-hub, web/libs/collab-hub, and collab_review.proto. Use whenever working on team collaboration, huddles, project status dashboards, or automated weekly/monthly/status reviews.
---

# Signal Native Collaboration Space, Project Status & Automated Reviews

Governing spec: `architecture/21-collaboration-space-project-status-and-automated-reviews.md` | UI Standard: `docs/engineering/ANGULAR_UI_STYLE_GUIDE.md`.

## 1. Zero-Leak Cross-Functional Collaboration (`services/collab-hub` + `web/libs/collab-hub`)
- Provide scoped native channels unifying **Design**, **Development/Bring-Up**, **Software/Firmware**, **Manufacturing**, and **Operations/Supply-Chain** (`#project-status`, `#design-ecad-mcad`, `#dev-bringup-sim`, `#software-firmware`, `#manufacturing-cam-fab`, `#ops-plm-supply`, `#ccb-approvals`, plus org portfolio hubs) so teams never resort to Slack or Microsoft Teams.
- Support typed live **`@entity` anchors** (`@req:`, `@net:`, `@comp:`, `@pcb:`, `@cad:`, `@cam:`, `@ecn:`, `@pcn:`, `@bom:`, `@commit:`) and self-hosted encrypted **WebRTC Audio/Video + "Follow My Viewport" Huddles** (`signal-huddle-sfu` over `turns:443`).
- **Hard Security & Isolation Invariants:**
  - Encrypt message bodies, decisions, and review narratives with the **Project's Hybrid PQC DEK** (`HKDF` subkey `collab-v1`) inside PostgreSQL RLS (`NOBYPASSRLS`) and route realtime events over per-project NATS JWT ACLs (`signal.{org}.{proj}.collab.>`).
  - Enforce **`required_repo_scopes[]`** against live `REPO_ACCESS_ENTRY` on every channel, `@entity` anchor, and review section (uniform `404` for unauthorized projects; `[Restricted]` redaction for out-of-scope sub-project entities).
  - **ZERO raw file upload/download attachments in chat** (reference canonical entities or Zero-Leak watermarked viewport snapshots only); honor Encrypted Cloud Clipboard.
  - Place `services/collab-hub` (Python `3.15+` FastAPI / Granian) and `signal-huddle-sfu` strictly in **Tier 2 (`pool-api`)** — never burden Tier 3 Rust `1.100+` worker nodes (`pool-rust-engine`) with chat/huddle load!

## 2. Live Project Status & Automated Weekly / Monthly / Status Review Engine
- Maintain native Stage-Gates (`CONCEPT` -> `EVT` -> `DVT` -> `PVT` -> `MP`), RAG health (`GREEN`/`AMBER`/`RED` with mandatory blocker links), cross-team dependencies, action items, and immutable Engineering Decisions (`ADR`/`EDR`).
- Generate **`WEEKLY_REVIEW`**, **`MONTHLY_REVIEW`**, and **`PROJECT_STATUS_REVIEW`** directly from **100% deterministic ground-truth PostgreSQL `19+`/Rust telemetry** (DRC/ERC/LVS/Sim/DFM/CAM counts, `Super-Commit`s, `ECO`/`ECN`/`MCN`/`PCN`/`Deviation`/`NCR`, `CTB %`, BOM cost, `vcs-auditor` posture) — NEVER let an LLM compute numeric KPIs!
- Route any free-text chat/blocker summarization strictly through Signal's **Zero-Tool Quarantine LLM (`GBNF` JSON output)** with verifiable message/commit citations.
- Archive signed review snapshots to `<project>-ops/reviews/{weekly,monthly,status}/` (Padmé-blinded `AES-256-GCM-SIV`) and require **Step-Up 2FA `ML-DSA-65` + Fail-Closed 3x Quorum Audit Outbox** for Stage-Gate approvals; deliver external archives ONLY via Step-Up 2FA write-only `russh` SFTP.

## 3. Clean Angular v22+ UI Implementation (`web/libs/collab-hub`)
- Follow `ANGULAR_UI_STYLE_GUIDE.md` strictly (Angular `v22+` ONLY): separate `.component.ts` (`<= 120 LOC`) / `.component.html` (`<= 150 LOC`, max nesting `<= 2`, zero `@if`-soup/duplication) / `.logic.ts` / `.model.ts`; use ONLY `@signal/design-system` (`.sig-*`) and `@signal/ui-kit` (`sig-panel`, `sig-card`, `sig-data-grid`, `sig-status-pill`, `sig-stepper-timeline`) with ZERO per-component CSS!

## Dev Response & Anti-Hallucination
- Verify exact `collab_review.proto`, `project-status-review.schema.json`, `@signal/ui-kit` components, and RLS SQL columns via `rg`/`view_file` before editing; never claim builds/tests pass without observed exit code `0`.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and verification result.
