# Signal Platform Changelog

All notable platform releases are automatically synthesized from Conventional Commits by `./signal.sh release`.

## [0.1.0] - 2026-11-21

### Added
- **Contracts (`C-P0-01..03`):** Canonical SI-unit `bigint` (`[jstype = JS_STRING]`) Protobuf v1 suite, OpenAPI 3.1 Control Plane (`control-plane.v1.yaml`), 11 Draft 2020-12 JSON Schemas, golden `> 2^53 - 1` fixtures, and stateful session JSONL transcript.
- **Pure-Rust `1.100+` Engine (`AG-P0-01..02`):** 18-crate `#![forbid(unsafe_code)]` Cargo workspace (`signal-proto` single-build codegen, `signal-core` `SecretBox<T>` & `dev-guest-auth` release compile-error guard, `signal-geom` `i128` predicates, `signal-pod`, `signal-operator`, `signal-egress-proxy`, `signal-sftp-egress`, `signal-mcad`, `signal-cam`, `signal-plm`).
- **Unified Ops CLI & 3-Tier K8s Topology (`AG-P0-03..05`):** `./signal.sh` + stdlib Python `3.15+` `scripts/signal_ops.py` + `scripts/components.toml`, 3-Tier K8s `1.37+` Umbrella Helm Chart (`pool-web`, `pool-api`, `1–4 -> N` extensible `pool-rust-engine` worker nodes), and macOS Apple Silicon `< 1.5 GB RAM` `dev:lite` / `dev:full` harness.
