---
name: signal-cross-ide-handoff
description: Manages deadlock-free handoffs, contract-blocker escalations, overlap resolution, and >24h stale-claim takeovers between Antigravity (Lane A) and Cursor (Lane B). Use whenever passing deliverables to the peer IDE, encountering BLOCKED_ON_CONTRACT or OVERLAP_DETECTED, or reclaiming an abandoned task.
---

# Signal Cross-IDE Handoff Protocol (Antigravity ↔ Cursor)

Governing spec: `architecture/12-multi-ide-collaboration.md`.

## Core Rule: Never Block, Never Poll, Never Edit Peer Journal
- **Antigravity** writes ONLY to `coordination/agents/antigravity.md`.
- **Cursor** writes ONLY to `coordination/agents/cursor.md`.
- All cross-IDE messages go into a **new immutable file**:
  `coordination/handoffs/YYYYMMDD-<task-id>-<from>-to-<to>.md`
  (`from`/`to` = `ag` or `cursor`).

## Immutable Handoff File Template (Keep `< 40` Lines — Token Efficient!)
```markdown
# Handoff: <task-id> (<from> -> <to>)

- **Date (UTC):** YYYY-MM-DD
- **From / To:** Antigravity -> Cursor (or Cursor -> Antigravity)
- **Trigger:** `HANDOFF_READY` | `BLOCKED_ON_CONTRACT` | `OVERLAP_DETECTED` | `STALE_CLAIM_RECLAIM`
- **Merged Commit / Branch:** `<sha>` (`<branch>`)
- **Contract / Schema Anchors:** `contracts/proto/signal/v1/<file>.proto`, `contracts/transcripts/<file>.jsonl`

## What Was Delivered / Needed
- Bullet 1 (concrete file paths & symbol names)
- Bullet 2 (exact verification command that already passes)

## Next Action for Receiver
1. `git fetch origin --prune && git rebase origin/main`
2. Exact command or task ID to pick up next on `coordination/TASK_BOARD.md`.
```

## Blocker & Stale-Claim Resolution
1. **`BLOCKED_ON_CONTRACT`:** Log status in your own journal, write a crisp handoff note specifying the exact missing `.proto` message/field (with SI unit suffix + `bigint`), open a tiny `contract/<task-id>-<slug>` PR if authorized, and immediately pivot to the next unblocked disjoint task on `TASK_BOARD.md`.
2. **`OVERLAP_DETECTED`:** Never edit a file owned by an active peer claim. Log `OVERLAP_DETECTED` in your own journal, emit a handoff note proposing a clean file boundary split, and pivot to a disjoint task.
3. **Stale Claim Takeover (`> 24h` Inactive):** If a peer `IN_PROGRESS` claim has zero commits or updates for `> 24h`, create an immutable `STALE_CLAIM_RECLAIM` handoff note citing the last commit timestamp, claim the task in your own journal, and branch cleanly from `origin/main`.
