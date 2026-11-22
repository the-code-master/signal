# Module: services/collab-hub/src/collab_hub/repository.py — Tenant-Isolated DEK-Encrypted Collab & Status Store
# Invariants: Never stores raw message plaintext; enforces org_slug+proj_slug isolation & single-use 2FA burn (F-076).
# Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
from __future__ import annotations

import hashlib
import hmac
import os
from dataclasses import dataclass, field

from collab_hub.schemas import (
    ActionItemDto,
    AutomatedReviewSummary,
    CollabChannelView,
    CrossTeamDependencyDto,
    DecisionRecordDto,
    EntityAnchorRefDto,
    PeriodicReviewPacketView,
    RagHealthLiteral,
    StageGateMilestoneDto,
    TeamWorkspaceLiteral,
)

_MASTER_PEPPER = os.environ.get("SIGNAL_COLLAB_DEK_PEPPER", "signal-hybrid-pqc-collab-dek-v1").encode("utf-8")


def _derive_collab_subkey(org_slug: str, proj_slug: str) -> bytes:
    info = f"collab-v1|{org_slug}|{proj_slug}".encode()
    return hmac.new(_MASTER_PEPPER, info, hashlib.sha256).digest()


def _keystream(subkey: bytes, nonce: bytes, length: int) -> bytes:
    out = bytearray()
    counter = 0
    while len(out) < length:
        block = hmac.new(subkey, nonce + counter.to_bytes(4, "big"), hashlib.sha256).digest()
        out.extend(block)
        counter += 1
    return bytes(out[:length])


def seal_collab_dek_envelope(org_slug: str, proj_slug: str, channel_id: str, plaintext: str) -> str:
    subkey = _derive_collab_subkey(org_slug, proj_slug)
    raw = plaintext.encode("utf-8")
    nonce = os.urandom(12)
    stream = _keystream(subkey, nonce, len(raw))
    ct = bytes(a ^ b for a, b in zip(raw, stream, strict=True))
    aad = f"{org_slug}|{proj_slug}|{channel_id}".encode()
    tag = hmac.new(subkey, nonce + aad + ct, hashlib.sha256).hexdigest()[:32]
    return f"sigv1.collab-dek.{nonce.hex()}.{ct.hex()}.{tag}"


def open_collab_dek_envelope(org_slug: str, proj_slug: str, channel_id: str, envelope: str) -> str:
    parts = envelope.split(".")
    if len(parts) != 5 or parts[0] != "sigv1" or parts[1] != "collab-dek":
        raise ValueError("Invalid collab-v1 DEK ciphertext envelope")
    nonce = bytes.fromhex(parts[2])
    ct = bytes.fromhex(parts[3])
    subkey = _derive_collab_subkey(org_slug, proj_slug)
    aad = f"{org_slug}|{proj_slug}|{channel_id}".encode()
    expected_tag = hmac.new(subkey, nonce + aad + ct, hashlib.sha256).hexdigest()[:32]
    if not hmac.compare_digest(parts[4], expected_tag):
        raise ValueError("Collab DEK authentication tag mismatch")
    stream = _keystream(subkey, nonce, len(ct))
    return bytes(a ^ b for a, b in zip(ct, stream, strict=True)).decode("utf-8")


@dataclass(frozen=True)
class StoredCiphertextMessage:
    message_id: str
    channel_id: str
    team: TeamWorkspaceLiteral
    author_actor_hmac: str
    dek_envelope: str
    thread_parent_id: str | None
    entity_anchors: tuple[EntityAnchorRefDto, ...]
    created_at_utc_ms: int


def _empty_overrides() -> dict[TeamWorkspaceLiteral, tuple[RagHealthLiteral, str]]:
    return {}


def _empty_reviews() -> dict[str, PeriodicReviewPacketView]:
    return {}


