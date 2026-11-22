# Module: ai/rag-service/src/rag_service/mirrored_bom.py — Mirrored Multi-Distributor Catalog & k=256 Chaff Batcher
# Invariants: Queries in-cluster mirror first; external delta batches fixed at k=256 shuffled MPNs (I-09, F-061, F-146).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/16-*
from __future__ import annotations

import hashlib
import hmac

from rag_service.schemas import MirroredDistributorOffer, PaddedExternalBatchQuery

CHAFF_PADDED_BATCH_SIZE = 256

_DECOY_PREFIXES: dict[str, tuple[str, ...]] = {
    "PASSIVE": ("RC0402FR-07", "GRM155R71C", "CL05B104KO", "ERJ-2RKF"),
    "POWER": ("TPS6284", "LM5164Q", "LT8610A", "MP2315G"),
    "MCU": ("STM32G47", "RP2350A", "ATSAMD21G", "NRF52840"),
}


def build_k256_chaff_padded_batch(
    real_mpns: tuple[str, ...] | list[str],
    *,
    category_hint: str = "PASSIVE",
    batch_hmac_seed: bytes = b"signal-k256-chaff-seed",
) -> PaddedExternalBatchQuery:
    """Pads real MPNs with deterministic category-matched chaff MPNs to exact k=256 and shuffles via HMAC-DRBG (I-09)."""
    dedup_real = list(dict.fromkeys(m.strip().upper() for m in real_mpns if m.strip()))
    if not dedup_real:
        raise ValueError("real_mpns must contain at least one MPN")
    if len(dedup_real) > CHAFF_PADDED_BATCH_SIZE:
        raise ValueError(f"single external batch cannot exceed {CHAFF_PADDED_BATCH_SIZE} MPNs")

    prefixes = _DECOY_PREFIXES.get(category_hint.upper(), _DECOY_PREFIXES["PASSIVE"])
    seen = set(dedup_real)
    chaff: list[str] = []
    idx = 0
    while len(dedup_real) + len(chaff) < CHAFF_PADDED_BATCH_SIZE:
        pfx = prefixes[idx % len(prefixes)]
        digest = hmac.new(batch_hmac_seed, f"chaff:{idx}".encode(), hashlib.sha256).hexdigest()[:4].upper()
        candidate = f"{pfx}{digest}"
        idx += 1
        if candidate not in seen:
            seen.add(candidate)
            chaff.append(candidate)

    combined = [*dedup_real, *chaff]
    # Deterministic Fisher-Yates shuffle keyed by batch_hmac_seed
    for i in range(len(combined) - 1, 0, -1):
        prf = hmac.new(batch_hmac_seed, f"perm:{i}".encode(), hashlib.sha256).digest()
        j = int.from_bytes(prf[:4], "big") % (i + 1)
        combined[i], combined[j] = combined[j], combined[i]

    return PaddedExternalBatchQuery(
        shuffled_mpns=tuple(combined),
        real_mpn_count=len(dedup_real),
        chaff_mpn_count=len(chaff),
    )


class MirroredDistributorCatalog:
    """In-cluster mirrored multi-distributor catalog with k=256 chaff-padded external fallback planner (I-09, F-146)."""

    def __init__(self) -> None:
        self._offers_by_mpn: dict[str, list[MirroredDistributorOffer]] = {}

    def ingest_mirror_snapshot(self, offers: tuple[MirroredDistributorOffer, ...] | list[MirroredDistributorOffer]) -> None:
        for offer in offers:
            key = offer.mpn.strip().upper()
            bucket = self._offers_by_mpn.setdefault(key, [])
            bucket.append(offer)

    def lookup_bom_with_privacy_guard(
        self,
        requested_mpns: tuple[str, ...] | list[str],
        *,
        category_hint: str = "PASSIVE",
        batch_hmac_seed: bytes = b"signal-k256-chaff-seed",
    ) -> tuple[tuple[MirroredDistributorOffer, ...], PaddedExternalBatchQuery | None]:
        matched_offers: list[MirroredDistributorOffer] = []
        cache_misses: list[str] = []

        for raw in requested_mpns:
            key = raw.strip().upper()
            if not key:
                continue
            offers = self._offers_by_mpn.get(key)
            if offers:
                matched_offers.extend(offers)
            else:
                cache_misses.append(key)

        padded_batch = (
            build_k256_chaff_padded_batch(cache_misses, category_hint=category_hint, batch_hmac_seed=batch_hmac_seed)
            if cache_misses
            else None
        )
        return tuple(matched_offers), padded_batch
