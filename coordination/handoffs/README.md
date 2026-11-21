# Cross-IDE Handoff Notes (`coordination/handoffs/`)

This folder holds **immutable, append-only handoff notes** between **Antigravity** and **Cursor**.

## Rules
1. Never edit an existing handoff file created by the other IDE.
2. Create one new file per handoff using the naming pattern:
   `YYYYMMDD-<task-id>-<from>-to-<to>.md`  
   Example: `20261201-P0-01-ag-to-cursor.md`
3. Include:
   - Contract/commit SHA being handed off
   - Breaking/additive API changes or mock endpoints ready
   - Verification commands run and passing
   - Any explicit assumptions or edge cases
