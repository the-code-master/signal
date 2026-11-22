// Module: web/libs/bringup-lab/src/lib/bringup-lab-workbench/bringup-lab-workbench.model.ts — I-06 Digital Twin Host Models
// Invariants: Fitted parasitic inductances (_ph) and resistances (_mohm) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type LabCaptureSource = "SCPI_MSO64B_SCOPE" | "TOUCHSTONE_VNA_S4P" | "FLIR_THERMAL_MAP";

export interface BringupLabWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly captureSource: LabCaptureSource;
  readonly captureSources: readonly LabCaptureSource[];
  readonly bayesianCalibrated: boolean;
  readonly postCalibrationRmsErrorPct: number;
  readonly fittedLoopInductancePh: bigint;
  readonly twinBadge: string;
  readonly twinTone: SigStatusTone;
}
