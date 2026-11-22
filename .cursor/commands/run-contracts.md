Execute the next serialized **Shared Contract Freeze (`C-P*`)** task before parallel Antigravity + Cursor waves begin:

1. Run `git fetch origin --prune && git pull --rebase origin main` and `./signal.sh next --lane contracts --json`.
2. Read `.agents/skills/signal-contract-first/SKILL.md` and `architecture/10-api-and-data-contracts.md`.
3. Checkout `<next_task.branch>` from `origin/main` and modify **ONLY** `contracts/**`, `plan/TODO-LEDGER-SHARED-CONTRACTS.md`, your own journal (`coordination/agents/cursor.md`), and a new handoff note in `coordination/handoffs/`.
4. Ensure 100% of 64-bit integer fields carry `[jstype = JS_STRING]`, explicit SI suffixes (`_nm`, `_ps`, `_fs`, `_uv`), and zero `/export` or `/download` OpenAPI routes.
5. Run `./signal.sh check --target contracts` (must exit `0`), tick `plan/TODO-LEDGER-SHARED-CONTRACTS.md`, merge to `main`, and `git push origin main` to unblock both IDEs.
