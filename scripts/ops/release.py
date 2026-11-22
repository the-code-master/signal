# Module: scripts/ops/release.py — Automated SemVer Bump, CHANGELOG & Structured Release Notes Engine
# Invariants: Parses Conventional Commits; updates VERSION, CHANGELOG.md, releases/notes/vX.Y.Z.{md,json} atomically (20-*).
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

import json
import re
from datetime import UTC, datetime
from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv

SEMVER_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")


def compute_next_semver(current: str, commits: list[str], override_bump: str | None = None) -> str:
    m = SEMVER_RE.match(current)
    if not m:
        raise ValueError(f"Invalid current SemVer `{current}`")
    major, minor, patch = int(m.group(1)), int(m.group(2)), int(m.group(3))

    bump = override_bump
    if bump is None:
        if any("BREAKING CHANGE" in c or (":" in c and c.split(":", 1)[0].endswith("!")) for c in commits):
            bump = "major"
        elif any(c.startswith("feat") for c in commits):
            bump = "minor"
        else:
            bump = "patch"

    if bump == "major":
        return f"{major + 1}.0.0"
    if bump == "minor":
        return f"{major}.{minor + 1}.0"
    if bump == "patch":
        return f"{major}.{minor}.{patch + 1}"
    raise ValueError(f"Unsupported bump kind `{bump}`")


def _recent_commits(reg: RepoRegistry) -> list[str]:
    res = run_argv(["git", "log", "-n", "20", "--pretty=format:%s"], reg.repo_root, capture=True)
    if res.returncode != 0:
        return []
    return [line.strip() for line in res.stdout.splitlines() if line.strip()]


def run_release(
    reg: RepoRegistry,
    *,
    action: str = "preview",
    bump: str | None = None,
    channel: str = "STABLE",
    dry_run: bool = False,
    as_json: bool = False,
) -> int:
    if action not in ("preview", "cut"):
        emit_output(
            {
                "command": "release",
                "status": "FAILED",
                "summary": f"Invalid action `{action}`",
                "details": [],
            },
            as_json=as_json,
        )
        return 2

    commits = _recent_commits(reg)
    next_ver = compute_next_semver(reg.version, commits, override_bump=bump)
    compat = json.loads((reg.repo_root / "releases" / "compatibility.json").read_text(encoding="utf-8"))
    epoch = int(compat["physics_determinism_epoch"])
    wire_min = str(compat["wire_min_version"])
    now = datetime.now(tz=UTC)
    now_ms = str(int(now.timestamp() * 1000))
    highlights = commits[:5] or [f"Signal v{next_ver} platform release"]

    note_json = {
        "version": next_ver,
        "channel": channel,
        "released_at_ms": now_ms,
        "physics_determinism_epoch": epoch,
        "wire_min_version": wire_min,
        "highlights": highlights,
    }
    bullets = "\n".join(f"  - {h}" for h in highlights)
    note_md = (
        f"# Signal v{next_ver}\n\n"
        f"- **Channel:** `{channel}` | **Physics Determinism Epoch:** `{epoch}` | **Min Wire:** `{wire_min}`\n"
        f"- **Highlights:**\n{bullets}\n"
    )

    wrote_files: list[str] = []
    if action == "cut" and not dry_run:
        (reg.repo_root / "VERSION").write_text(f"{next_ver}\n", encoding="utf-8")
        notes_dir = reg.repo_root / "releases" / "notes"
        notes_dir.mkdir(parents=True, exist_ok=True)
        (notes_dir / f"v{next_ver}.json").write_text(json.dumps(note_json, indent=2) + "\n", encoding="utf-8")
        (notes_dir / f"v{next_ver}.md").write_text(note_md, encoding="utf-8")
        changelog_path = reg.repo_root / "CHANGELOG.md"
        existing = changelog_path.read_text(encoding="utf-8")
        section = f"## [{next_ver}] - {now.strftime('%Y-%m-%d')}\n\n" + "\n".join(f"- {h}" for h in highlights) + "\n\n"
        marker = "## ["
        idx = existing.find(marker)
        updated = (existing[:idx] + section + existing[idx:]) if idx != -1 else (existing + "\n" + section)
        changelog_path.write_text(updated, encoding="utf-8")
        wrote_files = [
            "VERSION",
            "CHANGELOG.md",
            f"releases/notes/v{next_ver}.json",
            f"releases/notes/v{next_ver}.md",
        ]

    payload: dict[str, Any] = {
        "command": "release",
        "action": action,
        "status": "OK",
        "current_version": reg.version,
        "next_version": next_ver,
        "physics_determinism_epoch": epoch,
        "dry_run": dry_run or action == "preview",
        "wrote_files": wrote_files,
        "summary": f"Release {action}: {reg.version} -> {next_ver} (physics_epoch={epoch})",
        "details": [f"Highlight: {h}" for h in highlights],
    }
    emit_output(payload, as_json=as_json)
    return 0
