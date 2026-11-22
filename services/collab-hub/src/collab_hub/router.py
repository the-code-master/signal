# Module: services/collab-hub/src/collab_hub/router.py — Fail-Closed URL-Scoped Collab, Tower & Review Router
# Invariants: 100% Depends(require_*) guarded + uniform 404 anti-enumeration + zero /export paths (F-084).
# Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
from __future__ import annotations

from fastapi import APIRouter, Depends

from collab_hub.schemas import (
    ActionItemCreateRequest,
    ActionItemDto,
    AutomatedReviewSummary,
    ChannelMessageView,
    CollabChannelView,
    DecisionCreateRequest,
    DecisionRecordDto,
    DependencyCreateRequest,
    DependencyHandshakeRequest,
    EntityAnchorRefDto,
    GenerateReviewPayload,
    HuddleTicketView,
    IssueHuddleTicketPayload,
    PeriodicReviewPacketView,
    PostChannelMessageRequest,
    ProjectStatusTowerView,
    RagOverrideRequest,
    StageGateSignOffPayload,
    StageGateSignOffReceipt,
)
from collab_hub.service import (
    apply_rag_override,
    build_status_tower,
    compile_periodic_review,
    create_action_item,
    create_decision_record,
    create_dependency,
    execute_collab_hub,
    issue_huddle_ticket,
    list_authorized_channels,
    list_channel_messages,
    post_channel_message,
    resolve_entity_anchor,
    sign_off_stage_gate_review,
    update_dependency_handshake,
)
from services.common.idempotency import require_idempotency_key
from services.common.schemas import PrincipalContext
from services.common.security import require_repo_access, require_step_up_2fa

