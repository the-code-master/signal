Synchronize repository status across Antigravity (Lane A) and Cursor (Lane B):

1. Run `git fetch origin --prune && git status -sb`.
2. Run `./signal.sh next --lane contracts`, `./signal.sh next --lane ag`, and `./signal.sh next --lane cursor`.
3. Inspect the latest file in `coordination/handoffs/` and report in 3 bullet points: (a) Contract gate status, (b) Next Antigravity task, and (c) Next Cursor task.
