# Module: ai/llm-gateway/src/llm_gateway/router.py — Fail-Closed FastAPI Router for Model-Agnostic LLM Gateway
# Invariants: Guarded by Depends(require_org_member); maps FinOps circuit-breaker trips to HTTP 429.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md
from __future__ import annotations

from fastapi import APIRouter, Depends, HTTPException

from llm_gateway.finops import FinOpsQuotaExceededError
from llm_gateway.gateway import ModelAgnosticLlmGateway
from llm_gateway.schemas import GatewayCompletionRequest, GatewayCompletionResponse
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_org_member

router = APIRouter()
_DEFAULT_GATEWAY = ModelAgnosticLlmGateway()


@router.post("/api/v1/orgs/{org_slug}/ai/gateway/complete", response_model=GatewayCompletionResponse)
async def handle_gateway_complete(
    org_slug: str,
    payload: GatewayCompletionRequest,
    principal: PrincipalContext = Depends(require_org_member),
    _idem: str = Depends(require_idempotency_key),
) -> GatewayCompletionResponse:
    _ = org_slug
    scoped = payload.model_copy(
        update={"org_id": principal.org_id, "actor_hmac": principal.actor_hmac}
    )
    try:
        return _DEFAULT_GATEWAY.complete_structured(scoped)
    except FinOpsQuotaExceededError as exc:
        raise HTTPException(status_code=429, detail=str(exc)) from exc
