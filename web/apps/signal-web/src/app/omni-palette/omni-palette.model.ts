// Module: web/apps/signal-web/src/app/omni-palette/omni-palette.model.ts — FEAT-091 Cmd+K Deterministic CLI & Fuzzy Jump Models
// Invariants: Coordinates parsed into ES2020 bigint nanometers (_nm); zero eval().
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";

export type OmniCommandVerb = "PLACE" | "CONNECT" | "ROUTE" | "SIM" | "GOTO";

export interface ParsedOmniCommand {
  readonly valid: boolean;
  readonly verb: OmniCommandVerb | null;
  readonly normalizedAction: string;
  readonly targetWorkbenchSlug: string | null;
  readonly xNm?: bigint;
  readonly yNm?: bigint;
}

export interface OmniJumpCandidate {
  readonly slug: string;
  readonly label: string;
  readonly score: number;
}

export interface OmniPaletteVm {
  readonly open: boolean;
  readonly rawInput: string;
  readonly parsed: ParsedOmniCommand;
  readonly statusTone: SigStatusTone;
  readonly jumpCandidates: readonly OmniJumpCandidate[];
  readonly quickExamples: readonly string[];
}
