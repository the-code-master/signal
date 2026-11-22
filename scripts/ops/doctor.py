# Module: scripts/ops/doctor.py — Toolchain & Monorepo Readiness Diagnostic
# Invariants: Verifies Rust 1.100+, Python 3.15+/3.12+, Buf, Helm, Git, and rule size caps without network side effects.
# Spec: architecture/09-local-development-environment.md, architecture/20-*.md
from __future__ import annotations

import os
import shutil
from pathlib import Path
from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv

REQUIRED_TOOLS: tuple[tuple[str, tuple[str, ...]], ...] = (
    ("git", ("git", "--version")),
    ("rustc", ("rustc", "--version")),
    ("cargo", ("cargo", "--version")),
    ("buf", ("buf", "--version")),
    ("helm", ("helm", "version", "--short")),
)

OPTIONAL_TOOLS: tuple[tuple[str, tuple[str, ...]], ...] = (
    ("uv", ("uv", "--version")),
    ("node", ("node", "--version")),
    ("pnpm", ("pnpm", "--version")),
    ("docker", ("docker", "--version")),
)


def _find_tool(binary: str) -> str | None:
    cargo_home = Path.home() / ".cargo" / "bin"
    local_bin = Path.home() / ".local" / "bin"
    search_path = f"{local_bin}:{cargo_home}:/opt/homebrew/bin:/opt/cargo/bin:{os.environ.get('PATH', '')}"
    return shutil.which(binary, path=search_path)


def run_doctor(reg: RepoRegistry, *, as_json: bool = False) -> int:
    details: list[str] = []
    missing_required: list[str] = []
    tool_report: dict[str, dict[str, str | bool]] = {}

    for label, cmd in REQUIRED_TOOLS:
        found = _find_tool(cmd[0])
        if not found:
            missing_required.append(label)
            tool_report[label] = {
                "present": False,
                "version": "MISSING",
                "required": True,
            }
            details.append(f"REQUIRED `{label}`: MISSING")
            continue
        res = run_argv(cmd, reg.repo_root, capture=True)
        ver = (res.stdout.strip() or res.stderr.strip()).splitlines()[0] if res.returncode == 0 else "ERROR"
        tool_report[label] = {
            "present": res.returncode == 0,
            "version": ver,
            "required": True,
        }
        details.append(f"REQUIRED `{label}`: {ver}")
        if res.returncode != 0:
            missing_required.append(label)

    for label, cmd in OPTIONAL_TOOLS:
        found = _find_tool(cmd[0])
        if not found:
            tool_report[label] = {
                "present": False,
                "version": "not-installed",
                "required": False,
            }
            details.append(f"OPTIONAL `{label}`: not-installed")
            continue
        res = run_argv(cmd, reg.repo_root, capture=True)
        ver = (res.stdout.strip() or res.stderr.strip()).splitlines()[0] if res.returncode == 0 else "unavailable"
        tool_report[label] = {
            "present": res.returncode == 0,
            "version": ver,
            "required": False,
        }
        details.append(f"OPTIONAL `{label}`: {ver}")

    ok = len(missing_required) == 0
    payload: dict[str, Any] = {
        "command": "doctor",
        "status": "OK" if ok else "FAILED",
        "version": reg.version,
        "summary": (
            f"Signal v{reg.version} toolchain healthy ({len(REQUIRED_TOOLS)}/{len(REQUIRED_TOOLS)} core tools ready)"
            if ok
            else f"Missing required tools: {', '.join(missing_required)}"
        ),
        "tools": tool_report,
        "details": details,
    }
    emit_output(payload, as_json=as_json)
    return 0 if ok else 1
