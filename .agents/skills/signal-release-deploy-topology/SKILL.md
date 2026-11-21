---
name: signal-release-deploy-topology
description: Builds and maintains Signal's Platform Version Management, Automated Release Notes (CHANGELOG.md, releases/notes/vX.Y.Z.{md,json}, GitHub Releases, In-App What's New), Unified Common Build/CI-CD/Deploy CLI (./signal.sh -> Python 3.15+ stdlib scripts/signal_ops.py + scripts/components.toml, thin GitHub Actions YAML, multi-arch Docker, Helm, Karpenter, GitOps), and 3-Tier K8s 1.37+ Topology (separate Angular v22++Node 26 Web pod, dedicated Python 3.15+ FastAPI/Granian API + Envoy Engine Bridge tier, and 1-4 baseline extensible Rust 1.100+ worker nodes). Use whenever working on versioning, releases, build/deploy scripts, CI/CD, Dockerfiles, Helm charts, Karpenter node pools, or monorepo workflows.
---

# Signal Platform Versioning, Common CI/CD CLI & 3-Tier K8s 1.37+ Topology

Governing specs: `architecture/20-versioning-release-monorepo-build-and-k8s-topology.md`, `architecture/04-kubernetes-runtime-and-collaboration.md`, `architecture/09-local-development-environment.md`.

## 1. Hard 3-Tier Kubernetes 1.37+ Pod & Extensible Node Separation
- **Tier 1 (`pool-web` — Separate Web Pod):** Hosts `signal-web` (**Angular `v22+` ONLY** static/SSR assets + thin **Node.js `26 LTS+`** UI edge for CSP nonces & Route-Partitioned `COOP`/`COEP`). Cilium eBPF restricts egress strictly to `signal-api-gateway` (ZERO path to Rust pods, Postgres, KMS, or BYO-Git).
- **Tier 2 (`pool-api` — Dedicated HA API & Engine Bridge Tier):** Co-locates stateless **Python `3.15+` FastAPI (`Granian 2.8+`)** `signal-api-gateway` & control-plane services (Auth/RBAC/Tickets/PLM/Collab/Release Notes) + native **Envoy `signal-engine-bridge`** (zero-copy HTTP/3 WebTransport/gRPC/Arrow relay over SPIFFE mTLS with Ed25519 ticket check & multi-version sticky routing). Never proxy 120 FPS binary pixel streams through FastAPI JSON handlers!
- **Tier 3 (`pool-rust-engine` — `1–4` Baseline, Elastically Extensible to `N` Worker Nodes):** Tainted `signal.io/workload=rust-engine:NoSchedule`. Schedules isolated per-project **Rust `1.100+`** `signal-pod` (Kata/gVisor MicroVMs) or `< 8 MB` `signal-cell-wasm` cells—preserving "1 Project = 1 Security Cell" and 100% lateral pod drop. Extend horizontally (`1 -> 4 -> N`) or via capability labels (`core-standard`, `core-highmem`, `core-gpu`, `core-confidential`) with adaptive `1`-node fallback and `< 150 ms` Pre-Copy live migration.

## 2. Platform Version Management & Automated Release Notes
- Maintain single canonical SemVer in `VERSION`, synchronized by `./signal.sh release` into `engine/Cargo.toml` (Rust `>= 1.100`), `web/package.json` (Angular `>= 22`, Node `>= 26`), `pyproject.toml` (Python `>= 3.15` FastAPI/AI `uv` workspace), and `infra/k8s/helm/signal/Chart.yaml`.
- Maintain `releases/compatibility.json` (`N-2` Buf wire window, `physics_determinism_epoch`, Expand-and-Contract PostgreSQL `19+` bounds) and support per-project pinned `engine_version` digests in `.signal/platform.lock.json` via multi-version Rust pod coexistence.
- Automate release notes from Conventional Commits + `releases/changesets/*.md` into `CHANGELOG.md`, `releases/notes/vX.Y.Z.md`, `releases/notes/vX.Y.Z.json` (served by `GET /api/v1/platform/releases` to Angular's "What's New" drawer), and signed GitHub Releases with SLSA L3 provenance, Cosign signatures, and SPDX 3.0 / CycloneDX SBOMs.
- Retain `N-3` content-hashed Angular chunks (`72h`) to prevent mid-session `ChunkLoadError`, enforce Expand-and-Contract DB migrations, and fail release builds if binary symbol scans detect `dev-guest-auth`.

## 3. Unified Small-Team Common Build, CI/CD, Deploy & Rollback CLI (`./signal.sh`)
- **Single Entrypoint (`Local == CI`):** ALL local dev, PR checks, builds, multi-arch Docker images, releases, deploys, and rollbacks MUST execute through `./signal.sh` (thin POSIX/Bash shim) → `scripts/signal_ops.py` + modular `scripts/ops/*.py` (`<= 250 LOC` each, **Python `3.15+` stdlib ONLY — zero pip bootstrap deps**). Root `justfile` stays a 1-line alias delegating to `./signal.sh`.
- **Single Declarative Registry (`scripts/components.toml`):** Register EVERY deployable component (`contracts`, `rust-*`, `api`, `ui`, `bridge`, `ai`, `k8s`) in `scripts/components.toml` so adding a service wires up `build`, `check`, `image`, `release`, `deploy`, and `rollback` automatically.
- **Zero Duplicated CI YAML Scripts:** Keep `.github/workflows/{pr-gate,release,deploy-gitops}.yml` thin (`~25–45 LOC`) invoking ONLY `./signal.sh <doctor|dev|build|check|image|release|deploy|rollback>` with `--changed`, `--dry-run`, and `--json`; NEVER bury build/release shell logic inside workflow YAML.
- **Safety & Testing:** Enforce clean git tree + signed tag before `--env staging|prod`, mask secrets, and test all `scripts/ops/*.py` modules via `uv run ruff`, `pyright --strict`, `bandit`, and `pytest scripts/tests/`.

## Dev Response & Anti-Hallucination
- Verify exact `VERSION`, `signal.sh`, `scripts/components.toml`, `justfile`, Helm values, `.github/workflows/`, and `platform_release.proto` fields via `rg`/`view_file` before editing; never claim builds, helm lints, or releases pass without observed exit code `0`.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and verification result.
