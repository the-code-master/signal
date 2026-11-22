// Module: web/libs/sim-viewer/src/lib/formula-bar/formula-bar.model.ts — FEAT-082 Waveform Math DSL Tokenizer & Autocomplete Models
// Invariants: Zero eval(); deterministic token stream and trapezoidal/RMS trace reduction.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";

export type FormulaTokenKind = "FUNC" | "SIGNAL_ACCESS" | "IDENT" | "NUMBER" | "OPERATOR" | "PAREN" | "COMMA";

export interface FormulaToken {
  readonly kind: FormulaTokenKind;
  readonly text: string;
}

export interface FormulaTokenizeResult {
  readonly valid: boolean;
  readonly tokens: readonly FormulaToken[];
  readonly errorHint: string;
}

export interface WaveformFormulaBarVm {
  readonly expression: string;
  readonly tokenized: FormulaTokenizeResult;
  readonly suggestions: readonly string[];
  readonly evaluatedSummary: string;
  readonly statusTone: SigStatusTone;
}