@dataclass
class TenantProjectCollabState:
    channels: dict[str, CollabChannelView]
    anchors: dict[str, EntityAnchorRefDto]
    messages_by_channel: dict[str, list[StoredCiphertextMessage]]
    milestones: list[StageGateMilestoneDto]
    dependencies: dict[str, CrossTeamDependencyDto]
    action_items: list[ActionItemDto]
    decisions: list[DecisionRecordDto]
    rag_overrides: dict[TeamWorkspaceLiteral, tuple[RagHealthLiteral, str]] = field(default_factory=_empty_overrides)
    reviews: dict[str, PeriodicReviewPacketView] = field(default_factory=_empty_reviews)


class SingleUseStepUpBurnStore:
    def __init__(self) -> None:
        self._burned: dict[str, int] = {}

    def burn_once(self, key: str, now_utc_ms: int) -> bool:
        if key in self._burned:
            return False
        self._burned[key] = now_utc_ms
        return True


def _seed_channels(proj_slug: str) -> dict[str, CollabChannelView]:
    dek_id = f"dek-pqc-{proj_slug}-collab-v1"
    specs: tuple[tuple[str, str, str, TeamWorkspaceLiteral, tuple[str, ...]], ...] = (
        ("ch-design-ee", "#design-ee-layout", "Design — Schematic & PCB", "DESIGN", ("local_gh_repo_1002",)),
        ("ch-design-mcad", "#design-mcad", "Design — MCAD Enclosure", "DESIGN", ("local_gh_repo_1004",)),
        ("ch-dev-hil", "#dev-hil-bringup", "Development — HIL Bring-Up", "DEVELOPMENT_BRINGUP", ("local_gh_repo_1003",)),
        ("ch-sw-fw", "#sw-firmware-hal", "Software — PinMux & SVD Firmware", "SOFTWARE_FIRMWARE", ("local_gh_repo_1002",)),
        ("ch-mfg-cam", "#mfg-cam-fab", "Manufacturing — DFM, CAM & Fab", "MANUFACTURING", ("local_gh_repo_1005",)),
        ("ch-ops-plm", "#ops-plm-supply", "Operations — BOM, CTB & AVL", "OPERATIONS_SUPPLY_CHAIN", ("local_gh_repo_1005",)),
        ("ch-all-hands", "#all-hands-status", "Cross-Team Program Status", "CROSS_TEAM_PROGRAM", ("local_gh_repo_1001",)),
        ("ch-ccb-board", "#ccb-change-board", "Cross-Team CCB Release", "CROSS_TEAM_PROGRAM", ("local_gh_repo_1005",)),
    )
    return {
        cid: CollabChannelView(
            channel_id=cid,
            slug=slug,
            title=title,
            team=team,
            required_repo_scopes=scopes,
            encrypted_dek_key_id=dek_id,
        )
        for cid, slug, title, team, scopes in specs
    }


