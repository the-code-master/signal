# Module: ai/datasheet-ingest/src/datasheet_ingest/consensus_curves.py — Dual-VLM Consensus & Condition Curve Extractor
# Invariants: Flags >2% VLM-A vs VLM-B pin/rating/curve divergence for human review; links Silicon Errata & PCN (F-132).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/18-universal-component-and-datasheet-repository.md
from __future__ import annotations

import math

from datasheet_ingest.schemas import (
    ConditionCurvePointAst,
    ConditionCurveSummary,
    DatasheetPinAst,
    DatasheetRecordAst,
    DualVlmConsensusReport,
    PcnPdnBulletin,
    SiliconErrataBulletin,
)


def mlcc_dc_bias_c_ratio(bias_v: float, v_rated_v: float, temp_c: float = 25.0) -> float:
    """Class-II MLCC nonlinear C(V_bias, T)/C_0 derating curve (F-063, F-132)."""
    v_knee = max(0.5, 0.45 * abs(v_rated_v))
    dc_factor = 0.20 + 0.80 / (1.0 + (abs(bias_v) / v_knee) ** 2.0)
    temp_delta = (temp_c - 25.0) / 100.0
    temp_factor = max(0.70, 1.0 - 0.12 * (temp_delta * temp_delta))
    return round(max(0.10, min(1.05, dc_factor * temp_factor)), 6)


def inductor_isat_l_ratio(dc_current_a: float, i_sat_30pct_a: float) -> float:
    """Soft-saturating power inductor L(I_dc)/L_0 curve matching datasheet 30% drop at I_sat (F-132)."""
    isat = max(1e-3, abs(i_sat_30pct_a))
    # At I == I_sat, ratio == 1 / (1 + 0.42857 * 1^2) ~= 0.70
    ratio = 0.15 + 0.85 / (1.0 + 0.541667 * ((abs(dc_current_a) / isat) ** 2.2))
    return round(max(0.15, min(1.0, ratio)), 6)


def mosfet_rds_on_uohm(rds25_uohm: int, tj_mkelvin: int, vgs_uv: int, vth_uv: int = 1_800_000) -> int:
    """Temperature- and overdrive-coupled MOSFET R_DS(on)(T_j, V_GS) in micro-ohms (F-132)."""
    tj_k = max(200.0, tj_mkelvin / 1_000.0)
    temp_mult = (tj_k / 298.15) ** 1.65
    overdrive_v = max(0.35, (vgs_uv - vth_uv) / 1_000_000.0)
    gate_mult = 1.0 + 0.45 / overdrive_v
    return max(1, round(rds25_uohm * temp_mult * gate_mult))


def copper_area_r_theta_ja_mk_per_w(
    r_theta_jc_mk_per_w: int,
    r_theta_ja_100sqmm_mk_per_w: int,
    copper_area_sq_mm: float,
) -> int:
    """PCB copper-pour-coupled R_thetaJA(A_cu) scaling relative to JEDEC 100 mm^2 coupon (F-132)."""
    area = max(10.0, min(2500.0, copper_area_sq_mm))
    spread_100 = max(0, r_theta_ja_100sqmm_mk_per_w - r_theta_jc_mk_per_w)
    scaled_spread = spread_100 * math.sqrt(100.0 / area)
    return round(r_theta_jc_mk_per_w + scaled_spread)


def _rel_divergence_pct(a: int, b: int) -> float:
    denom = max(abs(a), abs(b), 1)
    return abs(a - b) * 100.0 / denom


