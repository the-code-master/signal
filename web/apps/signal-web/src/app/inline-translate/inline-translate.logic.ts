// Module: web/apps/signal-web/src/app/inline-translate/inline-translate.logic.ts — FEAT-110 Quarantined Glossary Translator Logic
// Invariants: Masks RefDes, NetName & SI literals before glossary lookup so 100% of engineering tokens stay verbatim.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, contracts/proto/signal/v1/ai_rag_formal.proto
import type {
  InlineTranslateVm,
  QuarantinedTranslationResult,
  TechnicalLocaleCode,
} from "./inline-translate.model";

export const SUPPORTED_TECHNICAL_LOCALES: readonly TechnicalLocaleCode[] = [
  "ja",
  "de",
  "zh-TW",
  "ko",
  "fr",
  "es",
  "it",
  "pt",
  "nl",
  "sv",
  "pl",
  "cs",
  "hi",
  "vi",
  "ar",
];

interface LocaleGlossaryMap {
  readonly route: string;
  readonly place: string;
  readonly clearance: string;
  readonly near: string;
}

const LOCALE_PHRASE_PREFIX: Readonly<Record<TechnicalLocaleCode, LocaleGlossaryMap>> = {
  ja: { route: "配線", place: "配置", clearance: "沿面距離", near: "近傍" },
  de: { route: "Leiterbahn", place: "Platziere", clearance: "Kriechstrecke", near: "nahe" },
  "zh-TW": { route: "佈線", place: "放置", clearance: "爬電距離", near: "靠近" },
  ko: { route: "배선", place: "배치", clearance: "연면거리", near: "근접" },
  fr: { route: "Router", place: "Placer", clearance: "Ligne de fuite", near: "près de" },
  es: { route: "Enrutar", place: "Colocar", clearance: "Distancia de fuga", near: "cerca de" },
  it: { route: "Instradare", place: "Posizionare", clearance: "Distanza superficiale", near: "vicino a" },
  pt: { route: "Rotear", place: "Posicionar", clearance: "Distância de escoamento", near: "próximo a" },
  nl: { route: "Routeer", place: "Plaats", clearance: "Kruipweg", near: "nabij" },
  sv: { route: "Dra ledare", place: "Placera", clearance: "Krypavstånd", near: "nära" },
  pl: { route: "Poprowadź", place: "Umieść", clearance: "Droga upływu", near: "blisko" },
  cs: { route: "Vézt spoj", place: "Umístit", clearance: "Povrchová cesta", near: "blízko" },
  hi: { route: "रूट करें", place: "रखें", clearance: "क्रीपेज दूरी", near: "के पास" },
  vi: { route: "Đi dây", place: "Đặt", clearance: "Khoảng cách rò", near: "gần" },
  ar: { route: "توجيه المسار", place: "وضع", clearance: "مسافة الزحف", near: "بالقرب من" },
};

const PROTECTED_TOKEN_RE =
  /(?<![A-Za-z0-9_])(?:[A-Z]{1,3}\d+[A-Z0-9_]*|[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+|\d+(?:\.\d+)?(?:nm|um|mm|ps|fs|ns|us|ms|uV|mV|V|kV|uA|mA|A|mΩ|Ω|kΩ|MΩ|Hz|kHz|MHz|GHz|pF|nF|uF|nH|uH)(?:\/ns)?)(?![A-Za-z0-9_])/g;

export function extractProtectedEngineeringTokens(sourceText: string): readonly string[] {
  const seen = new Set<string>();
  for (const match of sourceText.matchAll(PROTECTED_TOKEN_RE)) {
    seen.add(match[0]);
  }
  return [...seen];
}

export function translateWithQuarantinedGlossary(
  sourceText: string,
  targetLocale: TechnicalLocaleCode,
): QuarantinedTranslationResult {
  const preservedTokens = extractProtectedEngineeringTokens(sourceText);
  const placeholders: string[] = [];
  const masked = sourceText.replace(PROTECTED_TOKEN_RE, (tok) => {
    const idx = placeholders.length;
    placeholders.push(tok);
    return `@@SIGTOK_${idx}@@`;
  });

  const dict = LOCALE_PHRASE_PREFIX[targetLocale];
  let translated = masked
    .replace(/\bRoute\b/gi, dict.route)
    .replace(/\bPlace\b/gi, dict.place)
    .replace(/\bclearance\b/gi, dict.clearance)
    .replace(/\bnear\b/gi, dict.near);

  translated = `[${targetLocale}] ${translated}`.replace(/@@SIGTOK_(\d+)@@/g, (_m, rawIdx: string) => {
    return placeholders[Number(rawIdx)] ?? "";
  });

  const allTokensIntact = preservedTokens.every((tok) => translated.includes(tok));
  return {
    targetLocale,
    translatedText: translated,
    preservedTokens,
    allTokensIntact,
  };
}

export function buildInlineTranslateVm(
  sourceText: string,
  activeLocale: TechnicalLocaleCode,
): InlineTranslateVm {
  const result = translateWithQuarantinedGlossary(sourceText, activeLocale);
  return {
    locales: SUPPORTED_TECHNICAL_LOCALES,
    activeLocale,
    sourceText,
    result,
    quarantineBadge: result.allTokensIntact
      ? `ZERO_TOOL_QUARANTINE • ${SUPPORTED_TECHNICAL_LOCALES.length} Locales • 100% Tokens Intact`
      : "Token Drift Detected",
    quarantineTone: result.allTokensIntact ? "ok" : "danger",
    preservedSummary:
      result.preservedTokens.length > 0
        ? `Preserved Verbatim: ${result.preservedTokens.join(" • ")}`
        : "No RefDes/NetName/SI tokens in text",
  };
}
