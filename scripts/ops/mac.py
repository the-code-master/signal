# Module: scripts/ops/mac.py — Local macOS Apple Silicon Bootstrap & Dual-IDE Git Worktree Provisioner
# Invariants: Prevents Cursor + Antigravity branch collisions on one Mac via isolated git worktrees; zero shell=True.
# Spec: architecture/09-local-development-environment.md, architecture/12-multi-ide-collaboration.md
from __future__ import annotations

from pathlib import Path
from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv


def run_bootstrap(reg: RepoRegistry, *, dry_run: bool = False, as_json: bool = False) -> int:
    steps: list[list[str]] = [
        ["uv", "sync"],
        ["pnpm", "--dir", "web", "install"],
        ["pnpm", "--dir", "web", "build"],
        ["cargo", "build", "--manifest-path", "engine/Cargo.toml", "--bin", "signal-pod", "--features", "dev-guest-auth"],
    ]
    details: list[str] = []
    for cmd in steps:
        res = run_argv(cmd, reg.repo_root, dry_run=dry_run)
        details.append(f"{'Planned' if dry_run else 'Completed'}: {' '.join(cmd)}")
        if res.returncode != 0:
            emit_output(
                {
                    "command": "bootstrap",
                    "status": "FAILED",
                    "summary": f"Bootstrap step failed: {' '.join(cmd)}",
                    "details": details,
                },
                as_json=as_json,
            )
            return res.returncode

    payload: dict[str, Any] = {
        "command": "bootstrap",
        "status": "OK",
        "dry_run": dry_run,
        "summary": "Local workspace dependencies & 3-tier binaries bootstrapped",
        "details": details,
    }
    emit_output(payload, as_json=as_json)
    return 0


def run_worktree_setup(
    reg: RepoRegistry,
    *,
    peer_dir: str = "../signal-ag",
    dry_run: bool = False,
    as_json: bool = False,
) -> int:
    target_path = (reg.repo_root / peer_dir).resolve()
    cmd = ["git", "worktree", "add", "-B", "ag/workspace", str(target_path), "origin/main"]
    details = [
        f"Cursor Workspace (Lane B `web/`, `services/`): {reg.repo_root}",
        f"Antigravity Workspace (Lane A `engine/`, `infra/`, `contracts/`): {target_path}",
    ]
    if target_path.exists() and not dry_run:
        details.append(f"Worktree path `{target_path}` already exists — skipping creation.")
    else:
        res = run_argv(cmd, reg.repo_root, dry_run=dry_run)
        details.append(f"{'Planned' if dry_run else 'Created'}: {' '.join(cmd)}")
        if res.returncode != 0:
            emit_output(
                {
                    "command": "worktree",
                    "status": "FAILED",
                    "summary": f"Failed to create peer worktree at {target_path}",
                    "details": details,
                },
                as_json=as_json,
            )
            return res.returncode

    payload: dict[str, Any] = {
        "command": "worktree",
        "status": "OK",
        "cursor_dir": str(reg.repo_root),
        "antigravity_dir": str(target_path),
        "dry_run": dry_run,
        "summary": f"Dual-IDE isolated git worktrees ready ({Path(reg.repo_root).name} + {target_path.name})",
        "details": details,
    }
    emit_output(payload, as_json=as_json)
    return 0
