# Module: services/plm-ops/src/plm_ops/router.py — Fail-Closed URL-Scoped PLM & Supply-Chain FastAPI Router
# Invariants: Explicit Depends(require_repo_access("local_gh_repo_1005")) + uniform 404 + zero /export paths (F-084).
# Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from plm_ops.schemas import (
    BomLineDto,
    BomViewKind,
    CtbEvaluationRequest,
    CtbReportView,
    DeviationCreateRequest,
    DeviationGateCheckResult,
    DeviationWaiverView,
    EcoCreateRequest,
    EcoRecordView,
    EcoTransitionRequest,
    ErpSyncEnvelope,
    ErpSyncRequest,
    McnCreateRequest,
    McnRecordView,
    MultiViewBomRollup,
    NcrCapaCreateRequest,
    NcrCapaView,
    PcnImpactView,
    PcnIngestRequest,
    PlmChangeNoticeSummary,
)
from plm_ops.service import (
    add_bom_line_item,
    build_signed_erp_sync_envelope,
    compile_multi_view_bom,
    create_change_order,
    create_deviation_waiver,
    create_manufacturing_change_notice,
    create_ncr_capa_record,
    evaluate_clear_to_build,
    execute_plm_ops,
    ingest_part_change_notice,
    transition_change_order,
    verify_deviation_sftp_gate,
)
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access

router = APIRouter()
_REPO_GUARD = require_repo_access("local_gh_repo_1005")


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/change-notices",
    response_model=PlmChangeNoticeSummary,
)
async def handle_plm_ops(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> PlmChangeNoticeSummary:
    _ = (org_slug, proj_slug)
    return await execute_plm_ops(principal.org_slug)


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/boms/{view_kind}",
    response_model=MultiViewBomRollup,
)
async def handle_get_bom_view(
    org_slug: str,
    proj_slug: str,
    view_kind: BomViewKind,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> MultiViewBomRollup:
    _ = org_slug
    return await compile_multi_view_bom(principal.org_slug, proj_slug, view_kind)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/boms/{view_kind}/lines",
    response_model=MultiViewBomRollup,
)
async def handle_add_bom_line(
    org_slug: str,
    proj_slug: str,
    view_kind: BomViewKind,
    payload: BomLineDto,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> MultiViewBomRollup:
    _ = org_slug
    return await add_bom_line_item(principal.org_slug, proj_slug, view_kind, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/ecos",
    response_model=EcoRecordView,
)
async def handle_create_eco(
    org_slug: str,
    proj_slug: str,
    payload: EcoCreateRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> EcoRecordView:
    _ = org_slug
    return await create_change_order(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/ecos/{change_id}/transitions",
    response_model=EcoRecordView,
)
async def handle_transition_eco(
    org_slug: str,
    proj_slug: str,
    change_id: str,
    payload: EcoTransitionRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> EcoRecordView:
    _ = org_slug
    return await transition_change_order(principal.org_slug, proj_slug, change_id, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/mcns",
    response_model=McnRecordView,
)
async def handle_create_mcn(
    org_slug: str,
    proj_slug: str,
    payload: McnCreateRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> McnRecordView:
    _ = org_slug
    return await create_manufacturing_change_notice(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/pcns",
    response_model=PcnImpactView,
)
async def handle_ingest_pcn(
    org_slug: str,
    proj_slug: str,
    payload: PcnIngestRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> PcnImpactView:
    _ = org_slug
    return await ingest_part_change_notice(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/deviations",
    response_model=DeviationWaiverView,
)
async def handle_create_deviation(
    org_slug: str,
    proj_slug: str,
    payload: DeviationCreateRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> DeviationWaiverView:
    _ = org_slug
    return await create_deviation_waiver(principal.org_slug, proj_slug, payload)


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/deviations/gate-check",
    response_model=DeviationGateCheckResult,
)
async def handle_check_deviation_gate(
    org_slug: str,
    proj_slug: str,
    requested_qty: int = 1,
    now_utc_ms: int = 1_795_324_800_000,
    principal: PrincipalContext = Depends(_REPO_GUARD),
) -> DeviationGateCheckResult:
    _ = org_slug
    return verify_deviation_sftp_gate(principal.org_slug, proj_slug, requested_qty, now_utc_ms)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/ncrs",
    response_model=NcrCapaView,
)
async def handle_create_ncr(
    org_slug: str,
    proj_slug: str,
    payload: NcrCapaCreateRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> NcrCapaView:
    _ = org_slug
    return await create_ncr_capa_record(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/ctb-evaluations",
    response_model=CtbReportView,
)
async def handle_evaluate_ctb(
    org_slug: str,
    proj_slug: str,
    payload: CtbEvaluationRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> CtbReportView:
    _ = org_slug
    return await evaluate_clear_to_build(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/plm/erp-sync",
    response_model=ErpSyncEnvelope,
)
async def handle_erp_sync(
    org_slug: str,
    proj_slug: str,
    payload: ErpSyncRequest,
    principal: PrincipalContext = Depends(_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> ErpSyncEnvelope:
    _ = org_slug
    return await build_signed_erp_sync_envelope(principal.org_slug, proj_slug, payload)