router = APIRouter()
_META_REPO_GUARD = require_repo_access("local_gh_repo_1001")
_OPS_REPO_GUARD = require_repo_access("local_gh_repo_1005")


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/reviews",
    response_model=AutomatedReviewSummary,
)
async def handle_collab_hub(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(_OPS_REPO_GUARD),
) -> AutomatedReviewSummary:
    _ = (org_slug, proj_slug)
    return await execute_collab_hub(principal.org_slug)


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/channels",
    response_model=list[CollabChannelView],
)
async def handle_list_channels(
    org_slug: str,
    proj_slug: str,
    principal: PrincipalContext = Depends(_META_REPO_GUARD),
) -> list[CollabChannelView]:
    _ = org_slug
    return await list_authorized_channels(principal, proj_slug)


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/channels/{channel_id}/messages",
    response_model=list[ChannelMessageView],
)
async def handle_list_channel_messages(
    org_slug: str,
    proj_slug: str,
    channel_id: str,
    principal: PrincipalContext = Depends(_META_REPO_GUARD),
) -> list[ChannelMessageView]:
    _ = org_slug
    return await list_channel_messages(principal, proj_slug, channel_id)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/channels/{channel_id}/messages",
    response_model=ChannelMessageView,
)
async def handle_post_channel_message(
    org_slug: str,
    proj_slug: str,
    channel_id: str,
    payload: PostChannelMessageRequest,
    principal: PrincipalContext = Depends(_META_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> ChannelMessageView:
    _ = org_slug
    return await post_channel_message(principal, proj_slug, channel_id, payload)


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/anchors/{anchor_id}",
    response_model=EntityAnchorRefDto,
)
async def handle_resolve_anchor(
    org_slug: str,
    proj_slug: str,
    anchor_id: str,
    principal: PrincipalContext = Depends(_META_REPO_GUARD),
) -> EntityAnchorRefDto:
    _ = org_slug
    return await resolve_entity_anchor(principal, proj_slug, anchor_id)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/channels/{channel_id}/huddle-tickets",
    response_model=HuddleTicketView,
)
async def handle_issue_huddle_ticket(
    org_slug: str,
    proj_slug: str,
    channel_id: str,
    payload: IssueHuddleTicketPayload,
    principal: PrincipalContext = Depends(_META_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> HuddleTicketView:
    _ = org_slug
    return await issue_huddle_ticket(principal, proj_slug, channel_id, payload)


@router.get(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/status-tower",
    response_model=ProjectStatusTowerView,
)
async def handle_get_status_tower(
    org_slug: str,
    proj_slug: str,
    now_utc_ms: int = 1_795_324_800_000,
    principal: PrincipalContext = Depends(_META_REPO_GUARD),
) -> ProjectStatusTowerView:
    _ = org_slug
    return build_status_tower(principal.org_slug, proj_slug, now_utc_ms)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/dependencies",
    response_model=ProjectStatusTowerView,
)
async def handle_create_dependency(
    org_slug: str,
    proj_slug: str,
    payload: DependencyCreateRequest,
    now_utc_ms: int = 1_795_324_800_000,
    principal: PrincipalContext = Depends(_OPS_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> ProjectStatusTowerView:
    _ = org_slug
    return await create_dependency(principal.org_slug, proj_slug, payload, now_utc_ms)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/dependencies/{dependency_id}/handshake",
    response_model=ProjectStatusTowerView,
)
async def handle_update_dependency_handshake(
    org_slug: str,
    proj_slug: str,
    dependency_id: str,
    payload: DependencyHandshakeRequest,
    now_utc_ms: int = 1_795_324_800_000,
    principal: PrincipalContext = Depends(_OPS_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> ProjectStatusTowerView:
    _ = org_slug
    return await update_dependency_handshake(principal.org_slug, proj_slug, dependency_id, payload, now_utc_ms)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/status-tower/rag-overrides",
    response_model=ProjectStatusTowerView,
)
async def handle_rag_override(
    org_slug: str,
    proj_slug: str,
    payload: RagOverrideRequest,
    now_utc_ms: int = 1_795_324_800_000,
    principal: PrincipalContext = Depends(_OPS_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> ProjectStatusTowerView:
    _ = org_slug
    return await apply_rag_override(principal.org_slug, proj_slug, payload, now_utc_ms)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/actions",
    response_model=ActionItemDto,
)
async def handle_create_action_item(
    org_slug: str,
    proj_slug: str,
    payload: ActionItemCreateRequest,
    principal: PrincipalContext = Depends(_OPS_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> ActionItemDto:
    _ = org_slug
    return await create_action_item(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/decisions",
    response_model=DecisionRecordDto,
)
async def handle_create_decision_record(
    org_slug: str,
    proj_slug: str,
    payload: DecisionCreateRequest,
    principal: PrincipalContext = Depends(_OPS_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> DecisionRecordDto:
    _ = org_slug
    return await create_decision_record(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/reviews/generate",
    response_model=PeriodicReviewPacketView,
)
async def handle_generate_periodic_review(
    org_slug: str,
    proj_slug: str,
    payload: GenerateReviewPayload,
    principal: PrincipalContext = Depends(_OPS_REPO_GUARD),
    _idem: str = Depends(require_idempotency_key),
) -> PeriodicReviewPacketView:
    _ = org_slug
    return await compile_periodic_review(principal.org_slug, proj_slug, payload)


@router.post(
    "/api/v1/orgs/{org_slug}/projects/{proj_slug}/collab/reviews/{review_id}/sign-off",
    response_model=StageGateSignOffReceipt,
)
async def handle_sign_off_stage_gate_review(
    org_slug: str,
    proj_slug: str,
    review_id: str,
    payload: StageGateSignOffPayload,
    principal: PrincipalContext = Depends(_OPS_REPO_GUARD),
    _stepup: PrincipalContext = Depends(require_step_up_2fa),
    _idem: str = Depends(require_idempotency_key),
) -> StageGateSignOffReceipt:
    _ = org_slug
    return await sign_off_stage_gate_review(principal, proj_slug, review_id, payload)
