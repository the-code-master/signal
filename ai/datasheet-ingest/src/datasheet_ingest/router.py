# Module: ai/datasheet-ingest/src/datasheet_ingest/router.py — Fail-Closed FastAPI Router for Quarantined Datasheet Ingest
# Invariants: Explicit Depends(require_org_member) + startup assert_all_routes_guarded; zero raw untrusted output (F-090).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/FASTAPI_ENGINEERING_GUIDE.md
from __future__ import annotations

from fastapi import APIRouter, Depends, FastAPI

from datasheet_ingest.consensus_curves import reconcile_dual_vlm_extractions
from datasheet_ingest.glossary_translator import translate_quarantined_technical_text
from datasheet_ingest.ref_circuit import reconstruct_typical_application_testbench
from datasheet_ingest.revision_diff import diff_datasheet_revisions
from datasheet_ingest.schemas import (
    DatasheetRecordAst,
    DatasheetRevisionDiff,
    DatasheetTestbenchPlan,
    DualVlmConsensusReport,
    GlossaryTranslationResult,
    StrictSchema,
)
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import assert_all_routes_guarded, require_org_member

router = APIRouter()


class DualVlmIngestRequest(StrictSchema):
    vlm_a: DatasheetRecordAst
    vlm_b: DatasheetRecordAst


class RevisionDiffApiRequest(StrictSchema):
    old_record: DatasheetRecordAst
    new_record: DatasheetRecordAst


class GlossaryTranslateApiRequest(StrictSchema):
    source_text: str
    target_locale: str


@router.post(
    "/api/v1/orgs/{org_slug}/ai/datasheets/consensus",
    response_model=DualVlmConsensusReport,
)
async def handle_dual_vlm_consensus(
    org_slug: str,
    payload: DualVlmIngestRequest,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> DualVlmConsensusReport:
    _ = org_slug
    return reconcile_dual_vlm_extractions(payload.vlm_a, payload.vlm_b)


@router.post(
    "/api/v1/orgs/{org_slug}/ai/datasheets/revision-diff",
    response_model=DatasheetRevisionDiff,
)
async def handle_revision_diff(
    org_slug: str,
    payload: RevisionDiffApiRequest,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> DatasheetRevisionDiff:
    _ = org_slug
    return diff_datasheet_revisions(payload.old_record, payload.new_record)


@router.post(
    "/api/v1/orgs/{org_slug}/ai/datasheets/testbench",
    response_model=DatasheetTestbenchPlan,
)
async def handle_build_testbench(
    org_slug: str,
    payload: DatasheetRecordAst,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> DatasheetTestbenchPlan:
    _ = org_slug
    return reconstruct_typical_application_testbench(payload)


@router.post(
    "/api/v1/orgs/{org_slug}/ai/datasheets/translate-glossary",
    response_model=GlossaryTranslationResult,
)
async def handle_translate_glossary(
    org_slug: str,
    payload: GlossaryTranslateApiRequest,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> GlossaryTranslationResult:
    _ = org_slug
    return translate_quarantined_technical_text(payload.source_text, payload.target_locale)


def create_app() -> FastAPI:
    app = FastAPI(title="Signal Quarantined Datasheet Ingest")
    app.include_router(router)
    assert_all_routes_guarded(app)
    return app
