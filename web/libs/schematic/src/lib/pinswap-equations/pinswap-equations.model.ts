// Module: web/libs/schematic/src/lib/pinswap-equations/pinswap-equations.model.ts — FEAT-008..010 Pin Swap, Reannotate & Equations Models
// Invariants: Wirelength savings and component anchors use bigint nanometers (_nm); evaluated equations emit exact bigint SI integers.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type SpatialSweepDirection =
  | "LEFT_TO_RIGHT_TOP_DOWN"
  | "TOP_DOWN_LEFT_TO_RIGHT"
  | "HIERARCHICAL_SHEET_BLOCK";

export interface PinSwapCandidate {
  readonly id: string;
  readonly entityId: string;
  readonly fromPin: number;
  readonly toPin: number;
  readonly netName: string;
  readonly wirelengthSavedNm: bigint;
}

export interface SpatialComponentAnchor {
  readonly entityId: string;
  readonly prefix: string;
  readonly oldRefdes: string;
  readonly sheetIndex: number;
  readonly xNm: bigint;
  readonly yNm: bigint;
}

export interface RefdesRemapResult {
  readonly entityId: string;
  readonly oldRefdes: string;
  readonly newRefdes: string;
}

export interface EquationEvalResult {
  readonly valid: boolean;
  readonly siValue: bigint;
  readonly summaryText: string;
  readonly tone: SigStatusTone;
}

export interface PinswapEquationsVm {
  readonly swapSavingsBadge: string;
  readonly swapColumns: readonly SigGridColumn[];
  readonly swapRows: readonly SigGridRow[];
  readonly reannotateColumns: readonly SigGridColumn[];
  readonly reannotateRows: readonly SigGridRow[];
  readonly equationResult: EquationEvalResult;
}
