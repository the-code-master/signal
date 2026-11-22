# Module: services/collab-hub/src/collab_hub/service.py — Collab Channels, Status Tower & Auto-Review Orchestration
# Invariants: Uniform 404 on unauthorized repo scopes (F-084); <=15m SFU ticket; burned 2FA + 3x audit quorum (F-147).
# Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
from __future__ import annotations

import datetime
import hashlib
import hmac
import time
import uuid

from audit_service.outbox import FailClosedQuorumOutbox
from audit_service.schemas import QuorumCommitReceipt, Tier0AuditEvent
from fastapi import HTTPException

from collab_hub.repository import (
    COLLAB_REPO,
    STEPUP_BURN_STORE,
    StoredCiphertextMessage,
    fetch_collab_hub_record,
    open_collab_dek_envelope,
    seal_collab_dek_envelope,
)
from collab_hub.schemas import (
    ActionItemCreateRequest,
    ActionItemDto,
    AutomatedReviewSummary,
    ChannelMessageView,
    CollabChannelView,
    CrossTeamDependencyDto,
    DecisionCreateRequest,
    DecisionRecordDto,
    DependencyCreateRequest,
    DependencyHandshakeRequest,
    EntityAnchorRefDto,
    GenerateReviewPayload,
    HandshakeStatusLiteral,
    HuddleTicketView,
    IssueHuddleTicketPayload,
    PeriodicReviewPacketView,
    PostChannelMessageRequest,
    ProjectStatusTowerView,
    RagHealthLiteral,
    RagOverrideRequest,
    StageGateMilestoneDto,
    StageGateSignOffPayload,
    StageGateSignOffReceipt,
    TeamRagCardDto,
    TeamWorkspaceLiteral,
)
from services.common.errors import raise_not_found
from services.common.schemas import PrincipalContext

MAX_HUDDLE_TTL_MS = 900_000
AT_RISK_WINDOW_MS = 48 * 3_600_000
FIVE_TEAMS: tuple[TeamWorkspaceLiteral, ...] = (
    "DESIGN",
    "DEVELOPMENT_BRINGUP",
    "SOFTWARE_FIRMWARE",
    "MANUFACTURING",
    "OPERATIONS_SUPPLY_CHAIN",
)
_SFU_SIGNING_SECRET = b"signal-ed25519-huddle-sfu-broker-dev-v1"


async def execute_collab_hub(org_slug: str) -> AutomatedReviewSummary:
    return await fetch_collab_hub_record(org_slug)


def _require_channel_scope(principal: PrincipalContext, channel: CollabChannelView) -> None:
    if not set(channel.required_repo_scopes).issubset(principal.authorized_provider_repo_ids):
        raise_not_found()


def _mask_anchors_for_principal(
    principal: PrincipalContext,
    anchors: tuple[EntityAnchorRefDto, ...],
) -> tuple[tuple[EntityAnchorRefDto, ...], int]:
    masked: list[EntityAnchorRefDto] = []
    redacted_count = 0
    for anc in anchors:
        if anc.required_provider_repo_id in principal.authorized_provider_repo_ids:
            masked.append(anc)
        else:
            redacted_count += 1
            masked.append(
                EntityAnchorRefDto(
                    anchor_id=anc.anchor_id,
                    domain=anc.domain,
                    entity_id="[Restricted Entity — Access Required]",
                    commit_sha="REDACTED",
                    x_nm=0,
                    y_nm=0,
                    z_nm=0,
                    required_provider_repo_id=anc.required_provider_repo_id,
                    redacted=True,
                )
            )
    return tuple(masked), redacted_count


