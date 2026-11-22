Execute the next ready **Cursor Lane B (`CU-P*`)** task(s) following `AGENTS.md` without colliding with Antigravity:

1. **Sync & Check Contract Gate:**
   - Run `git fetch origin --prune && git pull --rebase origin main`.
   - Run `./signal.sh next --lane cursor --json`.
   - Read `coordination/agents/antigravity.md` (read-only) and any new files in `coordination/handoffs/`.
   - If `status == "BLOCKED_ON_CONTRACTS"`, stop immediately and report which `C-P*` contract tasks Antigravity/Contracts must merge first.

2. **Execute Strictly Inside Lane B (`web/**`, `services/**`, `ai/{orchestrator,llm-gateway}/**`):**
   - Checkout `<next_task.branch>` from `origin/main`.
   - Read the matching `.agents/skills/*/SKILL.md` files listed in `Required Skills`.
   - Enforce **Angular `v22+` ONLY** (4-file split `.component.ts <= 120 LOC` / `.component.html <= 150 LOC` nesting `<= 2` / `.logic.ts <= 250 LOC` / `.model.ts`, centralized `@signal/design-system` + `@signal/ui-kit`, zero local CSS) and **Python `3.15+` FastAPI/Granian** (fail-closed `Depends(require_*)`, uniform `404`, `with_rls_tx`).
   - **NEVER** edit `engine/**`, `infra/**`, `coordination/agents/antigravity.md`, or `plan/TODO-LEDGER-ANTIGRAVITY-LANE-A.md`.

3. **Verify, Tick & Push:**
   - Run `./signal.sh check --changed` and confirm exit code `0`.
   - Tick ONLY `plan/TODO-LEDGER-CURSOR-LANE-B.md` (`- [ ]` → `- [x]`) and log in `coordination/agents/cursor.md`.
   - Commit with a Conventional Commit message, rebase onto `origin/main`, fast-forward merge to `main`, and `git push origin main`.
   - Reply in **2–3 sentences max** (`Issue/Task -> Implementation -> Observed Verification`).
