# Module: services/audit-service/src/audit_service/router.py — Fail-Closed Enterprise Compliance & GDPR Router
# Invariants: 100% Depends(require_org_admin) guarded (uniform 404 on non-admin); zero /export paths (F-084, F-100).
# Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

from fastapi import APIRouter, Depends, Request

from audit_service.compliance import generate_compliance_attestation
from audit_service.schemas import (
    EnterpriseComplianceAttestationView,
    GdprSaltShredReceipt,
    GdprSaltShredRequest,
)
from audit_service.shredder import GDPR_SALT_VAULT, GLOBAL_WORM_OUTBOX
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_admin, require_step_up_2fa

router = APIRouter()


@router.get(
    "/api/v1/orgs/{org_slug}/compliance/attestations",
    response_model=EnterpriseComplianceAttestationView,
)
async def handle_get_compliance_attestations(
    org_slug: str,
    request: Request,
    principal: PrincipalContext = Depends(require_org_admin),
) -> EnterpriseComplianceAttestationView:
    return await generate_compliance_attestation(
        request.app,
        org_slug=org_slug,
        org_id=principal.org_id,
        actor_hmac=principal.actor_hmac,
    )


@router.post(
    "/api/v1/orgs/{org_slug}/compliance/gdpr-shred",
    response_model=GdprSaltShredReceipt,
)
async def handle_gdpr_salt_shred(
    org_slug: str,
    payload: GdprSaltShredRequest,
    principal: PrincipalContext = Depends(require_org_admin),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
    _idem: str = Depends(require_idempotency_key),
) -> GdprSaltShredReceipt:
    _ = org_slug
    return GDPR_SALT_VAULT.shred_subject_salt(
        org_id=principal.org_id,
        provider_user_id=payload.subject_provider_user_id,
        erasure_ticket_ref=payload.erasure_ticket_ref,
        worm_outbox=GLOBAL_WORM_OUTBOX,
    )
