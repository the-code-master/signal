# Module: services/parts-registry/src/parts_registry/schemas.py — 3-Tier Parts, Org AVL & Provenance Schemas
# Invariants: StrictSchema (extra="forbid", frozen=True); Org NDA AVL overlays never enter Tier-1 Global catalog (18-*).
# Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-*.md
from __future__ import annotations

from typing import Literal

from services.common.schemas import Int64Str, StrictSchema

AvlLifecycleStatus = Literal["PREFERRED", "APPROVED", "NRND", "OBSOLETE"]
VerificationBadgeTier = Literal["BRONZE_PARSED", "SILVER_SIM_MATCHED", "GOLD_ATTESTED"]
CounterfeitRiskTier = Literal["AUTHORIZED_FRANCHISE_VERIFIED", "UNVERIFIED_LOT_WARN", "GREY_MARKET_BROKER_BLOCKED"]


class PartsSearchResponse(StrictSchema):
    part_urn: str
    mpn: str
    verification_tier: str
    avl_status: str


class OrgAvlOverlayUpsertRequest(StrictSchema):
    part_urn: str
    mpn: str
    internal_ipn: str
    avl_status: AvlLifecycleStatus
    org_private_nda_taint: bool = True


class OrgAvlOverlayRecord(StrictSchema):
    org_slug: str
    vault_repo: str
    part_urn: str
    mpn: str
    internal_ipn: str
    avl_status: AvlLifecycleStatus
    org_private_nda_taint: bool


class ResolveThreeTierPartRequest(StrictSchema):
    part_urn: str
    pinned_blake3: str | None = None


class ResolvedUniversalPartResponse(StrictSchema):
    part_urn: str
    mpn: str
    resolved_tier: Literal["TIER_1_GLOBAL_CAS", "TIER_2_ORG_AVL_VAULT", "TIER_3_PROJECT_LOCKFILE"]
    revision_blake3: str
    padstack_blake3: str
    verification_badge: VerificationBadgeTier
    avl_status: AvlLifecycleStatus
    org_private_nda_taint: bool


class LockfileEntryCheck(StrictSchema):
    part_urn: str
    pinned_revision_blake3: str
    pinned_padstack_blake3: str


class LockfileDriftRequest(StrictSchema):
    project_slug: str
    locked_at_utc_ms: Int64Str
    entries: tuple[LockfileEntryCheck, ...]


class LockfileDriftItem(StrictSchema):
    part_urn: str
    pinned_revision_blake3: str
    latest_upstream_blake3: str
    has_upstream_update: bool
    board_protected_by_lock: bool


class LockfileDriftResponse(StrictSchema):
    project_slug: str
    drifted_count: int
    items: tuple[LockfileDriftItem, ...]


class DistributorProvenanceRequest(StrictSchema):
    orderable_mpn: str
    distributor_code: str
    lot_trace_signature: str


class DistributorProvenanceResponse(StrictSchema):
    orderable_mpn: str
    distributor_code: str
    authorized_franchise_verified: bool
    counterfeit_risk_tier: CounterfeitRiskTier
    procurement_blocked: bool
