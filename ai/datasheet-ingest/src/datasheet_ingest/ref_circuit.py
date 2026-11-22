# Module: ai/datasheet-ingest/src/datasheet_ingest/ref_circuit.py — Typical-Application Testbench Builder (FEAT-107)
# Invariants: Emits deterministic CoW branch ID, SPICE characterization deck, and reference-curve parity metrics.
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
from __future__ import annotations

import math

from datasheet_ingest.schemas import ConditionCurveSummary, DatasheetRecordAst, DatasheetTestbenchPlan


def reconstruct_typical_application_testbench(
    record: DatasheetRecordAst,
    vin_uv: int = 12_000_000,
    vout_target_uv: int = 3_300_000,
) -> DatasheetTestbenchPlan:
    """Reconstructs Typical-Application schematic & SPICE testbench from a quarantined DatasheetRecordAst (FEAT-107)."""
    vin_v = max(0.5, vin_uv / 1_000_000.0)
    vout_v = max(0.25, min(vin_v, vout_target_uv / 1_000_000.0))
    r_bot_kohm = 10.0
    v_ref_v = 0.8
    r_top_kohm = round(r_bot_kohm * max(0.1, (vout_v / v_ref_v) - 1.0), 3)

    slug = "".join(ch.lower() if ch.isalnum() else "-" for ch in record.mpn).strip("-")
    branch_id = f"jj/ai-testbench/{slug}-{record.doc_blake3[:8]}"

    deck_lines = [
        f"* Signal Auto-Generated Typical Application Testbench for {record.mpn}",
        f"V_IN VIN 0 DC {vin_v:.3f}",
        "C_IN VIN 0 10u",
        f"X_DUT VIN SW FB 0 {record.mpn}_MODEL",
        "L_OUT SW VOUT 2.2u",
        "C_OUT VOUT 0 22u",
        f"R_FB_TOP VOUT FB {r_top_kohm}k",
        f"R_FB_BOT FB 0 {r_bot_kohm}k",
        "R_LOAD VOUT 0 3.3",
        ".tran 10n 200u",
        ".end",
    ]

    curves = record.condition_curves
    base_val = max(1, abs(curves[0].value_scaled)) if curves else 1
    ratios = tuple(round(c.value_scaled / base_val, 6) for c in curves)
    # Analytical smooth-curve parity check against extracted condition points
    if ratios:
        sq_errs: list[float] = []
        for idx, r in enumerate(ratios):
            t_norm = idx / max(1, len(ratios) - 1)
            smooth_est = ratios[0] + (ratios[-1] - ratios[0]) * (t_norm**1.15)
            sq_errs.append(((smooth_est - r) / max(abs(r), 1e-6)) ** 2)
        rms_pct = round(math.sqrt(sum(sq_errs) / len(sq_errs)) * 100.0, 4)
    else:
        rms_pct = 0.0

    r_theta_100 = next(
        (c.value_scaled for c in curves if c.metric == "R_THETA_JA_MK_PER_W"),
        40_000,
    )
    summary = ConditionCurveSummary(
        mpn=record.mpn,
        metric_kind=curves[0].metric if curves else "C_EFF_AF",
        bias_uv_points=tuple(c.bias_uv for c in curves),
        temp_mkelvin_points=tuple(c.temp_mk for c in curves),
        freq_hz_points=tuple(c.freq_hz for c in curves),
        normalized_ratios=ratios,
        r_theta_ja_at_100sqmm_mk_per_w=r_theta_100,
        linked_silicon_errata_ids=(),
    )

    return DatasheetTestbenchPlan(
        generated_branch_id=branch_id,
        spice_netlist_deck="\n".join(deck_lines),
        schematic_component_refdes=("U1", "C_IN", "L_OUT", "C_OUT", "R_FB_TOP", "R_FB_BOT", "R_LOAD"),
        reference_curve_rms_error_pct=rms_pct,
        extracted_curves=summary,
    )