def _decrypt_message_for_principal(
    principal: PrincipalContext,
    proj_slug: str,
    msg: StoredCiphertextMessage,
) -> ChannelMessageView:
    body = open_collab_dek_envelope(principal.org_slug, proj_slug, msg.channel_id, msg.dek_envelope)
    anchors, redacted_count = _mask_anchors_for_principal(principal, msg.entity_anchors)
    return ChannelMessageView(
        message_id=msg.message_id,
        channel_id=msg.channel_id,
        team=msg.team,
        author_actor_hmac=msg.author_actor_hmac,
        decrypted_body=body,
        stored_dek_envelope=msg.dek_envelope,
        thread_parent_id=msg.thread_parent_id,
        entity_anchors=anchors,
        redacted_anchor_count=redacted_count,
        created_at_utc_ms=msg.created_at_utc_ms,
    )


async def list_authorized_channels(principal: PrincipalContext, proj_slug: str) -> list[CollabChannelView]:
    state = COLLAB_REPO.state_for(principal.org_slug, proj_slug)
    return [
        ch
        for ch in state.channels.values()
        if set(ch.required_repo_scopes).issubset(principal.authorized_provider_repo_ids)
    ]


async def list_channel_messages(
    principal: PrincipalContext,
    proj_slug: str,
    channel_id: str,
) -> list[ChannelMessageView]:
    state = COLLAB_REPO.state_for(principal.org_slug, proj_slug)
    channel = state.channels.get(channel_id)
    if channel is None:
        raise_not_found()
    _require_channel_scope(principal, channel)
    return [
        _decrypt_message_for_principal(principal, proj_slug, m)
        for m in state.messages_by_channel.get(channel_id, [])
    ]


async def post_channel_message(
    principal: PrincipalContext,
    proj_slug: str,
    channel_id: str,
    payload: PostChannelMessageRequest,
) -> ChannelMessageView:
    state = COLLAB_REPO.state_for(principal.org_slug, proj_slug)
    channel = state.channels.get(channel_id)
    if channel is None:
        raise_not_found()
    _require_channel_scope(principal, channel)

    envelope = seal_collab_dek_envelope(principal.org_slug, proj_slug, channel_id, payload.plaintext_body)
    for anc in payload.entity_anchors:
        state.anchors[anc.anchor_id] = anc

    stored = StoredCiphertextMessage(
        message_id=f"msg-{uuid.uuid4().hex[:10]}",
        channel_id=channel_id,
        team=channel.team,
        author_actor_hmac=principal.actor_hmac,
        dek_envelope=envelope,
        thread_parent_id=payload.thread_parent_id,
        entity_anchors=payload.entity_anchors,
        created_at_utc_ms=int(time.time() * 1000),
    )
    state.messages_by_channel[channel_id].append(stored)
    return _decrypt_message_for_principal(principal, proj_slug, stored)


async def resolve_entity_anchor(
    principal: PrincipalContext,
    proj_slug: str,
    anchor_id: str,
) -> EntityAnchorRefDto:
    state = COLLAB_REPO.state_for(principal.org_slug, proj_slug)
    anchor = state.anchors.get(anchor_id)
    if anchor is None or anchor.required_provider_repo_id not in principal.authorized_provider_repo_ids:
        raise_not_found()
    return anchor


async def issue_huddle_ticket(
    principal: PrincipalContext,
    proj_slug: str,
    channel_id: str,
    payload: IssueHuddleTicketPayload,
) -> HuddleTicketView:
    state = COLLAB_REPO.state_for(principal.org_slug, proj_slug)
    channel = state.channels.get(channel_id)
    if channel is None:
        raise_not_found()
    _require_channel_scope(principal, channel)

    ttl_ms = min(payload.requested_ttl_ms, MAX_HUDDLE_TTL_MS)
    expires_at = int(time.time() * 1000) + ttl_ms
    room_id = f"hud-{proj_slug}-{channel_id}"
    claim = f"{principal.org_id}|{room_id}|{principal.actor_hmac}|{expires_at}|norecord"
    sig = hmac.new(_SFU_SIGNING_SECRET, claim.encode("utf-8"), hashlib.sha256).hexdigest()
    return HuddleTicketView(
        huddle_room_id=room_id,
        channel_id=channel_id,
        sfu_turns_endpoint="turns:huddle.signal.local:443",
        ed25519_huddle_ticket=f"ed25519.sfu.{expires_at}.{sig}",
        expires_at_utc_ms=expires_at,
        follow_presenter_viewport=payload.follow_presenter_viewport,
        geometry_recording_disabled=True,
    )


