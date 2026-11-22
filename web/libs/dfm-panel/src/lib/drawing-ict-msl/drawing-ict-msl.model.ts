// Module: web/libs/dfm-panel/src/lib/drawing-ict-msl/drawing-ict-msl.model.ts — FEAT-067/069/080 Drawing, ICT & MSL Models
// Invariants: GD&T tolerances (_nm) and peak reflow temperatures (_mkelvin) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type Jstd020MslLevel = "MSL_1_UNLIMITED" | "MSL_3_168H" | "MSL_5A_24H";

export interface DrawingIctMslVm {
  readonly mslLevel: Jstd020MslLevel;
  readonly mslLevels: readonly Jstd020MslLevel[];
  readonly jtagBoundaryScanEnabled: boolean;
  readonly combinedTestCoveragePct: number;
  readonly peakReflowMkelvin: bigint;
  readonly gdtPositionTolNm: bigint;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
