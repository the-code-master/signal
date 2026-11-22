// Module: web/libs/bringup-lab/src/lib/bringup-lab-workbench/bringup-lab-workbench.logic.ts — I-06 Digital Twin Calibration Logic
// Invariants: Zero Angular decorators; owns BRINGUP_LAB_WORKBENCH_DESCRIPTOR and Bayesian parasitic fit metrics.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  BringupLabWorkbenchVm,
  LabCaptureSource,
} from "./bringup-lab-workbench.model";

export const BRINGUP_LAB_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "bringup-lab",
  title: "Post-Silicon Lab Bench Digital Twin & Shmoo Studio",
  subtitle: "SCPI/Touchstone Bayesian calibration, Cold-Boot POST, HIL & PVT Shmoo (I-06)",
  rustCrate: "signal-twin",
  subprojectRepo: "aurora-ev-inverter-prd",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Twin Residual: 0.14%", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Calibrated: YES", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Bench: 4-Ch MSO + HV Diff", "Temp: 358_150 mK"],
  gridTitle: "Captured Channels",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "T1", item: "Lab Vds vs Sim Vds", metric: "Delta 0.14%" },
    { id: "T2", item: "Thermal Camera IR Map", metric: "Delta 0.6 K" },
  ],
};

export function buildBringupLabWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  captureSource: LabCaptureSource,
  bayesianCalibrated: boolean,
): BringupLabWorkbenchVm {
  const postCalibrationRmsErrorPct = bayesianCalibrated ? 0.14 : 6.85;
  const fittedLoopInductancePh = bayesianCalibrated ? 4_820n : 3_100n;
  const twinTone: SigStatusTone = postCalibrationRmsErrorPct <= 1.0 ? "ok" : "warn";
  const twinBadge = `I-06 Digital Twin (${captureSource}): Sim-vs-Bench RMS=${postCalibrationRmsErrorPct}% • Fitted L_stray=${fittedLoopInductancePh.toString(10)} pH`;

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    captureSource,
    captureSources: ["SCPI_MSO64B_SCOPE", "TOUCHSTONE_VNA_S4P", "FLIR_THERMAL_MAP"],
    bayesianCalibrated,
    postCalibrationRmsErrorPct,
    fittedLoopInductancePh,
    twinBadge,
    twinTone,
  };
}
