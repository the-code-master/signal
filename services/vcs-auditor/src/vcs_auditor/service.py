# Module: services/vcs-auditor/src/vcs_auditor/service.py — Multi-Provider BYO-Git Security Posture Scorer (0-100)
# Invariants: Deducts weighted penalties across public exposure, branch protection, secrets, Padmé Merkle sig & expiry.
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md
from __future__ import annotations

from vcs_auditor.repository import fetch_vcs_auditor_record, save_vcs_posture_report
from vcs_auditor.schemas import VcsPostureEvaluateRequest, VcsPostureReport


def compute_posture_report(org_slug: str, req: VcsPostureEvaluateRequest) -> VcsPostureReport:
    if not req.repositories:
        return VcsPostureReport(
            org_slug=org_slug,
            score=100,
            padme_vault_enforced=True,
            public_exposure_blocked=True,
            findings=(),
        )
    score = 100
    findings: list[str] = []
    any_public = any(r.is_public for r in req.repositories)
    any_unprotected = any(not r.branch_protection_enabled for r in req.repositories)
    any_secret_leak = any(not r.secret_scan_clean for r in req.repositories)
    any_padme_broken = any(not r.padme_merkle_sig_valid for r in req.repositories)
    any_expired_contractor = any(r.expired_contractor_count > 0 for r in req.repositories)

    if any_public:
        score -= 25
        findings.append("CRITICAL: Public repository visibility detected on BYO-Git namespace")
    if any_unprotected:
        score -= 20
        findings.append("HIGH: Protected default branch / signed Super-Commit guard disabled")
    if any_secret_leak:
        score -= 20
        findings.append("HIGH: Unredacted credential or token flagged by secret scanner")
    if any_padme_broken:
        score -= 20
        findings.append("HIGH: Missing or tampered .signal/vault.merkle.sig Padmé integrity root")
    if any_expired_contractor:
        score -= 15
        findings.append("MEDIUM: Expired contractor collaborator still present on provider roster")

    return VcsPostureReport(
        org_slug=org_slug,
        score=max(0, min(100, score)),
        padme_vault_enforced=not any_padme_broken,
        public_exposure_blocked=not any_public,
        findings=tuple(findings),
    )


async def execute_vcs_auditor(org_slug: str) -> VcsPostureReport:
    return await fetch_vcs_auditor_record(org_slug)


async def evaluate_and_store_vcs_posture(
    org_slug: str,
    req: VcsPostureEvaluateRequest,
) -> VcsPostureReport:
    report = compute_posture_report(org_slug, req)
    return await save_vcs_posture_report(report)
