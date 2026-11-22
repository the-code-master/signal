// Module: web/libs/emc-lab/src/lib/emc-lab-workbench/emc-lab-workbench.model.ts — FEAT-031/039 EMC Lab Host Models
// Invariants: Scan frequencies (_hz) and ripple amplitudes (_uv) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type EmcStandardMask = "CISPR_25_CLASS_5" | "CISPR_32_CLASS_B" | "FCC_PART_15B";

export interface EmcHarmonicPeak {
  readonly peakId: string;
  readonly freqHz: bigint;
  readonly measuredDbuv: number;
  readonly sourceNet: string;
}

export interface EmcLabWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly standard: EmcStandardMask;
  readonly standards: readonly EmcStandardMask[];
  readonly pwmBurstFreqHz: bigint;
  readonly piezoAudibleRisk: boolean;
  readonly worstMarginDb: number;
  readonly spectrumBadge: string;
  readonly spectrumTone: SigStatusTone;
  readonly piezoBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