def reconcile_dual_vlm_extractions(
    vlm_a: DatasheetRecordAst,
    vlm_b: DatasheetRecordAst,
    errata_bulletins: tuple[SiliconErrataBulletin, ...] = (),
    pcn_bulletins: tuple[PcnPdnBulletin, ...] = (),
    divergence_threshold_pct: float = 2.0,
) -> DualVlmConsensusReport:
    """Reconciles independent VLM-A and VLM-B extractions, flagging >2% divergence for human review (F-052, F-132)."""
    reasons: list[str] = []
    max_div = 0.0

    pins_b_by_num = {p.pin_number: p for p in vlm_b.pins}
    if len(vlm_a.pins) != len(vlm_b.pins):
        reasons.append(f"PIN_COUNT_MISMATCH:{len(vlm_a.pins)}_vs_{len(vlm_b.pins)}")
        max_div = max(max_div, 100.0)

    merged_pins: list[DatasheetPinAst] = []
    for pa in vlm_a.pins:
        pb = pins_b_by_num.get(pa.pin_number)
        if pb is None:
            reasons.append(f"MISSING_PIN_IN_VLM_B:{pa.pin_number}")
            max_div = max(max_div, 100.0)
            merged_pins.append(pa)
            continue
        if pa.electrical_class != pb.electrical_class:
            reasons.append(f"ELECTRICAL_CLASS_MISMATCH:pin_{pa.pin_number}:{pa.electrical_class}_vs_{pb.electrical_class}")
            max_div = max(max_div, 100.0)
        div = _rel_divergence_pct(pa.abs_max_uv, pb.abs_max_uv)
        max_div = max(max_div, div)
        if div > divergence_threshold_pct:
            reasons.append(f"ABS_MAX_DIVERGENCE:pin_{pa.pin_number}:{div:.2f}%")
        # Conservative safe bound: lower magnitude abs-max rating
        conservative_abs_uv = pa.abs_max_uv if abs(pa.abs_max_uv) <= abs(pb.abs_max_uv) else pb.abs_max_uv
        merged_pins.append(
            DatasheetPinAst(
                pin_number=pa.pin_number,
                signal_name=pa.signal_name,
                electrical_class=pa.electrical_class,
                abs_max_uv=conservative_abs_uv,
                citation=pa.citation,
            )
        )

    merged_curves: list[ConditionCurvePointAst] = []
    if len(vlm_a.condition_curves) != len(vlm_b.condition_curves):
        reasons.append("CURVE_POINT_COUNT_MISMATCH")
        max_div = max(max_div, 100.0)
        merged_curves.extend(vlm_a.condition_curves)
    else:
        for ca, cb in zip(vlm_a.condition_curves, vlm_b.condition_curves, strict=True):
            c_div = _rel_divergence_pct(ca.value_scaled, cb.value_scaled)
            max_div = max(max_div, c_div)
            if ca.metric != cb.metric or c_div > divergence_threshold_pct:
                reasons.append(f"CURVE_DIVERGENCE:{ca.metric}@{ca.bias_uv}uV:{c_div:.2f}%")
            merged_curves.append(
                ConditionCurvePointAst(
                    metric=ca.metric,
                    temp_mk=ca.temp_mk,
                    bias_uv=ca.bias_uv,
                    freq_hz=ca.freq_hz,
                    value_scaled=round((ca.value_scaled + cb.value_scaled) / 2),
                )
            )

    reconciled = DatasheetRecordAst(
        mpn=vlm_a.mpn,
        manufacturer=vlm_a.manufacturer,
        doc_blake3=vlm_a.doc_blake3,
        pins=tuple(merged_pins),
        condition_curves=tuple(merged_curves),
    )

    baseline_val = max(1, abs(merged_curves[0].value_scaled)) if merged_curves else 1
    r_theta_100 = next(
        (c.value_scaled for c in merged_curves if c.metric == "R_THETA_JA_MK_PER_W"),
        42_000,
    )
    summary = ConditionCurveSummary(
        mpn=reconciled.mpn,
        metric_kind=merged_curves[0].metric if merged_curves else "C_EFF_AF",
        bias_uv_points=tuple(c.bias_uv for c in merged_curves),
        temp_mkelvin_points=tuple(c.temp_mk for c in merged_curves),
        freq_hz_points=tuple(c.freq_hz for c in merged_curves),
        normalized_ratios=tuple(round(c.value_scaled / baseline_val, 6) for c in merged_curves),
        r_theta_ja_at_100sqmm_mk_per_w=r_theta_100,
        linked_silicon_errata_ids=tuple(e.errata_id for e in errata_bulletins),
    )

    return DualVlmConsensusReport(
        reconciled_record=reconciled,
        needs_human_review=bool(reasons),
        max_observed_divergence_pct=round(max_div, 4),
        discrepancy_reasons=tuple(reasons),
        condition_curve_summary=summary,
        linked_pcn_ids=tuple(p.bulletin_id for p in pcn_bulletins),
    )
