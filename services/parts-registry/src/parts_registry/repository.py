# Module: services/parts-registry/src/parts_registry/repository.py — Global Tier-1 CAS Index & Isolated Tier-2 Org AVL Store
# Invariants: Global catalog never receives Tier-2 Org AVL mutations; Org AVL keyed strictly by org_slug (18-*).
# Spec: architecture/18-universal-component-and-datasheet-repository.md
from __future__ import annotations

from dataclasses import dataclass

from parts_registry.schemas import OrgAvlOverlayRecord, PartsSearchResponse, VerificationBadgeTier


@dataclass(frozen=True)
class GlobalCatalogEntry:
    part_urn: str
    mpn: str
    revision_blake3: str
    padstack_blake3: str
    badge: VerificationBadgeTier


_GLOBAL_TIER1_CATALOG: dict[str, GlobalCatalogEntry] = {
    "urn:signal:part:wolfspeed:C3M0016120K": GlobalCatalogEntry(
        part_urn="urn:signal:part:wolfspeed:C3M0016120K",
        mpn="C3M0016120K",
        revision_blake3="b3_rev_c3m0016120k_v2",
        padstack_blake3="b3_pad_to247_4l_kelvin",
        badge="GOLD_ATTESTED",
    ),
    "urn:signal:part:ti:TPS546D24ARVFR": GlobalCatalogEntry(
        part_urn="urn:signal:part:ti:TPS546D24ARVFR",
        mpn="TPS546D24ARVFR",
        revision_blake3="b3_rev_tps546d24a_v4",
        padstack_blake3="b3_pad_lqfn_clip_40",
        badge="GOLD_ATTESTED",
    ),
}

_ORG_TIER2_AVL_STORE: dict[str, dict[str, OrgAvlOverlayRecord]] = {}


async def fetch_parts_registry_record(org_slug: str) -> PartsSearchResponse:
    _ = org_slug
    return PartsSearchResponse(
        part_urn="urn:signal:part:wolfspeed:C3M0016120K",
        mpn="C3M0016120K",
        verification_tier="GOLD",
        avl_status="PREFERRED",
    )


async def get_global_part(part_urn: str) -> GlobalCatalogEntry | None:
    return _GLOBAL_TIER1_CATALOG.get(part_urn)


def global_catalog_contains(part_urn: str) -> bool:
    return part_urn in _GLOBAL_TIER1_CATALOG


async def upsert_org_avl_overlay(record: OrgAvlOverlayRecord) -> OrgAvlOverlayRecord:
    org_bucket = _ORG_TIER2_AVL_STORE.setdefault(record.org_slug, {})
    org_bucket[record.part_urn] = record
    return record


async def get_org_avl_overlay(org_slug: str, part_urn: str) -> OrgAvlOverlayRecord | None:
    return _ORG_TIER2_AVL_STORE.get(org_slug, {}).get(part_urn)


async def list_org_avl_overlays(org_slug: str) -> tuple[OrgAvlOverlayRecord, ...]:
    return tuple(_ORG_TIER2_AVL_STORE.get(org_slug, {}).values())
