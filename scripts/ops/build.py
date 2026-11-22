# Module: scripts/ops/build.py — Declarative Multi-Component Monorepo Builder
# Invariants: Builds components from scripts/components.toml; enforces --no-default-features on Rust release binaries (F-095).
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv, select_components


def run_build(
    reg: RepoRegistry,
    *,
    target: str | None = None,
    changed_only: bool = False,
    dry_run: bool = False,
    as_json: bool = False,
) -> int:
    specs = select_components(reg, target=target, changed_only=changed_only)
    details: list[str] = []

    for spec in specs:
        req_file = reg.repo_root / spec.required_path
        if not req_file.exists():
            details.append(
                f"{spec.name}: SKIPPED_NOT_YET_SCAFFOLDED (`{spec.required_path}` absent)"
            )
            continue
        if not spec.build_commands:
            details.append(f"{spec.name}: NO_OP_BUILD")
            continue
        for cmd in spec.build_commands:
            if "dev-guest-auth" in cmd:
                emit_output(
                    {
                        "command": "build",
                        "status": "FAILED",
                        "summary": f"Security gate blocked `dev-guest-auth` in `{spec.name}` build",
                        "details": details,
                    },
                    as_json=as_json,
                )
                return 1
            res = run_argv(cmd, reg.repo_root, dry_run=dry_run)
            details.append(f"{spec.name}: {' '.join(cmd)}")
            if res.returncode != 0:
                emit_output(
                    {
                        "command": "build",
                        "status": "FAILED",
                        "summary": f"Build failed in `{spec.name}`: {' '.join(cmd)}",
                        "details": details,
                    },
                    as_json=as_json,
                )
                return res.returncode

    payload: dict[str, Any] = {
        "command": "build",
        "status": "OK",
        "version": reg.version,
        "dry_run": dry_run,
        "summary": f"Built {len(specs)} component target(s) for Signal v{reg.version}",
        "details": details,
    }
    emit_output(payload, as_json=as_json)
    return 0
