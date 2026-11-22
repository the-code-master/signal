# Local macOS Apple Silicon Setup & Dual-IDE (`Cursor` + `Antigravity`) Playbook

> **Governing Specs:** [`09-local-development-environment.md`](../../architecture/09-local-development-environment.md) • [`12-multi-ide-collaboration.md`](../../architecture/12-multi-ide-collaboration.md) • [`AGENTS.md`](../../AGENTS.md)

---

## 1. Clone & Run Signal Locally on Your Mac (`< 2 Minutes`)

### Step 1 — One-Time Apple Silicon Prerequisites
```bash
# Homebrew tools (skip any already installed)
brew install git uv bufbuild/buf/buf helm just
rustup toolchain install 1.100.0 --component rustfmt clippy
uv python install 3.15

# Node 26 LTS + pnpm 11 (via nvm or fnm)
nvm install 26 && nvm use 26
npm install -g pnpm@11
```

### Step 2 — Clone, Bootstrap & Launch All 3 Tiers Locally
```bash
mkdir -p ~/Developer && cd ~/Developer
git clone https://github.com/the-code-master/signal.git
cd ~/Developer/signal

# Verify tools, install/build Rust + FastAPI + Angular v22.3, and launch!
./signal.sh doctor
./signal.sh bootstrap
./signal.sh dev run
```
Open `http://127.0.0.1:4200` on your Mac browser (`Ctrl+C` cleanly stops all 3 tiers: `:4200` Angular `v22.3` + Node `26` Edge, `:8080` Python `3.15` FastAPI/Granian, and `:50051` Pure-Rust `1.100` `signal-pod`).

> **Want `< 100 ms` Live Angular HMR While Editing UI in Cursor?**  
> Run `./signal.sh dev run` (or run `pnpm --dir web serve` in a second terminal—`web/proxy.conf.json` automatically proxies `/api/*` to local FastAPI `:8080`).

---

## 2. Connecting `Cursor` + `Antigravity` Together Without Conflicts

### CRITICAL Rule on One Mac: Use Isolated `git worktree` Folders!
Never open the **exact same folder** in both Cursor Desktop and Antigravity Desktop simultaneously—one `.git` working directory can only have **one checked-out branch at a time**, so `git checkout -b ag/...` in Antigravity would switch the active branch underneath Cursor!

Run **one built-in command** to create a zero-copy linked Git worktree sharing the exact same `.git` repository instantaneously:
```bash
cd ~/Developer/signal
./signal.sh worktree
```
This gives you two isolated workspaces backed by **one shared local Git object database**:

| IDE | Folder to Open on Your Mac | Branch Prefix & Swimlane |
|---|---|---|
| **Cursor** | `~/Developer/signal` | `cursor/*` — **Lane B:** `web/**`, `services/**`, `ai/{orchestrator,llm-gateway}/**` + `TODO-LEDGER-CURSOR-LANE-B.md` |
| **Antigravity Desktop** *(or Cloud Antigravity via `origin/main`)* | `~/Developer/signal-ag` | `ag/*` & `contract/*` — **Contracts + Lane A:** `contracts/**`, `engine/**`, `infra/**`, `ai/{datasheet-ingest,rag-service}/**` + `TODO-LEDGER-ANTIGRAVITY-LANE-A.md` |

---

## 3. Daily Phase-by-Phase Dual-IDE Loop (`Phase 1` → `Phase 8`)

1. **Check What's Next Anytime:**
   ```bash
   ./signal.sh next --lane contracts
   ./signal.sh next --lane ag
   ./signal.sh next --lane cursor
   ```
2. **Step A — Freeze Phase Contracts First (`~3 min`):**
   - In **Antigravity** (or in Cursor via **`/run-contracts`**), merge the phase's `C-P<N>-*` contract tasks (`contracts/**`) to `origin/main` first.
3. **Step B — Fire Both IDEs Simultaneously (Zero Overlap):**
   - **In Cursor (`~/Developer/signal`):** Type **`/run-lane-b`** in Cursor Agent chat (defined in [`.cursor/commands/run-lane-b.md`](../../.cursor/commands/run-lane-b.md)). Cursor pulls `origin/main`, verifies contracts are `[x]`, executes the next `CU-P<N>-*` task, runs `./signal.sh check --changed`, ticks `TODO-LEDGER-CURSOR-LANE-B.md`, and pushes to `origin/main`.
   - **In Antigravity (`~/Developer/signal-ag` or Cloud):** Prompt `"Execute next Lane A tasks for Phase <N>"`. Antigravity executes `AG-P<N>-*` on `engine/**` / `ai/**`, runs `./signal.sh check --changed`, ticks `TODO-LEDGER-ANTIGRAVITY-LANE-A.md`, and pushes to `origin/main`.
4. **Step C — Sync Status Anytime in Cursor:** Type **`/sync-status`**.
