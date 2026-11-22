# Module: services/vcs-auditor/src/vcs_auditor/schemas.py — Multi-Provider BYO-Git Security Posture Schemas
# Invariants: StrictSchema (extra="forbid", frozen=True); scores bounded 0..100 across 5 cryptographic/access gates.
# Spec: architecture/13-vcs-git-server-and-storage-analysis.md, architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

from typing import Literal

from services.common.schemas import StrictSchema

VcsProviderAuditKind = Literal["GITHUB", "GITLAB", "BITBUCKET"]


class VcsRepoAuditSnapshot(StrictSchema):
    provider_repo_id: str
    provider: VcsProviderAuditKind
    is_public: bool
    branch_protection_enabled: bool
    secret_scan_clean: bool
    padme_merkle_sig_valid: bool
    expired_contractor_count: int


class VcsPostureEvaluateRequest(StrictSchema):
    repositories: tuple[VcsRepoAuditSnapshot, ...]


class VcsPostureReport(StrictSchema):
    org_slug: str
    score: int
    padme_vault_enforced: bool
    public_exposure_blocked: bool = True
    findings: tuple[str, ...] = ()
