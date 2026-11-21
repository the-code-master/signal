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
2. **Check Phase Gate & Backlog:**
   - Read `AGENTS.md` Section 1 and `coordination/TASK_BOARD.md`.
   - If still in Pre-Phase-0 Architecture & Planning, **DO NOT** create or edit `engine/`, `services/`, `web/`, `ai/`, `contracts/`, or `infra/` without explicit user approval.
3. **Read Peer Agent Journal (Strictly Read-Only) & Handoffs:**
   - **Antigravity** reads `coordination/agents/cursor.md` (NEVER edits it).
   - **Cursor** reads `coordination/agents/antigravity.md` (NEVER edits it).
   - List `coordination/handoffs/` and read any handoff note created since your last commit.
4. **Verify Disjoint Scope & Swimlane:**
   - Confirm your planned `touched_paths` have **zero overlap** with any active `IN_PROGRESS` task in the peer journal.
   - Default swimlanes: Lane A (Antigravity: `engine/**`, `benchmarks/**`, `infra/k8s/**`, `ai/datasheet-ingest/**`, `ai/rag-service/**`), Lane B (Cursor: `web/**`, `services/**`, `ai/orchestrator/**`), Shared Governed Zone (`contracts/**`, `architecture/**`, `AGENTS.md`, `.cursor/rules/**`, `infra/local/**`, `VERSION`, `justfile`, `releases/**` — serialized PRs only).
   - If overlap exists, invoke `signal-cross-ide-handoff` and pivot to the next unblocked disjoint task.
5. **Read Governing Specs Narrowly:**
   - Consult ONLY the matching `architecture/01..21-*.md` and `docs/engineering/*.md` sections needed for the task (avoid ingesting unrelated specs).
6. **Branch From Latest `origin/main` (Implementation Work):**
   - Antigravity: `git checkout -b ag/<task-id>-<slug> origin/main`
   - Cursor: `git checkout -b cursor/<task-id>-<slug> origin/main`
   - Serialized contracts: `git checkout -b contract/<task-id>-<slug> origin/main`
7. **Claim Task in Own Journal Only:**
   - Append `IN_PROGRESS` claim with explicit `touched_paths` and `contract_dependency` to your own journal (`coordination/agents/antigravity.md` OR `coordination/agents/cursor.md`).

## Hard Guardrails
- Never commit parallel implementation code directly to `main`.
- Never edit the peer agent's journal file.
- Never wait or poll for the peer agent; build against merged `contracts/` stubs, `contracts/fixtures/`, and `contracts/transcripts/*.jsonl`.
