# Module: scripts/ops/dev.py — macOS Apple Silicon & Linux Local Dev Launcher (lite | full | k8s | down)
# Invariants: Refuses execution if SIGNAL_ENV=production (F-095); enforces <1.5 GB RAM dev:lite profile (F-105).
# Spec: architecture/09-local-development-environment.md
from __future__ import annotations

import os
from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv

VALID_MODES: tuple[str, ...] = ("lite", "full", "k8s", "down")


def run_dev(
    reg: RepoRegistry,
    *,
    mode: str = "lite",
    dry_run: bool = False,
    as_json: bool = False,
) -> int:
    if os.environ.get("SIGNAL_ENV", "").lower() in ("prod", "production"):
        emit_output(
            {
                "command": "dev",
                "status": "FAILED",
                "summary": "FATAL (F-095): `./signal.sh dev` is strictly forbidden when SIGNAL_ENV=production",
                "details": [],
            },
            as_json=as_json,
        )
        return 1

    if mode not in VALID_MODES:
        emit_output(
            {
                "command": "dev",
                "status": "FAILED",
                "summary": f"Invalid dev mode `{mode}` (choose from {', '.join(VALID_MODES)})",
                "details": [],
            },
            as_json=as_json,
        )
        return 2

    compose_file = reg.repo_root / "infra" / "local" / "docker-compose.arm64.yml"
    commands: list[list[str]] = []
    notes: list[str] = [
        "Guest Org: Guest @ Signal Design, Inc. (`local-guest-0001`, RBAC + Hybrid PQC enforced)",
        "LLM Mode: Ollama/MLX auto-probe with `deterministic_mock` offline fallback (F-143)",
    ]

    if mode == "lite":
        notes.append(
            "Profile: `dev:lite` (< 1.5 GB RAM; embedded storage + debug `dev-guest-auth` cell)"
        )
        commands.append(
            [
                "cargo",
                "check",
                "--manifest-path",
                "engine/Cargo.toml",
                "-p",
                "signal-pod",
                "--features",
                "dev-guest-auth",
            ]
        )
    elif mode == "full":
        notes.append(
            f"Profile: `dev:full` (native ARM64/multi-arch Compose `{compose_file.relative_to(reg.repo_root)}`)"
        )
        commands.append(["docker", "compose", "-f", str(compose_file), "up", "-d"])
    elif mode == "k8s":
        notes.append(
            "Profile: `dev:k8s` (local 3-Tier Helm deployment using `values-local.yaml`)"
        )
        commands.append(
            [
                "helm",
                "upgrade",
                "--install",
                "signal-local",
                "infra/k8s/helm/signal",
                "-f",
                "infra/k8s/helm/signal/values-local.yaml",
                "--namespace",
                "signal-local",
                "--create-namespace",
            ]
        )
    else:
        notes.append("Stopping local Docker Compose services")
        commands.append(["docker", "compose", "-f", str(compose_file), "down"])

    for cmd in commands:
        res = run_argv(cmd, reg.repo_root, dry_run=dry_run)
        notes.append(f"Executed: {' '.join(cmd)}")
        if res.returncode != 0:
            emit_output(
                {
                    "command": "dev",
                    "status": "FAILED",
                    "summary": f"Command failed: {' '.join(cmd)}",
                    "details": notes,
                },
                as_json=as_json,
            )
            return res.returncode

    payload: dict[str, Any] = {
        "command": "dev",
        "mode": mode,
        "dry_run": dry_run,
        "status": "OK",
        "summary": f"Local dev mode `{mode}` {'planned (dry-run)' if dry_run else 'ready'}",
        "details": notes,
    }
    emit_output(payload, as_json=as_json)
    return 0
