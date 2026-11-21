---
name: signal-local-mac-dev
description: Configures, runs, and debugs Signal's macOS Apple Silicon local development harness (dev:lite, dev:full, dev:k8s), shared signal-proto + sccache/lld fast Rust builds, headless lavapipe Vulkan WGSL parity, Local Mac Ollama/MLX + deterministic_mock fallback, and hard-guarded Guest @ Signal Design, Inc. mode. Use when working on infra/local/ or local dev workflows.
---

# Signal Local Mac Apple Silicon Dev Harness (`infra/local/**`)

Governing specs: `architecture/09-local-development-environment.md`, `architecture/20-versioning-release-monorepo-build-and-k8s-topology.md`.

## 1. Three Local Execution Profiles (Monorepo `justfile` Defaults)
- **`just dev` / `pnpm dev:lite` (Default Daily Inner Loop — `< 1.5 GB RAM`, `< 30 s` Boot):**
  - Native Rust `signal-pod` (Apple Silicon Metal `wgpu`), Angular HMR, Node.js BFF, SQLite/in-proc store, `local-git-mock`, DevKMS (active Hybrid PQC!), Mock SFTP, and Local Mac Ollama/MLX (falling back automatically to `deterministic_mock` when offline or in CI).
- **`just dev-full` / `pnpm dev:full` (Full Service Parity — Native `linux/arm64` Docker Compose):**
  - PostgreSQL + `pgvector` (with RLS active), Redis, NATS JetStream, Envoy edge relay, `signal-egress-proxy`, Vault/DevKMS, `local-git-mock`, and Mock SFTP.
- **`just dev-k8s` / `just deploy-local-k8s` (Full 3-Tier K8s Parity — Local `k3d` / `OrbStack`):**
  - Deploys `infra/k8s/helm/signal -f values-local.yaml` with separate `signal-web` (Angular+Node) pod, separate `signal-api` + Envoy `signal-engine-bridge` pod, and `1–4` simulated Rust Engine worker nodes (`signal-operator`, linearizable Leases, Cilium NetworkPolicies).

## 2. Fast Build & Cross-Platform GPU Parity
- Centralize Rust Protobuf codegen in `engine/crates/signal-proto` + `cargo-hakari` + `sccache` + `lld` so incremental Rust rebuilds stay `< 2 s`.
- Verify every WGSL shader written on Apple Metal against **headless software-Vulkan (`lavapipe`)** in Linux CI to eliminate Metal-vs-Vulkan visual or compute drift.
- Keep environment templates split per subsystem under `infra/local/env/<subsystem>.env.example` (never one monolithic conflict-prone `.env.example`).

## 3. Hard-Guarded Zero-Bypass `Guest @ Signal Design, Inc.`
- `local-guest-0001` is seeded as a real member inside `local-git-mock` `REPO_ACCESS_ENTRY` and executes **100% of production RBAC, RLS, and Padmé `AES-256-GCM-SIV` / Hybrid PQC encryption paths** — NEVER write `if (isLocalDev) return true;` security bypasses!
- `dev-guest-auth` activates ONLY when ALL THREE hold simultaneously:
  1. Compile-time Cargo feature `dev-guest-auth` (explicitly forbidden in release builds via `compile_error!`).
  2. Runtime `SIGNAL_ENV=local`.
  3. Bind address strictly loopback (`127.0.0.1` / `::1`).
- CI MUST verify that building with `--release --features dev-guest-auth` fails compilation deterministically!

## Dev Response & Anti-Hallucination
- Verify actual compose files, env templates (`infra/local/env/`), and CLI scripts via `view_file` before editing; never claim `dev:lite`/`dev:full` boots cleanly without observing command output.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and observed verification result.
