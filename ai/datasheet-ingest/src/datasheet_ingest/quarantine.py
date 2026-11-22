# Module: ai/datasheet-ingest/src/datasheet_ingest/quarantine.py — Multimodal VLM + Dual-LLM Zero-Tool Quarantine
# Invariants: Zero tool-calling authority (allowed_tools == ()); strips invisible unicode & image injection overlays (F-131).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/15-application-security-and-project-isolation.md
from __future__ import annotations

import re
from typing import Any, ClassVar

from datasheet_ingest.schemas import (
    ConditionCurvePointAst,
    DatasheetPinAst,
    DatasheetRecordAst,
    StrictSchema,
)

MAX_NORMALIZED_IMAGE_DIM_PX = 2048

_ZERO_WIDTH_AND_BIDI_RE = re.compile(r"[\u200b-\u200f\u202a-\u202e\u2060-\u2069\ufeff]")
_HTML_COMMENT_RE = re.compile(r"<!--.*?-->", flags=re.DOTALL)
_PROMPT_INJECTION_PATTERNS: tuple[tuple[str, re.Pattern[str]], ...] = (
    ("IGNORE_INSTRUCTIONS", re.compile(r"ignore\s+(?:all\s+)?(?:previous|prior)\s+instructions", re.IGNORECASE)),
    ("SYSTEM_OVERRIDE", re.compile(r"(?:system\s*prompt|developer\s*mode|jailbreak)\s*:", re.IGNORECASE)),
    ("TOOL_INVOCATION_INJECTION", re.compile(r"(?:call|invoke|execute)\s+(?:tool|sftp|webhook|shell)", re.IGNORECASE)),
    ("EXFILTRATION_DIRECTIVE", re.compile(r"(?:exfiltrate|upload\s+secret|send\s+netlist\s+to)", re.IGNORECASE)),
)


class QuarantineSecurityViolation(RuntimeError):
    """Raised when code attempts to grant or invoke tools inside the Zero-Tool Quarantine tier."""


class NormalizedImageEnvelope(StrictSchema):
    width_px: int
    height_px: int
    normalized_luma_u8: tuple[int, ...]
    exif_stripped: bool
    quarantine_warnings: tuple[str, ...]


def sanitize_untrusted_text(raw_text: str) -> tuple[str, tuple[str, ...]]:
    """Strips invisible Unicode/BiDi overrides, HTML comments, and imperative prompt-injection clauses (F-090, F-131)."""
    warnings: list[str] = []
    cleaned = _ZERO_WIDTH_AND_BIDI_RE.sub("", raw_text)
    if cleaned != raw_text:
        warnings.append("STRIPPED_ZERO_WIDTH_OR_BIDI_UNICODE")

    without_comments = _HTML_COMMENT_RE.sub(" ", cleaned)
    if without_comments != cleaned:
        warnings.append("STRIPPED_HIDDEN_MARKUP_COMMENT")
        cleaned = without_comments

    for code, pattern in _PROMPT_INJECTION_PATTERNS:
        if pattern.search(cleaned):
            warnings.append(f"NEUTRALIZED_{code}")
            cleaned = pattern.sub("[QUARANTINE_REDACTED]", cleaned)

    collapsed = re.sub(r"[ \t]+", " ", cleaned).strip()
    return collapsed, tuple(warnings)


def normalize_and_sanitize_image_frame(
    width_px: int,
    height_px: int,
    pixel_luma_u8: tuple[int, ...] | list[int],
    embedded_metadata_text: str = "",
) -> NormalizedImageEnvelope:
    """Rescales image dimensions <= 2048 px, strips EXIF/XMP text, and applies p5..p95 contrast normalization (F-131)."""
    if width_px <= 0 or height_px <= 0 or not pixel_luma_u8:
        raise ValueError("image dimensions and pixel buffer must be non-empty")

    warnings: list[str] = []
    if embedded_metadata_text.strip():
        _, meta_warns = sanitize_untrusted_text(embedded_metadata_text)
        warnings.append("STRIPPED_IMAGE_EXIF_XMP_METADATA")
        warnings.extend(meta_warns)

    scale = min(1.0, MAX_NORMALIZED_IMAGE_DIM_PX / max(width_px, height_px))
    out_w = max(1, round(width_px * scale))
    out_h = max(1, round(height_px * scale))

    clamped = [max(0, min(255, int(v))) for v in pixel_luma_u8]
    sorted_vals = sorted(clamped)
    n = len(sorted_vals)
    p5 = sorted_vals[int(0.05 * (n - 1))]
    p95 = sorted_vals[int(0.95 * (n - 1))]
    span = max(1, p95 - p5)

    # Low-contrast micro-contrast overlays (< 6 gray levels across 90% of pixels) are flattened & flagged
    if p95 - p5 < 6 and max(sorted_vals) - min(sorted_vals) > 0:
        warnings.append("NEUTRALIZED_SUB_THRESHOLD_CONTRAST_OVERLAY")
        normalized = tuple(128 for _ in clamped)
    else:
        normalized = tuple(max(0, min(255, round((v - p5) * 255 / span))) for v in clamped)

    return NormalizedImageEnvelope(
        width_px=out_w,
        height_px=out_h,
        normalized_luma_u8=normalized,
        exif_stripped=True,
        quarantine_warnings=tuple(warnings),
    )


class QuarantineReader:
    """Zero-Tool Quarantine extractor emitting strictly typed DatasheetRecordAst records (F-090, F-131)."""

    allowed_tools: ClassVar[tuple[str, ...]] = ()

    def register_tool(self, _tool_name: str, _fn: Any) -> None:
        raise QuarantineSecurityViolation("Zero-Tool Quarantine strictly forbids tool registration (F-090)")

    def invoke_tool(self, _tool_name: str, **_kwargs: Any) -> Any:
        raise QuarantineSecurityViolation("Zero-Tool Quarantine strictly forbids tool execution (F-090)")

    def extract_schema_locked_record(
        self,
        *,
        mpn: str,
        manufacturer: str,
        doc_blake3: str,
        pins: tuple[DatasheetPinAst, ...],
        condition_curves: tuple[ConditionCurvePointAst, ...],
        raw_ocr_notes: str = "",
    ) -> tuple[DatasheetRecordAst, tuple[str, ...]]:
        if self.allowed_tools:
            raise QuarantineSecurityViolation("QuarantineReader tool list was mutated")
        clean_mpn, w_mpn = sanitize_untrusted_text(mpn)
        clean_mfg, w_mfg = sanitize_untrusted_text(manufacturer)
        _, w_notes = sanitize_untrusted_text(raw_ocr_notes)
        sanitized_pins: list[DatasheetPinAst] = []
        pin_warnings: list[str] = []
        for p in pins:
            sig_clean, w_sig = sanitize_untrusted_text(p.signal_name)
            pin_warnings.extend(w_sig)
            sanitized_pins.append(
                DatasheetPinAst(
                    pin_number=p.pin_number.strip(),
                    signal_name=sig_clean or "SANITIZED_PIN",
                    electrical_class=p.electrical_class,
                    abs_max_uv=p.abs_max_uv,
                    citation=p.citation,
                )
            )
        record = DatasheetRecordAst(
            mpn=clean_mpn,
            manufacturer=clean_mfg,
            doc_blake3=doc_blake3,
            pins=tuple(sanitized_pins),
            condition_curves=condition_curves,
        )
        all_warnings = tuple(dict.fromkeys([*w_mpn, *w_mfg, *w_notes, *pin_warnings]))
        return record, all_warnings
