# Module: services/collab-hub/src/collab_hub/schemas.py — Collab Space, Status Tower & Auto-Review DTOs
# Invariants: StrictSchema (extra="forbid", frozen=True); Int64Str 64-bit parity; SecretStr step-up proofs (F-144).
# Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
from __future__ import annotations

from typing import Literal

from pydantic import Field, SecretStr

from services.common.schemas import Int64Str, StrictSchema

TeamWorkspaceLiteral = Literal[
    "DESIGN",
    "DEVELOPMENT_BRINGUP",
    "SOFTWARE_FIRMWARE",
    "MANUFACTURING",
    "OPERATIONS_SUPPLY_CHAIN",
    "CROSS_TEAM_PROGRAM",
]
RagHealthLiteral = Literal["GREEN", "AMBER", "RED"]
ReviewCadenceLiteral = Literal["WEEKLY_ENGINEERING", "MONTHLY_EXECUTIVE", "STAGE_GATE_STATUS"]
StageGateNameLiteral = Literal["CONCEPT", "PRD_FREEZE", "EVT", "DVT", "PVT", "TAPEOUT", "MP"]
HandshakeStatusLiteral = Literal["PROPOSED", "ACCEPTED", "AT_RISK", "BLOCKED"]


class AutomatedReviewSummary(StrictSchema):
    review_id: str
    cadence: str
    overall_rag: str
    stage_gate: str
    generated_at_ms: Int64Str


class EntityAnchorRefDto(StrictSchema):
    anchor_id: str
    domain: str
    entity_id: str
    commit_sha: str
    x_nm: Int64Str
    y_nm: Int64Str
    z_nm: Int64Str = 0
    required_provider_repo_id: str
    redacted: bool = False


class CollabChannelView(StrictSchema):
    channel_id: str
    slug: str
    title: str
    team: TeamWorkspaceLiteral
    required_repo_scopes: tuple[str, ...]
    encrypted_dek_key_id: str


class PostChannelMessageRequest(StrictSchema):
    plaintext_body: str = Field(min_length=1, max_length=4096)
    thread_parent_id: str | None = None
    entity_anchors: tuple[EntityAnchorRefDto, ...] = ()


class ChannelMessageView(StrictSchema):
    message_id: str
    channel_id: str
    team: TeamWorkspaceLiteral
    author_actor_hmac: str
    decrypted_body: str
    stored_dek_envelope: str
    thread_parent_id: str | None = None
    entity_anchors: tuple[EntityAnchorRefDto, ...]
    redacted_anchor_count: int
    created_at_utc_ms: Int64Str


class IssueHuddleTicketPayload(StrictSchema):
    requested_ttl_ms: int = Field(default=900_000, ge=60_000, le=900_000)
    follow_presenter_viewport: bool = True


class HuddleTicketView(StrictSchema):
    huddle_room_id: str
    channel_id: str
    sfu_turns_endpoint: str
    ed25519_huddle_ticket: str
    expires_at_utc_ms: Int64Str
    follow_presenter_viewport: bool
    geometry_recording_disabled: bool = True


class TeamRagCardDto(StrictSchema):
    team: TeamWorkspaceLiteral
    auto_health: RagHealthLiteral
    effective_health: RagHealthLiteral
    open_critical_blockers: int
    open_warnings: int
    override_rationale: str | None = None


class RagOverrideRequest(StrictSchema):
    team: TeamWorkspaceLiteral
    override_health: RagHealthLiteral
    override_rationale: str = Field(min_length=8, max_length=512)


class StageGateMilestoneDto(StrictSchema):
    gate_id: str
    gate_name: StageGateNameLiteral
    target_date_utc_ms: Int64Str
    health: RagHealthLiteral
    open_blocker_count: int


class DependencyCreateRequest(StrictSchema):
    producer_team: TeamWorkspaceLiteral
    consumer_team: TeamWorkspaceLiteral
    deliverable_title: str = Field(min_length=3, max_length=240)
    due_at_utc_ms: Int64Str
    linked_anchor_id: str | None = None


class DependencyHandshakeRequest(StrictSchema):
    handshake_status: HandshakeStatusLiteral


class CrossTeamDependencyDto(StrictSchema):
    dependency_id: str
    producer_team: TeamWorkspaceLiteral
    consumer_team: TeamWorkspaceLiteral
    deliverable_title: str
    due_at_utc_ms: Int64Str
    handshake_status: HandshakeStatusLiteral
    linked_anchor_id: str | None = None


class ActionItemCreateRequest(StrictSchema):
    owner_pseudo_id: str = Field(min_length=3, max_length=120)
    team: TeamWorkspaceLiteral
    summary: str = Field(min_length=3, max_length=300)
    due_at_utc_ms: Int64Str


class ActionItemDto(StrictSchema):
    action_id: str
    owner_pseudo_id: str
    team: TeamWorkspaceLiteral
    summary: str
    due_at_utc_ms: Int64Str
    completed: bool = False


class DecisionCreateRequest(StrictSchema):
    record_kind: Literal["ADR", "EDR"]
    title: str = Field(min_length=4, max_length=200)
    rationale: str = Field(min_length=8, max_length=1000)


class DecisionRecordDto(StrictSchema):
    decision_id: str
    record_kind: Literal["ADR", "EDR"]
    title: str
    rationale: str
    decided_at_utc_ms: Int64Str
    ml_dsa_65_signature: str


class ProjectStatusTowerView(StrictSchema):
    overall_health: RagHealthLiteral
    team_rag_cards: tuple[TeamRagCardDto, ...]
    milestones: tuple[StageGateMilestoneDto, ...]
    dependencies: tuple[CrossTeamDependencyDto, ...]
    action_items: tuple[ActionItemDto, ...]
    decisions: tuple[DecisionRecordDto, ...]


class GenerateReviewPayload(StrictSchema):
    cadence: ReviewCadenceLiteral
    window_start_utc_ms: Int64Str = 1_794_720_000_000
    window_end_utc_ms: Int64Str = 1_795_324_800_000
    stage_gate_name: StageGateNameLiteral = "DVT"
    now_utc_ms: Int64Str = 1_795_324_800_000


class PeriodicReviewPacketView(StrictSchema):
    review_id: str
    cadence: ReviewCadenceLiteral
    stage_gate_name: StageGateNameLiteral
    overall_health: RagHealthLiteral
    open_drc_erc_violations: int
    worst_spice_soa_margin_pct: float
    worst_sta_slack_fs: Int64Str
    clear_to_build_units: int
    open_deviations: int
    security_posture_score: float
    archived_ops_repo_path: str
    ml_dsa_65_review_signature: str
    gbnf_verified_narrative_md: str
    generated_at_utc_ms: Int64Str
    gate_approved: bool = False
    quorum_audit_outbox_seq: Int64Str | None = None
    ops_repo_commit_sha: str | None = None


class StageGateSignOffPayload(StrictSchema):
    stepup_2fa_nonce: SecretStr
    ml_dsa_65_approver_signature: str
    simulated_healthy_audit_replicas: int = Field(default=3, ge=0, le=3)
    now_utc_ms: Int64Str = 1_795_324_800_000


class StageGateSignOffReceipt(StrictSchema):
    review_id: str
    gate_approved: bool
    quorum_audit_outbox_seq: Int64Str
    audit_merkle_hash: str
    ops_repo_commit_sha: str
    signed_at_utc_ms: Int64Str