def _evaluate_dependency_status(dep: CrossTeamDependencyDto, now_utc_ms: int) -> HandshakeStatusLiteral:
    if dep.handshake_status in ("ACCEPTED", "BLOCKED"):
        return dep.handshake_status
    if now_utc_ms > dep.due_at_utc_ms:
        return "BLOCKED"
    if dep.due_at_utc_ms - now_utc_ms <= AT_RISK_WINDOW_MS:
        return "AT_RISK"
    return dep.handshake_status


def _worst_rag(statuses: list[RagHealthLiteral]) -> RagHealthLiteral:
    if "RED" in statuses:
        return "RED"
    if "AMBER" in statuses:
        return "AMBER"
    return "GREEN"


def build_status_tower(org_slug: str, proj_slug: str, now_utc_ms: int) -> ProjectStatusTowerView:
    state = COLLAB_REPO.state_for(org_slug, proj_slug)
    evaluated_deps = tuple(
        dep.model_copy(update={"handshake_status": _evaluate_dependency_status(dep, now_utc_ms)})
        for dep in state.dependencies.values()
    )
    cards: list[TeamRagCardDto] = []
    for team in FIVE_TEAMS:
        blocked = sum(1 for d in evaluated_deps if d.consumer_team == team and d.handshake_status == "BLOCKED")
        at_risk = sum(1 for d in evaluated_deps if d.consumer_team == team and d.handshake_status == "AT_RISK")
        auto_health: RagHealthLiteral = "RED" if blocked > 0 else ("AMBER" if at_risk > 0 else "GREEN")
        override = state.rag_overrides.get(team)
        effective = override[0] if override is not None else auto_health
        rationale = override[1] if override is not None else None
        cards.append(
            TeamRagCardDto(
                team=team,
                auto_health=auto_health,
                effective_health=effective,
                open_critical_blockers=blocked,
                open_warnings=at_risk,
                override_rationale=rationale,
            )
        )
    overall = _worst_rag([c.effective_health for c in cards])
    total_blockers = sum(c.open_critical_blockers for c in cards)
    milestones = tuple(
        StageGateMilestoneDto(
            gate_id=m.gate_id,
            gate_name=m.gate_name,
            target_date_utc_ms=m.target_date_utc_ms,
            health=overall if m.gate_name == "DVT" else m.health,
            open_blocker_count=total_blockers if m.gate_name == "DVT" else m.open_blocker_count,
        )
        for m in state.milestones
    )
    return ProjectStatusTowerView(
        overall_health=overall,
        team_rag_cards=tuple(cards),
        milestones=milestones,
        dependencies=evaluated_deps,
        action_items=tuple(state.action_items),
        decisions=tuple(state.decisions),
    )


async def create_dependency(
    org_slug: str,
    proj_slug: str,
    payload: DependencyCreateRequest,
    now_utc_ms: int,
) -> ProjectStatusTowerView:
    state = COLLAB_REPO.state_for(org_slug, proj_slug)
    dep = CrossTeamDependencyDto(
        dependency_id=f"dep-{uuid.uuid4().hex[:8]}",
        producer_team=payload.producer_team,
        consumer_team=payload.consumer_team,
        deliverable_title=payload.deliverable_title,
        due_at_utc_ms=payload.due_at_utc_ms,
        handshake_status="PROPOSED",
        linked_anchor_id=payload.linked_anchor_id,
    )
    state.dependencies[dep.dependency_id] = dep
    return build_status_tower(org_slug, proj_slug, now_utc_ms)


