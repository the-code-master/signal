# Module: scripts/ops/image.py — Multi-Arch OCI Image Builder, SBOM & Cosign Signing Orchestrator
# Invariants: Builds linux/amd64,linux/arm64 images with --build-arg SIGNAL_FORBID_DEV_AUTH=1; signs via Cosign (20-*).
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

from typing import Any

from scripts.ops.registry import RepoRegistry, emit_output, run_argv, select_components


def run_image(
    reg: RepoRegistry,
    *,
    target: str | None = None,
    push: bool = False,
    sign: bool = False,
    dry_run: bool = False,
    as_json: bool = False,
) -> int:
    specs = [
        s
        for s in select_components(reg, target=target)
        if s.dockerfile and s.image_name
    ]
    details: list[str] = []

    for spec in specs:
        assert spec.dockerfile is not None and spec.image_name is not None
        df_path = reg.repo_root / spec.dockerfile
        if not df_path.exists():
            details.append(
                f"{spec.name}: SKIPPED_MISSING_DOCKERFILE (`{spec.dockerfile}`)"
            )
            continue
        image_ref = f"{reg.default_registry}/{spec.image_name}:v{reg.version}"
        build_cmd = [
            "docker",
            "buildx",
            "build",
            "--platform",
            "linux/amd64,linux/arm64",
            "--file",
            spec.dockerfile,
            "--build-arg",
            f"SIGNAL_VERSION={reg.version}",
            "--build-arg",
            "SIGNAL_FORBID_DEV_AUTH=1",
            "--tag",
            image_ref,
        ]
        if push:
            build_cmd.append("--push")
        build_cmd.append(".")

        res = run_argv(build_cmd, reg.repo_root, dry_run=dry_run)
        details.append(f"{spec.name}: {' '.join(build_cmd)}")
        if res.returncode != 0:
            emit_output(
                {
                    "command": "image",
                    "status": "FAILED",
                    "summary": f"Image build failed: {spec.name}",
                    "details": details,
                },
                as_json=as_json,
            )
            return res.returncode

        if sign:
            sign_cmd = ["cosign", "sign", "--yes", image_ref]
            sres = run_argv(sign_cmd, reg.repo_root, dry_run=dry_run)
            details.append(f"{spec.name}: {' '.join(sign_cmd)}")
            if sres.returncode != 0:
                return sres.returncode

    payload: dict[str, Any] = {
        "command": "image",
        "status": "OK",
        "version": reg.version,
        "dry_run": dry_run,
        "summary": f"Processed {len(specs)} multi-arch OCI image target(s) for v{reg.version}",
        "details": details,
    }
    emit_output(payload, as_json=as_json)
    return 0
