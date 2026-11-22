# Module: ai/orchestrator/src/orchestrator/router.py — Fail-Closed FastAPI Router for CoW Orchestrator & Proof Gate
# Invariants: Guarded by Depends(require_org_member) + Idempotency-Key; blocks unverified CoW merges (F-133).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md
from __future__ import annotations

from fastapi import APIRouter, Depends, HTTPException

from orchestrator.guardian import QuiescentDesignGuardian
from orchestrator.merge_verifier import verify_proof_bundle_at_merge
from orchestrator.schemas import (
    GuardianCheckRequest,
    GuardianCheckResponse,
    MergeProofCheckRequest,
    MergeProofCheckResponse,
    SynthesisRunResponse,
    SynthesisSpecRequest,
)
from orchestrator.synthesis import ConeBudgetExceededError, NeuroSymbolicOrchestrator
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_member

router = APIRouter()
_ORCHESTRATOR = NeuroSymbolicOrchestrator()
_GUARDIAN = QuiescentDesignGuardian()


@router.post("/api/v1/orgs/{org_slug}/ai/synthesize", response_model=SynthesisRunResponse)
async def handle_synthesize(
    org_slug: str,
    payload: SynthesisSpecRequest,
    principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> SynthesisRunResponse:
    _ = org_slug
    scoped = payload.model_copy(
        update={"org_id": principal.org_id, "actor_hmac": principal.actor_hmac}
    )
    try:
        return _ORCHESTRATOR.run_speculative_synthesis(scoped)
    except ConeBudgetExceededError as exc:
        raise HTTPException(status_code=400, detail=str(exc)) from exc


@router.post("/api/v1/orgs/{org_slug}/ai/guardian/evaluate", response_model=GuardianCheckResponse)
async def handle_guardian_evaluate(
    org_slug: str,
    payload: GuardianCheckRequest,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> GuardianCheckResponse:
    _ = org_slug
    return _GUARDIAN.evaluate(payload)


@router.post("/api/v1/orgs/{org_slug}/ai/merge/verify-proof", response_model=MergeProofCheckResponse)
async def handle_verify_merge_proof(
    org_slug: str,
    payload: MergeProofCheckRequest,
    _principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> MergeProofCheckResponse:
    _ = org_slug
    return verify_proof_bundle_at_merge(payload)
