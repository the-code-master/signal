# Module: services/parts-registry/src/parts_registry/service.py — 3-Tier Universal Resolver, Drift Checker & Provenance Gate
# Invariants: Resolves Tier-3 Lock -> Tier-2 Org Vault -> Tier-1 Global CAS; blocks grey-market brokers (FEAT-077).
# Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-*.md
from __future__ import annotations

from parts_registry.repository import (
    fetch_parts_registry_record,
    get_global_part,
    get_org_avl_overlay,
    list_org_avl_overlays,
    upsert_org_avl_overlay,
)
from parts_registry.schemas import (
    DistributorProvenanceRequest,
    DistributorProvenanceResponse,
    LockfileDriftItem,
    LockfileDriftRequest,
    LockfileDriftResponse,
    OrgAvlOverlayRecord,
    OrgAvlOverlayUpsertRequest,
    PartsSearchResponse,
    ResolvedUniversalPartResponse,
    ResolveThreeTierPartRequest,
)
from services.common.errors import raise_not_found

AUTHORIZED_FRANCHISE_CODES: frozenset[str] = frozenset(
    {"DIGIKEY", "MOUSER", "ARROW", "AVNET", "TI_DIRECT", "WOLFSPEED_DIRECT", "TTI"}
)


async def execute_parts_registry(org_slug: str) -> PartsSearchResponse:
    return await fetch_parts_registry_record(org_slug)


async def save_org_avl_overlay(
    org_slug: str,
    req: OrgAvlOverlayUpsertRequest,
) -> OrgAvlOverlayRecord:
    record = OrgAvlOverlayRecord(
        org_slug=org_slug,
        vault_repo=f"{org_slug}/signal-org-parts",
        part_urn=req.part_urn,
        mpn=req.mpn,
        internal_ipn=req.internal_ipn,
        avl_status=req.avl_status,
        org_private_nda_taint=req.org_private_nda_taint,
    )
    return await upsert_org_avl_overlay(record)


async def get_all_org_avl_overlays(org_slug: str) -> tuple[OrgAvlOverlayRecord, ...]:
    return await list_org_avl_overlays(org_slug)


async def resolve_three_tier_universal_part(
    org_slug: str,
    req: ResolveThreeTierPartRequest,
) -> ResolvedUniversalPartResponse:
    org_overlay = await get_org_avl_overlay(org_slug, req.part_urn)
    global_part = await get_global_part(req.part_urn)
    if org_overlay is None and global_part is None:
        raise_not_found()

    mpn = org_overlay.mpn if org_overlay is not None else (global_part.mpn if global_part else "UNKNOWN")
    avl_status = org_overlay.avl_status if org_overlay is not None else "APPROVED"
    nda_taint = org_overlay.org_private_nda_taint if org_overlay is not None else False
    rev_hash = global_part.revision_blake3 if global_part is not None else f"b3_org_nda_{org_slug}"
    pad_hash = global_part.padstack_blake3 if global_part is not None else f"b3_org_pad_{org_slug}"
    badge = global_part.badge if global_part is not None else "GOLD_ATTESTED"

    if req.pinned_blake3 is not None and req.pinned_blake3.strip():
        return ResolvedUniversalPartResponse(
            part_urn=req.part_urn,
            mpn=mpn,
            resolved_tier="TIER_3_PROJECT_LOCKFILE",
            revision_blake3=req.pinned_blake3.strip(),
            padstack_blake3=pad_hash,
            verification_badge=badge,
            avl_status=avl_status,
            org_private_nda_taint=nda_taint,
        )
    if org_overlay is not None:
        return ResolvedUniversalPartResponse(
            part_urn=req.part_urn,
            mpn=mpn,
            resolved_tier="TIER_2_ORG_AVL_VAULT",
            revision_blake3=rev_hash,
            padstack_blake3=pad_hash,
            verification_badge=badge,
            avl_status=avl_status,
            org_private_nda_taint=nda_taint,
        )
    return ResolvedUniversalPartResponse(
        part_urn=req.part_urn,
        mpn=mpn,
        resolved_tier="TIER_1_GLOBAL_CAS",
        revision_blake3=rev_hash,
        padstack_blake3=pad_hash,
        verification_badge=badge,
        avl_status=avl_status,
        org_private_nda_taint=False,
    )


async def inspect_parts_lockfile_drift(
    org_slug: str,
    req: LockfileDriftRequest,
) -> LockfileDriftResponse:
    items: list[LockfileDriftItem] = []
    drifted = 0
    for entry in req.entries:
        global_part = await get_global_part(entry.part_urn)
        org_overlay = await get_org_avl_overlay(org_slug, entry.part_urn)
        latest = (
            global_part.revision_blake3
            if global_part is not None
            else (f"b3_org_nda_{org_slug}" if org_overlay is not None else entry.pinned_revision_blake3)
        )
        has_update = latest != entry.pinned_revision_blake3
        if has_update:
            drifted += 1
        items.append(
            LockfileDriftItem(
                part_urn=entry.part_urn,
                pinned_revision_blake3=entry.pinned_revision_blake3,
                latest_upstream_blake3=latest,
                has_upstream_update=has_update,
                board_protected_by_lock=True,
            )
        )
    return LockfileDriftResponse(
        project_slug=req.project_slug,
        drifted_count=drifted,
        items=tuple(items),
    )


def verify_distributor_provenance(req: DistributorProvenanceRequest) -> DistributorProvenanceResponse:
    code = req.distributor_code.strip().upper()
    sig_clean = req.lot_trace_signature.strip()
    if code not in AUTHORIZED_FRANCHISE_CODES:
        return DistributorProvenanceResponse(
            orderable_mpn=req.orderable_mpn,
            distributor_code=code,
            authorized_franchise_verified=False,
            counterfeit_risk_tier="GREY_MARKET_BROKER_BLOCKED",
            procurement_blocked=True,
        )
    if not sig_clean.startswith("lot_sig_") or len(sig_clean) < 20:
        return DistributorProvenanceResponse(
            orderable_mpn=req.orderable_mpn,
            distributor_code=code,
            authorized_franchise_verified=False,
            counterfeit_risk_tier="UNVERIFIED_LOT_WARN",
            procurement_blocked=True,
        )
    return DistributorProvenanceResponse(
        orderable_mpn=req.orderable_mpn,
        distributor_code=code,
        authorized_franchise_verified=True,
        counterfeit_risk_tier="AUTHORIZED_FRANCHISE_VERIFIED",
        procurement_blocked=False,
    )
