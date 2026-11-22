# Module: services/vcs-auditor/src/vcs_auditor/repository.py — Tenant-Isolated VCS Posture Snapshot Store
# Invariants: Stores latest evaluated VcsPostureReport per org_slug without cross-tenant leakage.
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md
from __future__ import annotations

from vcs_auditor.schemas import VcsPostureReport

_POSTURE_STORE: dict[str, VcsPostureReport] = {}


async def fetch_vcs_auditor_record(org_slug: str) -> VcsPostureReport:
    return _POSTURE_STORE.get(
        org_slug,
        VcsPostureReport(
            org_slug=org_slug,
            score=100,
            padme_vault_enforced=True,
            public_exposure_blocked=True,
            findings=(),
        ),
    )


async def save_vcs_posture_report(report: VcsPostureReport) -> VcsPostureReport:
    _POSTURE_STORE[report.org_slug] = report
    return report
