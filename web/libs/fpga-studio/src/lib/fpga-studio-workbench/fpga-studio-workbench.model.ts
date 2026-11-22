// Module: web/libs/fpga-studio/src/lib/fpga-studio-workbench/fpga-studio-workbench.model.ts — FPGA Host Models
// Invariants: Organizes FEAT-051/052 Die I/O + SDC and FEAT-057/060 FSM + Fixed-Point DSP studios cleanly.
// Spec: architecture/03-rust-eda-engine.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type FpgaPartPreset = "XCKU5P_ULTRASCALE_PLUS" | "AGFB014_AGILEX7" | "LFD2NX_CERTUS_NX";

export interface FpgaStudioWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly partPreset: FpgaPartPreset;
  readonly partPresets: readonly FpgaPartPreset[];
  readonly deviceSummaryBadge: string;
}
