# Module: scripts/tests/test_signal_ops.py — Unit & Dry-Run Verification Suite for signal_ops.py
# Invariants: Compatible with both stdlib unittest and pytest; tests SemVer, next-task gate, worktree, and all CLI verbs.
# Spec: architecture/20-versioning-release-monorepo-build-and-k8s-topology.md
from __future__ import annotations

import os
import sys
import unittest
from pathlib import Path
from unittest.mock import patch

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from scripts.ops.next_task import LEDGER_MAP, parse_ledger  # noqa: E402
from scripts.ops.registry import load_registry, select_components  # noqa: E402
from scripts.ops.release import compute_next_semver  # noqa: E402
from scripts.signal_ops import main  # noqa: E402


class SignalOpsCliTests(unittest.TestCase):
    def test_registry_loads_all_tiers_and_version(self) -> None:
        reg = load_registry(REPO_ROOT)
        self.assertEqual(reg.version, "0.1.0")
        for expected in ("contracts", "rust-pod", "rust-operator", "web", "api", "bridge", "k8s", "ops-cli"):
            self.assertIn(expected, reg.components)
        only_contracts = select_components(reg, target="contracts")
        self.assertEqual([c.name for c in only_contracts], ["contracts"])

    def test_semver_conventional_commit_bump_rules(self) -> None:
        self.assertEqual(compute_next_semver("0.1.0", ["fix(api): uniform 404"]), "0.1.1")
        self.assertEqual(compute_next_semver("0.1.0", ["feat(engine): add saltation matrix"]), "0.2.0")
        self.assertEqual(compute_next_semver("0.1.0", ["feat(proto)!: breaking wire bump"]), "1.0.0")
        self.assertEqual(compute_next_semver("0.1.0", ["chore: docs"], override_bump="major"), "1.0.0")

    def test_next_task_parses_all_three_ledgers_and_phase1_contract_gate(self) -> None:
        c_tasks = parse_ledger(REPO_ROOT / LEDGER_MAP["contracts"])
        ag_tasks = parse_ledger(REPO_ROOT / LEDGER_MAP["ag"])
        cu_tasks = parse_ledger(REPO_ROOT / LEDGER_MAP["cursor"])
        self.assertEqual(len(c_tasks) + len(ag_tasks) + len(cu_tasks), 108)
        self.assertEqual(main(["next", "--lane", "contracts", "--json"]), 0)
        self.assertEqual(main(["next", "--lane", "ag", "--json"]), 0)
        self.assertEqual(main(["next", "--lane", "cursor", "--json"]), 0)

    def test_dev_mode_refuses_production_environment(self) -> None:
        with patch.dict(os.environ, {"SIGNAL_ENV": "production"}):
            rc = main(["dev", "lite", "--dry-run", "--json"])
            self.assertEqual(rc, 1)

    def test_all_verbs_succeed_in_dry_run_mode(self) -> None:
        self.assertEqual(main(["doctor", "--json"]), 0)
        self.assertEqual(main(["bootstrap", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["worktree", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["dev", "run", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["dev", "lite", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["build", "--target", "contracts", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["check", "--target", "contracts", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["image", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["release", "preview", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["deploy", "--env", "local", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["rollback", "--env", "staging", "--to", "v0.1.0", "--dry-run", "--json"]), 0)


if __name__ == "__main__":
    unittest.main()
