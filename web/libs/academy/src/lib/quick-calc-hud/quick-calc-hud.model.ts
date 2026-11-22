// Module: web/libs/academy/src/lib/quick-calc-hud/quick-calc-hud.model.ts — FEAT-114 Resistor, EIA-96, Divider & LED Calc Models
// Invariants: All resistance (_mohm), voltage (_uv), current (_na), and power (_uw) quantities stay in ES2020 bigint.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";

export type ResistorBandColor =
  | "black"
  | "brown"
  | "red"
  | "orange"
  | "yellow"
  | "green"
  | "blue"
  | "violet"
  | "grey"
  | "white"
  | "gold"
  | "silver";

export interface ResistorDecodeResult {
  readonly valid: boolean;
  readonly nominalMohm: bigint;
  readonly tolerancePct: number;
  readonly tempcoPpmPerK: number | null;
  readonly formattedSummary: string;
  readonly tone: SigStatusTone;
}

export interface DividerCalcResult {
  readonly vOutUv: bigint;
  readonly branchCurrentNa: bigint;
  readonly formattedSummary: string;
}

export interface LedLimiterCalcResult {
  readonly exactMohm: bigint;
  readonly nearestE24Mohm: bigint;
  readonly powerDissUw: bigint;
  readonly formattedSummary: string;
}

export interface SchematicInsertPayload {
  readonly calculatorKind: "COLOR_BAND" | "EIA96_SMD" | "VOLTAGE_DIVIDER" | "LED_RESISTOR";
  readonly primaryValueMohm: bigint;
  readonly description: string;
}

export interface QuickCalcHudVm {
  readonly bandResult: ResistorDecodeResult;
  readonly smdResult: ResistorDecodeResult;
  readonly dividerResult: DividerCalcResult;
  readonly ledResult: LedLimiterCalcResult;
}
