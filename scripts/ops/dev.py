# Module: scripts/ops/dev.py — macOS Apple Silicon & Linux Local Dev Launcher (run | lite | full | k8s | down)
# Invariants: Blocks SIGNAL_ENV=production (F-095); `run` launches live 3-Tier stack with Ctrl+C teardown.
# Spec: architecture/09-local-development-environment.md
from __future__ import annotations

import os
import signal
import subprocess
import time
from pathlib import Path
from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv

VALID_MODES: tuple[str, ...] = ("run", "lite", "full", "k8s", "down")


def _pythonpath_for_services(repo_root: Path) -> str:
    svc_dirs = [str(p) for p in sorted((repo_root / "services").glob("*/src"))]
    return ":".join([str(repo_root), *svc_dirs])


def _run_live_three_tier_stack(reg: RepoRegistry, *, dry_run: bool, as_json: bool) -> int:
    pod_bin = reg.repo_root / "engine" / "target" / "debug" / "signal-pod"
    cmds: list[tuple[str, list[str]]] = [
        ("tier3-rust-cell", [str(pod_bin)]),
        (
            "tier2-fastapi-granian",
            ["uv", "run", "granian", "--interface", "asgi", "--host", "127.0.0.1", "--port", "8080", "api_gateway.main:app"],
        ),
        ("tier1-angular-edge", ["node", "web/apps/signal-web/server/edge-server.mjs"]),
    ]
    if dry_run:
        emit_output(
            {
                "command": "dev",
                "mode": "run",
                "dry_run": True,
                "status": "OK",
                "summary": "Live 3-Tier local stack planned (http://127.0.0.1:4200 -> :8080 -> :50051)",
                "details": [f"{name}: {' '.join(argv)}" for name, argv in cmds],
            },
            as_json=as_json,
        )
        return 0

    if not pod_bin.exists():
        pod_cmd = ["cargo", "build", "--manifest-path", "engine/Cargo.toml", "--bin", "signal-pod"]
        build_res = run_argv([*pod_cmd, "--features", "dev-guest-auth"], reg.repo_root)
        if build_res.returncode != 0:
            return build_res.returncode

    if not (reg.repo_root / "web" / "dist" / "signal-web" / "browser" / "index.html").exists():
        web_res = run_argv(["pnpm", "--dir", "web", "build"], reg.repo_root)
        if web_res.returncode != 0:
            return web_res.returncode

    env = os.environ.copy()
    env["PATH"] = f"{Path.home() / '.local' / 'bin'}:/opt/cargo/bin:{env.get('PATH', '')}"
    env["PYTHONPATH"] = _pythonpath_for_services(reg.repo_root)

    procs: list[subprocess.Popen[str]] = []
    try:
        for _name, argv in cmds:
            procs.append(subprocess.Popen(argv, cwd=str(reg.repo_root), env=env, text=True))
        print("\n[OK] Signal 3-Tier Stack running locally -> http://127.0.0.1:4200 (Press Ctrl+C to stop)\n", flush=True)
        while all(p.poll() is None for p in procs):
            time.sleep(0.5)
        return next((p.returncode or 1 for p in procs if p.poll() is not None), 0)
    except KeyboardInterrupt:
        return 0
    finally:
        for p in procs:
            if p.poll() is None:
                p.send_signal(signal.SIGTERM)
        for p in procs:
            try:
                p.wait(timeout=3)
            except subprocess.TimeoutExpired:
                p.kill()


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

    if mode == "run":
        return _run_live_three_tier_stack(reg, dry_run=dry_run, as_json=as_json)

    compose_file = reg.repo_root / "infra" / "local" / "docker-compose.arm64.yml"
    commands: list[list[str]] = []
    notes: list[str] = [
        "Guest Org: Guest @ Signal Design, Inc. (`local-guest-0001`, RBAC + Hybrid PQC enforced)",
        "LLM Mode: Ollama/MLX auto-probe with `deterministic_mock` offline fallback (F-143)",
    ]

    if mode == "lite":
        notes.append("Profile: `dev:lite` (< 1.5 GB RAM; embedded storage + debug `dev-guest-auth` cell)")
        commands.append(
            ["cargo", "check", "--manifest-path", "engine/Cargo.toml", "-p", "signal-pod", "--features", "dev-guest-auth"]
        )
    elif mode == "full":
        notes.append(f"Profile: `dev:full` (native ARM64/multi-arch Compose `{compose_file.relative_to(reg.repo_root)}`)")
        commands.append(["docker", "compose", "-f", str(compose_file), "up", "-d"])
    elif mode == "k8s":
        notes.append("Profile: `dev:k8s` (local 3-Tier Helm deployment using `values-local.yaml`)")
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
                {"command": "dev", "status": "FAILED", "summary": f"Command failed: {' '.join(cmd)}", "details": notes},
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
