#!/usr/bin/env bash
# Module: signal.sh — Unified Monorepo Build, CI/CD, Release, Deploy & Rollback Entrypoint
# Invariants: Local == CI; zero duplicated shell logic; delegates 100% to stdlib Python 3.15+ scripts/signal_ops.py.
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export PATH="${HOME}/.local/bin:/opt/cargo/bin:${PATH}"

if command -v python3.15 >/dev/null 2>&1; then
  exec python3.15 "${REPO_ROOT}/scripts/signal_ops.py" "$@"
elif command -v uv >/dev/null 2>&1 && uv python find 3.15 >/dev/null 2>&1; then
  exec uv run --no-project --python 3.15 python "${REPO_ROOT}/scripts/signal_ops.py" "$@"
elif command -v python3 >/dev/null 2>&1; then
  exec python3 "${REPO_ROOT}/scripts/signal_ops.py" "$@"
else
  echo "ERROR: Python 3.15+ (or python3>=3.12 fallback) is required to run ./signal.sh" >&2
  exit 127
fi