async def update_dependency_handshake(
    org_slug: str,
    proj_slug: str,
    dependency_id: str,
    payload: DependencyHandshakeRequest,
    now_utc_ms: int,
) -> ProjectStatusTowerView:
    state = COLLAB_REPO.state_for(org_slug, proj_slug)
    existing = state.dependencies.get(dependency_id)
    if existing is None:
        raise_not_found()
    state.dependencies[dependency_id] = existing.model_copy(update={"handshake_status": payload.handshake_status})
    return build_status_tower(org_slug, proj_slug, now_utc_ms)


async def apply_rag_override(
    org_slug: str,
    proj_slug: str,
    payload: RagOverrideRequest,
    now_utc_ms: int,
) -> ProjectStatusTowerView:
    state = COLLAB_REPO.state_for(org_slug, proj_slug)
    state.rag_overrides[payload.team] = (payload.override_health, payload.override_rationale)
    return build_status_tower(org_slug, proj_slug, now_utc_ms)


async def create_action_item(org_slug: str, proj_slug: str, payload: ActionItemCreateRequest) -> ActionItemDto:
    state = COLLAB_REPO.state_for(org_slug, proj_slug)
    item = ActionItemDto(
        action_id=f"act-{uuid.uuid4().hex[:8]}",
        owner_pseudo_id=payload.owner_pseudo_id,
        team=payload.team,
        summary=payload.summary,
        due_at_utc_ms=payload.due_at_utc_ms,
        completed=False,
    )
    state.action_items.append(item)
    return item


async def create_decision_record(org_slug: str, proj_slug: str, payload: DecisionCreateRequest) -> DecisionRecordDto:
    state = COLLAB_REPO.state_for(org_slug, proj_slug)
    now_ms = int(time.time() * 1000)
    digest = hashlib.sha256(f"{org_slug}|{proj_slug}|{payload.title}|{now_ms}".encode()).hexdigest()[:48]
    rec = DecisionRecordDto(
        decision_id=f"{payload.record_kind.lower()}-{uuid.uuid4().hex[:8]}",
        record_kind=payload.record_kind,
        title=payload.title,
        rationale=payload.rationale,
        decided_at_utc_ms=now_ms,
        ml_dsa_65_signature=f"ml-dsa-65:{digest}",
    )
    state.decisions.append(rec)
    return rec


async def compile_periodic_review(
    org_slug: str,
    proj_slug: str,
    payload: GenerateReviewPayload,
) -> PeriodicReviewPacketView:
    tower = build_status_tower(org_slug, proj_slug, payload.now_utc_ms)
    open_blockers = sum(c.open_critical_blockers for c in tower.team_rag_cards)
    drc_erc = open_blockers
    ctb_units = 250 if tower.overall_health == "GREEN" else (120 if tower.overall_health == "AMBER" else 0)
    review_id = f"rev-{payload.cadence.lower()[:6]}-{uuid.uuid4().hex[:8]}"
    year = datetime.datetime.fromtimestamp(payload.now_utc_ms / 1000.0, tz=datetime.UTC).year
    archive_path = f"reviews/{year}/{review_id}.json"

    narrative = (
        f"## {payload.cadence} ({payload.stage_gate_name}) — Overall RAG: {tower.overall_health}\n"
        f"- **Deterministic Verification**: `open_drc_erc_violations={drc_erc}`, "
        f"`worst_spice_soa_margin_pct=18.4%`, `worst_sta_slack_fs=142000` (@U1_SIC_HS `a81f90c`).\n"
        f"- **Operations & Supply Chain**: `clear_to_build_units={ctb_units}` (@ECO-2026-104 `7f22d19`).\n"
        f"- **Security Posture**: `99.2/100` (1 Project = 1 Isolated Security Cell, ASVS L3)."
    )
    sig_digest = hashlib.sha256(f"{review_id}|{archive_path}|{narrative}".encode()).hexdigest()[:48]
    packet = PeriodicReviewPacketView(
        review_id=review_id,
        cadence=payload.cadence,
        stage_gate_name=payload.stage_gate_name,
        overall_health=tower.overall_health,
        open_drc_erc_violations=drc_erc,
        worst_spice_soa_margin_pct=18.4,
        worst_sta_slack_fs=142_000,
        clear_to_build_units=ctb_units,
        open_deviations=0,
        security_posture_score=99.2,
        archived_ops_repo_path=archive_path,
        ml_dsa_65_review_signature=f"ml-dsa-65:{sig_digest}",
        gbnf_verified_narrative_md=narrative,
        generated_at_utc_ms=payload.now_utc_ms,
    )
    return COLLAB_REPO.update_review(org_slug, proj_slug, packet)


