---
name: signal-release-deploy-topology
description: Builds and maintains Signal's Platform Version Management, Automated Release Notes (CHANGELOG.md, releases/notes/vX.Y.Z.{md,json}, GitHub Releases, In-App What's New), Monorepo-Native Build/Deploy/Local Run (justfile, multi-arch Docker, Helm, Karpenter, GitOps), and 3-Tier K8s 1.37+ Topology (separate Angular v22++Node 26 Web pod, dedicated Python 3.15+ FastAPI/Granian API + Envoy Engine Bridge tier, and 1-4 baseline extensible Rust 1.100+ worker nodes). Use whenever working on versioning, releases, CI/CD, Dockerfiles, Helm charts, Karpenter node pools, or monorepo build/deploy workflows.
---

# Signal Platform Versioning, Automated Releases & 3-Tier K8s 1.37+ Topology

Governing specs: `architecture/20-versioning-release-monorepo-build-and-k8s-topology.md`, `architecture/04-kubernetes-runtime-and-collaboration.md`, `architecture/09-local-development-environment.md`.

## 1. Hard 3-Tier Kubernetes 1.37+ Pod & Extensible Node Separation
- **Tier 1 (`pool-web` — Separate Web Pod):** Hosts `signal-web` (**Angular `v22+` ONLY** static/SSR assets + thin **Node.js `26 LTS+`** UI edge for CSP nonces & Route-Partitioned `COOP`/`COEP`). Cilium eBPF restricts egress strictly to `signal-api-gateway` (ZERO path to Rust pods, Postgres, KMS, or BYO-Git).
- **Tier 2 (`pool-api` — Dedicated HA API & Engine Bridge Tier):** Co-locates stateless **Python `3.15+` FastAPI (`Granian 2.8+`)** `signal-api-gateway` & control-plane services (Auth/RBAC/Tickets/PLM/Collab/Release Notes) + native **Envoy `signal-engine-bridge`** (zero-copy HTTP/3 WebTransport/gRPC/Arrow relay over SPIFFE mTLS with Ed25519 ticket check & multi-version sticky routing). Never proxy 120 FPS binary pixel streams through FastAPI JSON handlers!
- **Tier 3 (`pool-rust-engine` — `1–4` Baseline, Elastically Extensible to `N` Worker Nodes):** Tainted `signal.io/workload=rust-engine:NoSchedule`. Schedules isolated per-project **Rust `1.100+`** `signal-pod` (Kata/gVisor MicroVMs) or `< 8 MB` `signal-cell-wasm` cells—preserving "1 Project = 1 Security Cell" and 100% lateral pod drop. Extend horizontally (`1 -> 4 -> N`) or via capability labels (`core-standard`, `core-highmem`, `core-gpu`, `core-confidential`) with adaptive `1`-node fallback and `< 150 ms` Pre-Copy live migration.

## 2. Platform Version Management & Automated Release Notes
- Maintain single canonical SemVer in `VERSION`, synchronized by CI into `engine/Cargo.toml` (Rust `>= 1.100`), `web/package.json` (Angular `>= 22`, Node `>= 26`), `pyproject.toml` (Python `>= 3.15` FastAPI/AI `uv` workspace), and `infra/k8s/helm/signal/Chart.yaml`.
- Maintain `releases/compatibility.json` (`N-2` Buf wire window, `physics_determinism_epoch`, Expand-and-Contract PostgreSQL `19+` bounds) and support per-project pinned `engine_version` digests in `.signal/platform.lock.json` via multi-version Rust pod coexistence.
- Automate release notes from Conventional Commits + `releases/changesets/*.md` into `CHANGELOG.md`, `releases/notes/vX.Y.Z.md`, `releases/notes/vX.Y.Z.json` (served by `GET /api/v1/platform/releases` to Angular's "What's New" drawer), and signed GitHub Releases with SLSA L3 provenance, Cosign signatures, and SPDX 3.0 / CycloneDX SBOMs.
- Retain `N-3` content-hashed Angular chunks (`72h`) to prevent mid-session `ChunkLoadError`, enforce Expand-and-Contract DB migrations, and fail release builds if binary symbol scans detect `dev-guest-auth`.

## 3. Monorepo-Native Default Build, Local Run & Deploy
- Keep all default build, test, local run, release, and deploy targets inside the monorepo via root `justfile` (`just dev`, `just dev-k8s`, `just build`, `just build-images`, `just test`, `just release-preview`, `just deploy-local-k8s`, `just deploy <env>`), multi-arch (`amd64`/`arm64`) `infra/docker/*`, unified Helm chart `infra/k8s/helm/signal/` (`values-{local,starter,enterprise}.yaml`), Karpenter pools, and digest-pinned GitOps overlays.

## Dev Response & Anti-Hallucination
- Verify exact `VERSION`, `justfile`, Helm values, `.github/workflows/`, and `platform_release.proto` fields via `rg`/`view_file` before editing; never claim builds, helm lints, or releases pass without observed exit code `0`.
- Reply in **2–3 sentences max** stating ONLY the issue, exact fix applied, and verification result.
