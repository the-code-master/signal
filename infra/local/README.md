# Signal Local macOS Apple Silicon & Linux Development Harness (`infra/local/`)

> **Governing Specs:** [`09-local-development-environment.md`](../../architecture/09-local-development-environment.md) • [`20-versioning-release-monorepo-build-and-k8s-topology.md`](../../architecture/20-versioning-release-monorepo-build-and-k8s-topology.md)

## Quick Commands (`./signal.sh`)
- `./signal.sh dev lite` — `< 1.5 GB RAM` lightweight profile (`F-105`) with `Guest @ Signal Design, Inc.` (`local-guest-0001`).
- `./signal.sh dev full` — Native ARM64/multi-arch Compose stack (`docker-compose.arm64.yml`: Postgres + `pgvector`, Valkey, NATS JetStream, MinIO, `local-git-mock`, `mock-sftp`).
- `./signal.sh dev k8s` — Single-node local Kubernetes (`OrbStack`/`Kind`) deploying `infra/k8s/helm/signal/values-local.yaml`.

## Security Invariants (`F-095`, `F-106`, `F-143`)
1. `dev-guest-auth` is compile-time blocked in `--release` Rust builds (`compile_error!`) and runtime-blocked when `SIGNAL_ENV=production`.
2. `seeds/guest_signal_design_inc.json` seeds both `local-guest-0001` (full access) and `local-pcb-contractor-0002` (`PCB`-only) so per-repo RBAC masking, RLS, and Hybrid PQC run identically in local dev.
3. Subsystem `.env.example` templates are split under `infra/local/env/*.env.example` so Antigravity and Cursor never collide on a shared `.env.example`.
