// Module: web/apps/signal-web/src/app/inline-translate/inline-translate.model.ts — FEAT-110 Quarantined Glossary Translator Models
// Invariants: Supports 15 technical locales while preserving RefDes, NetName, and SI unit literals verbatim.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, contracts/proto/signal/v1/ai_rag_formal.proto
import type { SigStatusTone } from "@signal/ui-kit";

export type TechnicalLocaleCode =
  | "ja"
  | "de"
  | "zh-TW"
  | "ko"
  | "fr"
  | "es"
  | "it"
  | "pt"
  | "nl"
  | "sv"
  | "pl"
  | "cs"
  | "hi"
  | "vi"
  | "ar";

export interface QuarantinedTranslationResult {
  readonly targetLocale: TechnicalLocaleCode;
  readonly translatedText: string;
  readonly preservedTokens: readonly string[];
  readonly allTokensIntact: boolean;
}

export interface InlineTranslateVm {
  readonly locales: readonly TechnicalLocaleCode[];
  readonly activeLocale: TechnicalLocaleCode;
  readonly sourceText: string;
  readonly result: QuarantinedTranslationResult;
  readonly quarantineBadge: string;
  readonly quarantineTone: SigStatusTone;
  readonly preservedSummary: string;
}
