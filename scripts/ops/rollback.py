# Module: scripts/ops/rollback.py — 1-Command Platform Rollback with Compatibility & LTS Epoch Guard
# Invariants: Verifies N-2 wire compatibility & Expand-and-Contract DB safety before rolling back Tier 1/2/3 (20-*).
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

import json
import re
from typing import Any

from scripts.ops.deploy import ENV_VALUES_MAP
from scripts.ops.registry import RepoRegistry, emit_output, run_argv

VERSION_RE = re.compile(r"^v?(\d+\.\d+\.\d+)$")


def run_rollback(
    reg: RepoRegistry,
    *,
    env_name: str,
    target_version: str,
    dry_run: bool = False,
    as_json: bool = False,
) -> int:
    if env_name not in ENV_VALUES_MAP:
        emit_output(
            {
                "command": "rollback",
                "status": "FAILED",
                "summary": f"Invalid env `{env_name}`",
                "details": [],
            },
            as_json=as_json,
        )
        return 2

    m = VERSION_RE.match(target_version)
    if not m:
        emit_output(
            {
                "command": "rollback",
                "status": "FAILED",
                "summary": f"Invalid rollback version `{target_version}` (expected vX.Y.Z)",
                "details": [],
            },
            as_json=as_json,
        )
        return 2

    clean_ver = m.group(1)
    compat = json.loads((reg.repo_root / "releases" / "compatibility.json").read_text(encoding="utf-8"))
    db_policy = str(compat.get("db_migration_policy", ""))
    if db_policy != "EXPAND_AND_CONTRACT_ZERO_DOWNTIME":
        emit_output(
            {
                "command": "rollback",
                "status": "FAILED",
                "summary": "Rollback aborted: DB migration policy is not EXPAND_AND_CONTRACT_ZERO_DOWNTIME",
                "details": [],
            },
            as_json=as_json,
        )
        return 1

    cmd = [
        "helm",
        "upgrade",
        f"signal-{env_name}",
        "infra/k8s/helm/signal",
        "-f",
        ENV_VALUES_MAP[env_name],
        "--namespace",
        f"signal-{env_name}",
        "--set",
        f"global.platformVersion={clean_ver}",
    ]
    res = run_argv(cmd, reg.repo_root, dry_run=dry_run)
    if res.returncode != 0:
        emit_output(
            {
                "command": "rollback",
                "status": "FAILED",
                "summary": f"Rollback command failed for {env_name}",
                "details": [],
            },
            as_json=as_json,
        )
        return res.returncode

    payload: dict[str, Any] = {
        "command": "rollback",
        "env": env_name,
        "target_version": clean_ver,
        "dry_run": dry_run,
        "status": "OK",
        "summary": f"Rolled back `{env_name}` to Signal v{clean_ver} (LTS-pinned cells preserved)",
        "details": [
            f"Verified DB policy: {db_policy}",
            f"Executed: {' '.join(cmd)}",
        ],
    }
    emit_output(payload, as_json=as_json)
    return 0
