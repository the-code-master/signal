# Module: ai/datasheet-ingest/src/datasheet_ingest/revision_diff.py — Datasheet Rev A -> Rev B Drift Analyzer (FEAT-078)
# Invariants: Deterministically flags pinout reassignments, tightened abs-max voltage limits, and derating curve shifts.
# Spec: architecture/17-extended-feature-specifications-ui-and-rust.md, architecture/18-*
from __future__ import annotations

from datasheet_ingest.schemas import DatasheetRecordAst, DatasheetRevisionDiff


def diff_datasheet_revisions(
    old_record: DatasheetRecordAst,
    new_record: DatasheetRecordAst,
) -> DatasheetRevisionDiff:
    """Computes pinout, absolute-maximum, and parametric derating diffs across datasheet revisions (FEAT-078)."""
    old_pins = {p.pin_number: p for p in old_record.pins}
    new_pins = {p.pin_number: p for p in new_record.pins}

    changed_pins = 0
    tightened_abs_max = 0
    summaries: list[str] = []

    all_pin_nums = sorted(set(old_pins) | set(new_pins))
    for pin_num in all_pin_nums:
        op = old_pins.get(pin_num)
        np = new_pins.get(pin_num)
        if op is None and np is not None:
            changed_pins += 1
            summaries.append(f"Pin {pin_num} ({np.signal_name}) added in new revision")
        elif op is not None and np is None:
            changed_pins += 1
            summaries.append(f"Pin {pin_num} ({op.signal_name}) removed in new revision")
        elif op is not None and np is not None:
            if op.signal_name != np.signal_name or op.electrical_class != np.electrical_class:
                changed_pins += 1
                summaries.append(
                    f"Pin {pin_num} changed from {op.signal_name}/{op.electrical_class} "
                    f"to {np.signal_name}/{np.electrical_class}"
                )
            if abs(np.abs_max_uv) < abs(op.abs_max_uv):
                tightened_abs_max += 1
                summaries.append(
                    f"Pin {pin_num} ({np.signal_name}) abs-max tightened from {op.abs_max_uv} uV to {np.abs_max_uv} uV"
                )

    old_curves = {(c.metric, c.temp_mk, c.bias_uv, c.freq_hz): c.value_scaled for c in old_record.condition_curves}
    for nc in new_record.condition_curves:
        key = (nc.metric, nc.temp_mk, nc.bias_uv, nc.freq_hz)
        old_val = old_curves.get(key)
        if old_val is not None and old_val != nc.value_scaled:
            pct = (nc.value_scaled - old_val) * 100.0 / max(abs(old_val), 1)
            if abs(pct) >= 1.0:
                summaries.append(f"Curve {nc.metric} @ {nc.bias_uv} uV shifted by {pct:+.2f}%")

    return DatasheetRevisionDiff(
        mpn=new_record.mpn,
        old_doc_blake3=old_record.doc_blake3,
        new_doc_blake3=new_record.doc_blake3,
        changed_pin_count=changed_pins,
        tightened_abs_max_count=tightened_abs_max,
        parameter_delta_summaries=tuple(summaries),
    )
