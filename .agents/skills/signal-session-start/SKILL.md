---
name: signal-session-start
description: Runs the mandatory Signal EDA session-start ritual before editing any file in Antigravity or Cursor. Use at the beginning of every session or before claiming a new task to sync origin/main, inspect coordination state, verify disjoint touched_paths, check the Phase Gate, and create a properly prefixed branch.
---

# Signal Session-Start Ritual

Execute this exact sequence before modifying any repository file. Never guess coordination state.

## Steps

1. **Sync Remote State (Read-Only Fetch):**
   ```bash
   git fetch origin --prune && git status -sb
   ```
2. **Check Phase Gate, Execution Plan & Single-Writer TODO Ledger:**
   - Read `AGENTS.md` Section 1, `coordination/TASK_BOARD.md`, [`plan/README.md`](../../../plan/README.md), the active [`plan/phase-*.md`](../../../plan/phase-0-foundation-contracts-and-cicd.md) plan, and your single-writer TODO ledger (`plan/TODO-LEDGER-ANTIGRAVITY-LANE-A.md` for Antigravity, `plan/TODO-LEDGER-CURSOR-LANE-B.md` for Cursor, or `plan/TODO-LEDGER-SHARED-CONTRACTS.md` for serialized `contract/*` PRs).
   - If still in Pre-Phase-0 Architecture & Planning, **DO NOT** create or edit `engine/`, `services/`, `web/`, `ai/`, `contracts/`, or `infra/` without explicit user approval.
3. **Read Peer Agent Journal & Peer TODO Ledger (Strictly Read-Only) & Handoffs:**
   - **Antigravity** reads `coordination/agents/cursor.md` & `plan/TODO-LEDGER-CURSOR-LANE-B.md` (NEVER edits them).
   - **Cursor** reads `coordination/agents/antigravity.md` & `plan/TODO-LEDGER-ANTIGRAVITY-LANE-A.md` (NEVER edits them).
   - List `coordination/handoffs/` and read any handoff note created since your last commit.
4. **Verify Disjoint Scope & Swimlane:**
   - Confirm your planned `touched_paths` have **zero overlap** with any active `IN_PROGRESS` task in the peer journal.
   - Default swimlanes: Lane A (Antigravity: `engine/**`, `benchmarks/**`, `infra/k8s/**`, `ai/datasheet-ingest/**`, `ai/rag-service/**`), Lane B (Cursor: `web/**`, `services/**`, `ai/orchestrator/**`, `ai/llm-gateway/**`), Shared Governed Zone (`contracts/**`, `architecture/**`, `plan/**`, `AGENTS.md`, `.cursor/rules/**`, `infra/local/**`, `VERSION`, `signal.sh`, `scripts/**`, `justfile`, `releases/**`).
   - If overlap exists, invoke `signal-cross-ide-handoff` and pivot to the next unblocked disjoint task.
5. **Read Governing Specs & Phase Plan Card Narrowly:**
   - Consult ONLY the matching `plan/phase-*.md` task card, `architecture/01..21-*.md`, and `docs/engineering/*.md` sections needed for the task (avoid ingesting unrelated specs).
6. **Branch From Latest `origin/main` (Implementation Work):**
   - Antigravity: `git checkout -b ag/<task-id>-<slug> origin/main`
   - Cursor: `git checkout -b cursor/<task-id>-<slug> origin/main`
   - Serialized contracts: `git checkout -b contract/<task-id>-<slug> origin/main`
7. **Claim Task in Own Journal Only:**
   - Append `IN_PROGRESS` claim with explicit `touched_paths` and `contract_dependency` to your own journal (`coordination/agents/antigravity.md` OR `coordination/agents/cursor.md`).

## Hard Guardrails
- Never commit parallel implementation code directly to `main`.
- Never edit the peer agent's journal file or the peer agent's TODO ledger (`plan/TODO-LEDGER-*.md`).
- Never wait or poll for the peer agent; build against merged `contracts/` stubs, `contracts/fixtures/`, and `contracts/transcripts/*.jsonl`.
