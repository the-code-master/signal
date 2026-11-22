# Module: ai/rag-service/src/rag_service/bom_consolidator.py — Greedy Set-Cover SMT Feeder Minimizer (FEAT-108)
# Invariants: Never downgrades voltage rating; verifies Saltation-Adjoint metric shift <= max_allowed_metric_shift_pct.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
from __future__ import annotations

from rag_service.schemas import BomConsolidationReport, BomLineItemSpec


def _can_cover_placement(
    candidate_reel: BomLineItemSpec,
    target_item: BomLineItemSpec,
    max_allowed_metric_shift_pct: float,
) -> bool:
    """Checks electrical/mechanical substitutability and first-order adjoint sensitivity shift bound."""
    if candidate_reel.component_kind != target_item.component_kind:
        return False
    if candidate_reel.component_kind == "IC":
        return candidate_reel.mpn == target_item.mpn
    if candidate_reel.package_code != target_item.package_code:
        return False
    if candidate_reel.min_voltage_rating_v < target_item.min_voltage_rating_v:
        return False

    denom = max(abs(target_item.nominal_value_si), 1e-18)
    rel_value_delta_pct = abs(candidate_reel.nominal_value_si - target_item.nominal_value_si) * 100.0 / denom
    # First-order adjoint metric shift: |dM/M| ~= |S_p^M| * |dp/p|
    predicted_metric_shift_pct = target_item.critical_loop_sensitivity * rel_value_delta_pct
    return rel_value_delta_pct <= 20.0 and predicted_metric_shift_pct <= max_allowed_metric_shift_pct


def consolidate_bom_feeder_slots(
    items: tuple[BomLineItemSpec, ...] | list[BomLineItemSpec],
    max_allowed_metric_shift_pct: float = 1.5,
) -> BomConsolidationReport:
    """Minimizes unique SMT pick-and-place feeder slots via greedy set-cover with adjoint shift guard (FEAT-108)."""
    if not items:
        return BomConsolidationReport(
            unique_line_items_before=0,
            unique_line_items_after=0,
            smt_feeder_slots_saved=0,
            adjoint_zero_shift_verified=True,
            replacement_map=(),
        )

    unique_before = len({item.mpn for item in items})
    uncovered_indices: set[int] = set(range(len(items)))
    candidate_reels = list(items)
    replacements: dict[int, str] = {}
    chosen_mpns: set[str] = set()

    while uncovered_indices:
        best_reel: BomLineItemSpec | None = None
        best_covered: set[int] = set()

        for cand in candidate_reels:
            covered = {
                idx
                for idx in uncovered_indices
                if _can_cover_placement(cand, items[idx], max_allowed_metric_shift_pct)
            }
            # Prefer highest coverage, then higher voltage rating, then lexicographically stable MPN
            if len(covered) > len(best_covered) or (
                len(covered) == len(best_covered)
                and best_reel is not None
                and (cand.min_voltage_rating_v, cand.mpn) > (best_reel.min_voltage_rating_v, best_reel.mpn)
            ):
                best_reel = cand
                best_covered = covered

        if best_reel is None or not best_covered:
            idx = min(uncovered_indices)
            best_reel = items[idx]
            best_covered = {idx}

        chosen_mpns.add(best_reel.mpn)
        for idx in best_covered:
            replacements[idx] = best_reel.mpn
            uncovered_indices.remove(idx)

    unique_after = len(chosen_mpns)
    rep_pairs = tuple(
        (items[idx].refdes, replacements[idx])
        for idx in sorted(replacements)
        if replacements[idx] != items[idx].mpn
    )

    return BomConsolidationReport(
        unique_line_items_before=unique_before,
        unique_line_items_after=unique_after,
        smt_feeder_slots_saved=max(0, unique_before - unique_after),
        adjoint_zero_shift_verified=True,
        replacement_map=rep_pairs,
    )
