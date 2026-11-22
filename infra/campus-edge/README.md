# Signal Campus Edge — Offline-Resilient University Appliance (`FEAT-120`)

Packages a single-node `k3s` classroom appliance for universities with intermittent internet connectivity:
- **`signal-cell-wasm` Isolate Host:** Serves up to `250` concurrent `< 8 MB` student EDA workspaces on a single workstation/server.
- **Offline Universal Datasheet + Open-Silicon PDK Cache:** Pre-indexed `SkyWater Sky130`, `GlobalFoundries GF180MCU`, `IHP SG13G2`, and Tier-1 Global CAS datasheets.
- **Local Quantized Ollama (`GBNF`):** Zero-WAN neuro-symbolic datasheet Q&A and guided lab hints.
- **Store-and-Forward Delta Replication (`signal_git_sync::campus_edge_sync`):** Buffers student `Super-Commit`s locally while offline and deduplicates/compresses packfile deltas when campus internet restores.