def _seed_state(org_slug: str, proj_slug: str) -> TenantProjectCollabState:
    channels = _seed_channels(proj_slug)
    pcb_anchor = EntityAnchorRefDto(
        anchor_id="anc-pcb-u1-sic",
        domain="PCB",
        entity_id="@U1_SIC_HS",
        commit_sha="a81f90c",
        x_nm=42_500_000,
        y_nm=18_250_000,
        z_nm=0,
        required_provider_repo_id="local_gh_repo_1003",
    )
    ops_anchor = EntityAnchorRefDto(
        anchor_id="anc-ops-eco-104",
        domain="PLM_OPS",
        entity_id="@ECO-2026-104",
        commit_sha="7f22d19",
        x_nm=0,
        y_nm=0,
        z_nm=0,
        required_provider_repo_id="local_gh_repo_1005",
    )
    anchors = {pcb_anchor.anchor_id: pcb_anchor, ops_anchor.anchor_id: ops_anchor}
    seeded_msg = StoredCiphertextMessage(
        message_id="msg-seed-001",
        channel_id="ch-all-hands",
        team="CROSS_TEAM_PROGRAM",
        author_actor_hmac="actor_hmac_0001seedlead0000",
        dek_envelope=seal_collab_dek_envelope(
            org_slug,
            proj_slug,
            "ch-all-hands",
            "DVT thermal margin check linking @U1_SIC_HS layout to @ECO-2026-104 second-source die.",
        ),
        thread_parent_id=None,
        entity_anchors=(pcb_anchor, ops_anchor),
        created_at_utc_ms=1_795_200_000_000,
    )
    messages: dict[str, list[StoredCiphertextMessage]] = {cid: [] for cid in channels}
    messages["ch-all-hands"].append(seeded_msg)

    milestones = [
        StageGateMilestoneDto(
            gate_id="gate-evt", gate_name="EVT", target_date_utc_ms=1_790_000_000_000, health="GREEN", open_blocker_count=0
        ),
        StageGateMilestoneDto(
            gate_id="gate-dvt", gate_name="DVT", target_date_utc_ms=1_796_000_000_000, health="GREEN", open_blocker_count=0
        ),
        StageGateMilestoneDto(
            gate_id="gate-pvt", gate_name="PVT", target_date_utc_ms=1_801_000_000_000, health="GREEN", open_blocker_count=0
        ),
    ]
    dep = CrossTeamDependencyDto(
        dependency_id="dep-ee-to-fw-01",
        producer_team="DESIGN",
        consumer_team="SOFTWARE_FIRMWARE",
        deliverable_title="Frozen SiC Gate-Driver PinMux & CMSIS-SVD",
        due_at_utc_ms=1_796_500_000_000,
        handshake_status="ACCEPTED",
        linked_anchor_id="anc-pcb-u1-sic",
    )
    action = ActionItemDto(
        action_id="act-001",
        owner_pseudo_id="actor_hmac_0001seedlead0000",
        team="MANUFACTURING",
        summary="Verify press-fit connector Z-axis CNC boss tolerance in STEP rev B",
        due_at_utc_ms=1_795_900_000_000,
        completed=False,
    )
    decision = DecisionRecordDto(
        decision_id="adr-001",
        record_kind="ADR",
        title="Adopt 800V Creepage Slotting Under High-Side Gate Driver",
        rationale="Maintains IEC 60664-1 Pollution Degree 2 margin >= 8.0 mm across PCB rev D.",
        decided_at_utc_ms=1_795_100_000_000,
        ml_dsa_65_signature="ml-dsa-65:9f4a8b12c701e3d4456a7890bcdef1234567890abcdef123",
    )
    return TenantProjectCollabState(
        channels=channels,
        anchors=anchors,
        messages_by_channel=messages,
        milestones=milestones,
        dependencies={dep.dependency_id: dep},
        action_items=[action],
        decisions=[decision],
    )


class CollabRepository:
    def __init__(self) -> None:
        self._states: dict[tuple[str, str], TenantProjectCollabState] = {}

    def state_for(self, org_slug: str, proj_slug: str) -> TenantProjectCollabState:
        key = (org_slug, proj_slug)
        if key not in self._states:
            self._states[key] = _seed_state(org_slug, proj_slug)
        return self._states[key]

    def update_review(self, org_slug: str, proj_slug: str, packet: PeriodicReviewPacketView) -> PeriodicReviewPacketView:
        state = self.state_for(org_slug, proj_slug)
        state.reviews[packet.review_id] = packet
        return packet


COLLAB_REPO = CollabRepository()
STEPUP_BURN_STORE = SingleUseStepUpBurnStore()


async def fetch_collab_hub_record(org_slug: str) -> AutomatedReviewSummary:
    _ = org_slug
    return AutomatedReviewSummary(
        review_id="rev-w47-dvt", cadence="WEEKLY", overall_rag="GREEN", stage_gate="DVT", generated_at_ms=1795324800000
    )
