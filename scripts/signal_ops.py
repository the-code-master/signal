#!/usr/bin/env python3
# Module: scripts/signal_ops.py — Canonical Stdlib-Only CLI Dispatcher for Build, Check, Image, Release, Deploy & Rollback
# Invariants: Zero third-party pip bootstrap dependencies; identical behavior across macOS dev and GitHub Actions CI (20-*).
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from scripts.ops.build import run_build
from scripts.ops.check import run_check
from scripts.ops.deploy import run_deploy
from scripts.ops.dev import run_dev
from scripts.ops.doctor import run_doctor
from scripts.ops.image import run_image
from scripts.ops.registry import load_registry
from scripts.ops.release import run_release
from scripts.ops.rollback import run_rollback


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="signal.sh", description="Signal Unified Monorepo Operations CLI"
    )
    sub = parser.add_subparsers(dest="verb", required=True)

    p_doc = sub.add_parser("doctor", help="Verify local/CI toolchain readiness")
    p_doc.add_argument("--json", action="store_true", dest="as_json")

    p_dev = sub.add_parser("dev", help="Launch macOS/Linux local dev environment")
    p_dev.add_argument(
        "mode", nargs="?", default="lite", choices=["lite", "full", "k8s", "down"]
    )
    p_dev.add_argument("--dry-run", action="store_true")
    p_dev.add_argument("--json", action="store_true", dest="as_json")

    p_build = sub.add_parser("build", help="Build monorepo components")
    p_build.add_argument("--target", default=None)
    p_build.add_argument("--changed", action="store_true")
    p_build.add_argument("--dry-run", action="store_true")
    p_build.add_argument("--json", action="store_true", dest="as_json")

    p_check = sub.add_parser("check", help="Run PR/local verification gates")
    p_check.add_argument("--target", default=None)
    p_check.add_argument("--changed", action="store_true")
    p_check.add_argument("--dry-run", action="store_true")
    p_check.add_argument("--json", action="store_true", dest="as_json")

    p_img = sub.add_parser("image", help="Build/sign multi-arch OCI container images")
    p_img.add_argument("--target", default=None)
    p_img.add_argument("--push", action="store_true")
    p_img.add_argument("--sign", action="store_true")
    p_img.add_argument("--dry-run", action="store_true")
    p_img.add_argument("--json", action="store_true", dest="as_json")

    p_rel = sub.add_parser(
        "release", help="Preview or cut SemVer release + automated release notes"
    )
    p_rel.add_argument(
        "action", nargs="?", default="preview", choices=["preview", "cut"]
    )
    p_rel.add_argument("--bump", default=None, choices=["major", "minor", "patch"])
    p_rel.add_argument(
        "--channel", default="STABLE", choices=["STABLE", "LTS", "CANARY"]
    )
    p_rel.add_argument("--dry-run", action="store_true")
    p_rel.add_argument("--json", action="store_true", dest="as_json")

    p_dep = sub.add_parser("deploy", help="Deploy 3-Tier K8s Umbrella Helm Chart")
    p_dep.add_argument("--env", default="local", dest="env_name")
    p_dep.add_argument("--version", default=None, dest="version_override")
    p_dep.add_argument("--dry-run", action="store_true")
    p_dep.add_argument("--json", action="store_true", dest="as_json")

    p_rb = sub.add_parser(
        "rollback", help="Roll back 3-Tier K8s deployment to a prior SemVer"
    )
    p_rb.add_argument("--env", required=True, dest="env_name")
    p_rb.add_argument("--to", required=True, dest="target_version")
    p_rb.add_argument("--dry-run", action="store_true")
    p_rb.add_argument("--json", action="store_true", dest="as_json")

    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    reg = load_registry(REPO_ROOT)

    if args.verb == "doctor":
        return run_doctor(reg, as_json=args.as_json)
    if args.verb == "dev":
        return run_dev(reg, mode=args.mode, dry_run=args.dry_run, as_json=args.as_json)
    if args.verb == "build":
        return run_build(
            reg,
            target=args.target,
            changed_only=args.changed,
            dry_run=args.dry_run,
            as_json=args.as_json,
        )
    if args.verb == "check":
        return run_check(
            reg,
            target=args.target,
            changed_only=args.changed,
            dry_run=args.dry_run,
            as_json=args.as_json,
        )
    if args.verb == "image":
        return run_image(
            reg,
            target=args.target,
            push=args.push,
            sign=args.sign,
            dry_run=args.dry_run,
            as_json=args.as_json,
        )
    if args.verb == "release":
        return run_release(
            reg,
            action=args.action,
            bump=args.bump,
            channel=args.channel,
            dry_run=args.dry_run,
            as_json=args.as_json,
        )
    if args.verb == "deploy":
        return run_deploy(
            reg,
            env_name=args.env_name,
            version_override=args.version_override,
            dry_run=args.dry_run,
            as_json=args.as_json,
        )
    if args.verb == "rollback":
        return run_rollback(
            reg,
            env_name=args.env_name,
            target_version=args.target_version,
            dry_run=args.dry_run,
            as_json=args.as_json,
        )
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
