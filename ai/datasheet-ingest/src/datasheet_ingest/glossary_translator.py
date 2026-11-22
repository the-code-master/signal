# Module: ai/datasheet-ingest/src/datasheet_ingest/glossary_translator.py — Quarantined Glossary Translator (FEAT-110)
# Invariants: Sanitizes untrusted text first; 100% verbatim shield for RefDes, NetNames, and SI literals (4.7kΩ, 100nF).
# Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
from __future__ import annotations

import re

from datasheet_ingest.quarantine import sanitize_untrusted_text
from datasheet_ingest.schemas import GlossaryTranslationResult

# Matches SI literals (4.7kΩ, 100nF, 3.3V, 50MHz, 125°C), RefDes (R12, C104, U3), and UPPER_SNAKE NetNames (I2C_SDA, VCC_3V3)
_PROTECTED_TOKEN_RE = re.compile(
    r"(?<![A-Za-z0-9_])(?:"
    r"[+-]?\d+(?:\.\d+)?\s*(?:f|p|n|u|µ|m|k|K|M|Meg|G|T)?(?:Ω|ohm|Ohm|F|H|V|A|W|Hz|s|°C|K)\b"
    r"|(?:R|C|L|U|Q|D|J|SW|FB|TP)\d+[A-Za-z]?\b"
    r"|[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+\b"
    r"|\b(?:VCC|VDD|GND|VSS|VBUS|AGND|DGND|PGND)\b"
    r")"
)

_TECHNICAL_LEXICON: dict[str, dict[str, str]] = {
    "en": {
        "デカップリングコンデンサ": "decoupling capacitor",
        "プルアップ抵抗": "pull-up resistor",
        "の近くに配置": "place close to",
        "過電圧保護": "overvoltage protection",
        "Stützkondensator": "decoupling capacitor",
        "Pull-up-Widerstand": "pull-up resistor",
        "去耦电容": "decoupling capacitor",
        "上拉电阻": "pull-up resistor",
        "靠近放置": "place close to",
    },
    "ja": {
        "decoupling capacitor": "デカップリングコンデンサ",
        "pull-up resistor": "プルアップ抵抗",
        "place close to": "の近くに配置",
        "overvoltage protection": "過電圧保護",
        "connect": "接続",
    },
    "de": {
        "decoupling capacitor": "Stützkondensator",
        "pull-up resistor": "Pull-up-Widerstand",
        "place close to": "nahe platzieren an",
        "overvoltage protection": "Überspannungsschutz",
        "connect": "verbinden",
    },
    "zh-CN": {
        "decoupling capacitor": "去耦电容",
        "pull-up resistor": "上拉电阻",
        "place close to": "靠近放置",
        "overvoltage protection": "过压保护",
        "connect": "连接",
    },
}


def translate_quarantined_technical_text(
    source_text: str,
    target_locale: str,
) -> GlossaryTranslationResult:
    """Translates technical notes in Zero-Tool Quarantine while preserving RefDes, NetName, and SI tokens (FEAT-110)."""
    sanitized, warnings = sanitize_untrusted_text(source_text)

    preserved: list[str] = []

    def _shield(match: re.Match[str]) -> str:
        tok = match.group(0)
        idx = len(preserved)
        preserved.append(tok)
        return f"__SIGTOK_{idx}__"

    shielded = _PROTECTED_TOKEN_RE.sub(_shield, sanitized)

    lexicon = _TECHNICAL_LEXICON.get(target_locale, _TECHNICAL_LEXICON["en"])
    translated = shielded
    for src_phrase, dst_phrase in sorted(lexicon.items(), key=lambda kv: len(kv[0]), reverse=True):
        pattern = re.compile(re.escape(src_phrase), flags=re.IGNORECASE)
        translated = pattern.sub(dst_phrase, translated)

    for idx, tok in enumerate(preserved):
        placeholder = f"__SIGTOK_{idx}__"
        translated = translated.replace(placeholder, tok)
        if tok not in translated:
            raise RuntimeError(f"Protected EDA token `{tok}` lost during glossary translation")

    return GlossaryTranslationResult(
        translated_text=translated,
        preserved_refdes_and_si_tokens=tuple(preserved),
        quarantine_warnings=warnings,
    )