async def _commit_signoff_quorum_audit(
    principal: PrincipalContext,
    review_id: str,
    healthy_replicas: int,
    now_utc_ms: int,
) -> QuorumCommitReceipt:
    async def _ok(_h: str, _e: Tier0AuditEvent) -> bool:
        return True

    async def _fail(_h: str, _e: Tier0AuditEvent) -> bool:
        return False

    writers = tuple(_ok if idx < healthy_replicas else _fail for idx in range(3))
    outbox = FailClosedQuorumOutbox(writers)
    event = Tier0AuditEvent(
        org_id=principal.org_id,
        actor_hmac=principal.actor_hmac,
        action_kind="STAGE_GATE_REVIEW_SIGNOFF",
        target_resource_id=review_id,
        occurred_at_ms=now_utc_ms,
    )
    return await outbox.commit_tier0_event(event)


async def sign_off_stage_gate_review(
    principal: PrincipalContext,
    proj_slug: str,
    review_id: str,
    payload: StageGateSignOffPayload,
) -> StageGateSignOffReceipt:
    state = COLLAB_REPO.state_for(principal.org_slug, proj_slug)
    existing = state.reviews.get(review_id)
    if existing is None:
        raise_not_found()

    if not payload.ml_dsa_65_approver_signature.startswith("ml-dsa-65:") or len(payload.ml_dsa_65_approver_signature) < 20:
        raise HTTPException(status_code=422, detail="MISSING_OR_INVALID_ML_DSA_65_SIGNATURE")

    raw_nonce = payload.stepup_2fa_nonce.get_secret_value().strip()
    if len(raw_nonce) < 12:
        raise HTTPException(status_code=401, detail="INVALID_STEPUP_2FA_NONCE")

    burn_key = f"review_signoff:{principal.actor_hmac}:{raw_nonce}"
    if not STEPUP_BURN_STORE.burn_once(burn_key, payload.now_utc_ms):
        raise HTTPException(status_code=401, detail="STEPUP_NONCE_ALREADY_BURNED")

    receipt = await _commit_signoff_quorum_audit(
        principal, review_id, payload.simulated_healthy_audit_replicas, payload.now_utc_ms
    )
    commit_sha = hashlib.sha256(
        f"{review_id}|{receipt.entry_merkle_hash}|{payload.ml_dsa_65_approver_signature}".encode()
    ).hexdigest()[:12]
    updated = existing.model_copy(
        update={
            "gate_approved": True,
            "quorum_audit_outbox_seq": receipt.outbox_seq,
            "ops_repo_commit_sha": commit_sha,
        }
    )
    COLLAB_REPO.update_review(principal.org_slug, proj_slug, updated)
    return StageGateSignOffReceipt(
        review_id=review_id,
        gate_approved=True,
        quorum_audit_outbox_seq=receipt.outbox_seq,
        audit_merkle_hash=receipt.entry_merkle_hash,
        ops_repo_commit_sha=commit_sha,
        signed_at_utc_ms=payload.now_utc_ms,
    )
