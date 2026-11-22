# Module: scripts/tests/test_ga_invariants.py — GA 148-Flaw, 120-Feature, DLP, Transcript, DR & Release Suite
# Invariants: Verifies F-001..F-148 & FEAT-001..FEAT-120, zero OpenAPI DLP leaks, 6 transcripts & CLI dry-runs.
# Spec: architecture/15-application-security-and-project-isolation.md, architecture/16-*
from __future__ import annotations

import json
import re
import sys
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from scripts.ops.check import _check_governance_invariants  # noqa: E402
from scripts.signal_ops import main  # noqa: E402


class GaPlatformReadinessTests(unittest.TestCase):
    def test_all_148_flaws_and_120_features_documented_without_gaps(self) -> None:
        arch_text = "\n".join(
            p.read_text(encoding="utf-8")
            for p in sorted((REPO_ROOT / "architecture").glob("*.md"))
        )
        flaw_nums = {int(m) for m in re.findall(r"\bF-(\d{3})\b", arch_text)}
        feat_nums = {int(m) for m in re.findall(r"\bFEAT-(\d{3})\b", arch_text)}
        self.assertEqual(flaw_nums, set(range(1, 149)))
        self.assertEqual(feat_nums, set(range(1, 121)))

    def test_governance_caps_and_zero_leak_openapi_dlp_clean(self) -> None:
        errors = _check_governance_invariants(REPO_ROOT)
        self.assertEqual(errors, [])

    def test_all_golden_transcripts_and_fixtures_parse_cleanly(self) -> None:
        transcript_files = sorted((REPO_ROOT / "contracts" / "transcripts").glob("*.jsonl"))
        self.assertGreaterEqual(len(transcript_files), 6)
        for tf in transcript_files:
            rows = [
                json.loads(line)
                for line in tf.read_text(encoding="utf-8").splitlines()
                if line.strip()
            ]
            self.assertGreaterEqual(len(rows), 1, f"empty transcript {tf}")
            for idx, row in enumerate(rows, 1):
                self.assertEqual(row["seq"], idx)
                self.assertIn("rpc", row)
                self.assertIn("request", row)

        fixture_files = sorted((REPO_ROOT / "contracts" / "fixtures").glob("*.json"))
        self.assertGreaterEqual(len(fixture_files), 13)
        for ff in fixture_files:
            parsed = json.loads(ff.read_text(encoding="utf-8"))
            self.assertIsInstance(parsed, dict)

    def test_in_jurisdiction_dr_campus_edge_and_release_image_dry_runs(self) -> None:
        dr_file = (
            REPO_ROOT
            / "infra"
            / "k8s"
            / "gitops"
            / "overlays"
            / "prod"
            / "dr-paired-region-policy.yaml"
        )
        self.assertTrue(dr_file.exists())
        dr_text = dr_file.read_text(encoding="utf-8")
        self.assertIn("eu-central-1-frankfurt", dr_text)
        self.assertIn("eu-west-3-paris", dr_text)
        self.assertIn("us-gov-west-1", dr_text)
        self.assertIn("ap-south-2-hyderabad", dr_text)

        campus_chart = REPO_ROOT / "infra" / "campus-edge" / "Chart.yaml"
        self.assertTrue(campus_chart.exists())

        self.assertEqual(main(["release", "preview", "--dry-run", "--json"]), 0)
        self.assertEqual(main(["image", "--dry-run", "--json"]), 0)


if __name__ == "__main__":
    unittest.main()
