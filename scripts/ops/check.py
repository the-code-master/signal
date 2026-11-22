# Module: scripts/ops/check.py — Unified Monorepo PR & Local Verification Gate
# Invariants: Enforces AGENTS.md (<12 KiB) & Cursor rule (<4 KiB) caps, DLP OpenAPI checks, and component gates.
# Spec: architecture/12-multi-ide-collaboration.md, architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

from pathlib import Path
from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv, select_components

AGENTS_MD_MAX_BYTES = 12_288
CURSOR_CORE_RULE_MAX_BYTES = 4_096
FORBIDDEN_OPENAPI_SUBSTRINGS: tuple[str, ...] = (
    "/export",
    "/download",
    "/print",
    "/raw-file",
)


def _check_governance_invariants(repo_root: Path) -> list[str]:
    errors: list[str] = []
    agents_md = repo_root / "AGENTS.md"
    if agents_md.exists() and agents_md.stat().st_size >= AGENTS_MD_MAX_BYTES:
        errors.append(f"AGENTS.md size {agents_md.stat().st_size} B >= {AGENTS_MD_MAX_BYTES} B limit")

    cursor_rule = repo_root / ".cursor" / "rules" / "signal-architecture.mdc"
    if cursor_rule.exists() and cursor_rule.stat().st_size >= CURSOR_CORE_RULE_MAX_BYTES:
        errors.append(
            f".cursor/rules/signal-architecture.mdc size {cursor_rule.stat().st_size} B >= {CURSOR_CORE_RULE_MAX_BYTES} B"
        )

    openapi_file = repo_root / "contracts" / "openapi" / "control-plane.v1.yaml"
    if openapi_file.exists():
        for idx, line in enumerate(openapi_file.read_text(encoding="utf-8").splitlines(), 1):
            stripped = line.strip()
            if stripped.startswith("#") or not stripped.startswith("/"):
                continue
            for bad in FORBIDDEN_OPENAPI_SUBSTRINGS:
                if bad in stripped.lower():
                    errors.append(f"DLP violation in control-plane.v1.yaml:{idx} -> `{stripped}`")
    return errors


def run_check(
    reg: RepoRegistry,
    *,
    target: str | None = None,
    changed_only: bool = False,
    dry_run: bool = False,
    as_json: bool = False,
) -> int:
    gov_errors = _check_governance_invariants(reg.repo_root)
    if gov_errors:
        emit_output(
            {
                "command": "check",
                "status": "FAILED",
                "summary": "Governance/DLP invariant check failed",
                "details": gov_errors,
            },
            as_json=as_json,
        )
        return 1

    specs = select_components(reg, target=target, changed_only=changed_only)
    details: list[str] = ["Governance caps & Zero-Leak OpenAPI DLP audit: OK"]

    for spec in specs:
        req_file = reg.repo_root / spec.required_path
        if not req_file.exists():
            details.append(f"{spec.name}: SKIPPED_NOT_YET_SCAFFOLDED (`{spec.required_path}` absent)")
            continue
        for cmd in spec.check_commands:
            res = run_argv(cmd, reg.repo_root, dry_run=dry_run)
            details.append(f"{spec.name}: {' '.join(cmd)}")
            if res.returncode != 0:
                emit_output(
                    {
                        "command": "check",
                        "status": "FAILED",
                        "summary": f"Verification failed in `{spec.name}`: {' '.join(cmd)}",
                        "details": details,
                    },
                    as_json=as_json,
                )
                return res.returncode

    payload: dict[str, Any] = {
        "command": "check",
        "status": "OK",
        "version": reg.version,
        "dry_run": dry_run,
        "summary": f"All checks green across {len(specs)} selected component(s)",
        "details": details,
    }
    emit_output(payload, as_json=as_json)
    return 0
