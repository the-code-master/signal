// Module: web/libs/dfm-panel/src/lib/panel-dfa-fatigue-coating/panel-dfa-fatigue-coating.model.ts — FEAT-063..066 Models
// Invariants: Panel offsets (_nm), coating keepout clearances (_nm), and modal frequencies (_hz) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type DepanelingMethod = "ROUTER_MOUSE_BITE_TABS" | "V_SCORE_GUILLOTINE";

export interface PanelDfaFatigueCoatingVm {
  readonly depanelMethod: DepanelingMethod;
  readonly depanelMethods: readonly DepanelingMethod[];
  readonly arrayCols: number;
  readonly arrayRows: number;
  readonly mlccPeakMicrostrain: number;
  readonly stencilAreaRatio: number;
  readonly coffinMansonCyclesN50: number;
  readonly steinbergFirstModeHz: bigint;
  readonly coatingKeepoutMarginNm: bigint;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
