# Module: scripts/ops/registry.py — Component Manifest Loader, Git Diff Resolver & Safe Subprocess Runner
# Invariants: Never invokes shell=True; parses scripts/components.toml via stdlib tomllib; caps file size <= 250 LOC.
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

import json
import os
import subprocess
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class ComponentSpec:
    name: str
    tier: str
    required_path: str
    watched_paths: tuple[str, ...]
    check_commands: tuple[tuple[str, ...], ...]
    build_commands: tuple[tuple[str, ...], ...]
    dockerfile: str | None = None
    image_name: str | None = None
    forbid_features: tuple[str, ...] = ()


@dataclass(frozen=True)
class RepoRegistry:
    repo_root: Path
    version: str
    default_registry: str
    components: dict[str, ComponentSpec]


def resolve_repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def load_registry(repo_root: Path | None = None) -> RepoRegistry:
    root = repo_root or resolve_repo_root()
    version = (root / "VERSION").read_text(encoding="utf-8").strip()
    raw = tomllib.loads((root / "scripts" / "components.toml").read_text(encoding="utf-8"))
    comps: dict[str, ComponentSpec] = {}
    for name, cfg in raw.get("components", {}).items():
        comps[name] = ComponentSpec(
            name=name,
            tier=str(cfg["tier"]),
            required_path=str(cfg["required_path"]),
            watched_paths=tuple(str(p) for p in cfg.get("watched_paths", [])),
            check_commands=tuple(tuple(str(x) for x in cmd) for cmd in cfg.get("check_commands", [])),
            build_commands=tuple(tuple(str(x) for x in cmd) for cmd in cfg.get("build_commands", [])),
            dockerfile=str(cfg["dockerfile"]) if "dockerfile" in cfg else None,
            image_name=str(cfg["image_name"]) if "image_name" in cfg else None,
            forbid_features=tuple(str(f) for f in cfg.get("forbid_features", [])),
        )
    return RepoRegistry(
        repo_root=root,
        version=version,
        default_registry=str(raw.get("default_registry", "ghcr.io/the-code-master/signal")),
        components=comps,
    )


def run_argv(
    argv: list[str] | tuple[str, ...],
    cwd: Path,
    *,
    dry_run: bool = False,
    capture: bool = False,
) -> subprocess.CompletedProcess[str]:
    cmd_list = list(argv)
    if dry_run:
        return subprocess.CompletedProcess(cmd_list, 0, stdout=f"[DRY-RUN] {' '.join(cmd_list)}\n", stderr="")
    env = os.environ.copy()
    extra = f"{Path.home() / '.local' / 'bin'}:{Path.home() / '.cargo' / 'bin'}:/opt/homebrew/bin:/opt/cargo/bin"
    env["PATH"] = f"{extra}:{env.get('PATH', '')}"
    try:
        return subprocess.run(
            cmd_list,
            cwd=str(cwd),
            env=env,
            shell=False,
            check=False,
            text=True,
            capture_output=capture,
        )
    except OSError as exc:
        return subprocess.CompletedProcess(cmd_list, 127, stdout="", stderr=str(exc))


def detect_changed_files(repo_root: Path, base_ref: str = "origin/main") -> list[str]:
    res = run_argv(["git", "diff", "--name-only", base_ref], repo_root, capture=True)
    tracked = res.stdout.splitlines() if res.returncode == 0 else []
    status = run_argv(["git", "status", "--porcelain"], repo_root, capture=True)
    untracked = [line[3:].strip() for line in status.stdout.splitlines() if len(line) > 3]
    merged = sorted({p for p in (tracked + untracked) if p})
    return merged


def select_components(
    reg: RepoRegistry,
    *,
    target: str | None = None,
    changed_only: bool = False,
    base_ref: str = "origin/main",
) -> list[ComponentSpec]:
    if target and target != "all":
        resolved: list[ComponentSpec] = []
        for raw_part in target.split(","):
            name = "web" if raw_part.strip() == "ui" else raw_part.strip()
            if not name:
                continue
            if name not in reg.components:
                valid = ", ".join(sorted(reg.components))
                raise ValueError(f"Unknown component target `{name}` (valid: {valid})")
            resolved.append(reg.components[name])
        return resolved
    if not changed_only:
        return list(reg.components.values())

    changed = detect_changed_files(reg.repo_root, base_ref=base_ref)
    if not changed:
        return list(reg.components.values())

    selected: list[ComponentSpec] = []
    for spec in reg.components.values():
        if any(ch == wp.rstrip("/") or ch.startswith(wp) for ch in changed for wp in spec.watched_paths):
            selected.append(spec)
    return selected or [reg.components["ops-cli"]]


def emit_output(payload: dict[str, Any], *, as_json: bool) -> None:
    if as_json:
        sys.stdout.write(json.dumps(payload, indent=2) + "\n")
    else:
        status = payload.get("status", "OK")
        summary = payload.get("summary", "")
        sys.stdout.write(f"[{status}] {summary}\n")
        for line in payload.get("details", []):
            sys.stdout.write(f"  - {line}\n")
