// Module: web/libs/harness-3d/src/lib/harness-3d-workbench/harness-3d-workbench.logic.ts — Harness 3D Host Logic
// Invariants: Zero Angular decorators; owns HARNESS_3D_WORKBENCH_DESCRIPTOR and bend-radius telemetry.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { Harness3dWorkbenchVm } from "./harness-3d-workbench.model";

export const HARNESS_3D_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "harness-3d",
  title: "3D HV Wire Harness, Multi-Board & Rigid-Flex Studio",
  subtitle: "Multi-board mating, IPC-2223 flex folding & WHMA-A-620 1:1 nailboard",
  rustCrate: "signal-harness",
  subprojectRepo: "aurora-ev-inverter-cad",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Min Bend: 42_000_000 nm", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Shield Continuity: 100%", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Connector: TE HVP800", "Mass: 1.42 kg"],
  gridTitle: "Harness Bundles",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "HB1", item: "HV_DC_INPUT_70MM2", metric: "Orange Shielded" },
    { id: "HB2", item: "LV_CAN_FD_TWISTED", metric: "120 Ohm Term" },
  ],
};

export function buildHarness3dWorkbenchVm(
  descriptor: WorkbenchDescriptor,
): Harness3dWorkbenchVm {
  const minBundleBendNm = 42_000_000n;
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    minBundleBendNm,
    bendMarginBadge: `HV 70 mm² Min Spline Bend=${(minBundleBendNm / 1_000_000n).toString(10)} mm (>= 6x OD=38 mm)`,
  };
}
