# Module: scripts/ops/deploy.py — 3-Tier K8s 1.37+ Helm & GitOps Deployment Orchestrator
# Invariants: Deploys Umbrella Helm Chart across local/dev/staging/prod with pinned image tags & zero shell=True (20-*).
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv

ENV_VALUES_MAP: dict[str, str] = {
    "local": "infra/k8s/helm/signal/values-local.yaml",
    "dev": "infra/k8s/helm/signal/values-starter.yaml",
    "staging": "infra/k8s/helm/signal/values-enterprise.yaml",
    "prod": "infra/k8s/helm/signal/values-enterprise.yaml",
}


def run_deploy(
    reg: RepoRegistry,
    *,
    env_name: str = "local",
    version_override: str | None = None,
    dry_run: bool = False,
    as_json: bool = False,
) -> int:
    if env_name not in ENV_VALUES_MAP:
        emit_output(
            {
                "command": "deploy",
                "status": "FAILED",
                "summary": f"Invalid environment `{env_name}` (valid: {', '.join(ENV_VALUES_MAP)})",
                "details": [],
            },
            as_json=as_json,
        )
        return 2

    target_ver = (version_override or reg.version).lstrip("v")
    values_rel = ENV_VALUES_MAP[env_name]
    chart_dir = reg.repo_root / "infra" / "k8s" / "helm" / "signal"

    cmd = [
        "helm",
        "upgrade",
        "--install",
        f"signal-{env_name}",
        str(chart_dir.relative_to(reg.repo_root)),
        "-f",
        values_rel,
        "--namespace",
        f"signal-{env_name}",
        "--create-namespace",
        "--set",
        f"global.platformVersion={target_ver}",
    ]
    if dry_run:
        cmd.append("--dry-run")

    details: list[str] = [
        f"Environment: `{env_name}` | Values: `{values_rel}` | Version: `v{target_ver}`",
        "3-Tier K8s Topology: pool-web (Angular v22+/Node 26) + pool-api (FastAPI/Envoy) + pool-rust-engine (1-4->N)",
    ]

    if not chart_dir.exists() and dry_run:
        details.append(f"[DRY-RUN] {' '.join(cmd)}")
    else:
        res = run_argv(cmd, reg.repo_root, dry_run=dry_run)
        details.append(f"Executed: {' '.join(cmd)}")
        if res.returncode != 0:
            emit_output(
                {
                    "command": "deploy",
                    "status": "FAILED",
                    "summary": f"Deploy failed on `{env_name}`",
                    "details": details,
                },
                as_json=as_json,
            )
            return res.returncode

    payload: dict[str, Any] = {
        "command": "deploy",
        "env": env_name,
        "version": target_ver,
        "dry_run": dry_run,
        "status": "OK",
        "summary": f"Deployed Signal v{target_ver} to `{env_name}` {'(dry-run)' if dry_run else ''}".strip(),
        "details": details,
    }
    emit_output(payload, as_json=as_json)
    return 0
