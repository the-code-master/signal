// Module: web/libs/cam-print/src/lib/cam-print-workbench/cam-print-workbench.logic.ts — CAM & 3D Print Host Logic
// Invariants: Zero Angular decorators; owns CAM_PRINT_WORKBENCH_DESCRIPTOR and tab catalog.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  CamPrintTabId,
  CamPrintTabOption,
  CamPrintWorkbenchVm,
} from "./cam-print-workbench.model";

export const CAM_PRINT_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "cam-print",
  title: "CNC 3/5-Axis CAM & Additive 3D Print Slicer",
  subtitle: "Gouge-free adaptive clearing & direct-B-Rep 3MF/DMLS slicing (19-*)",
  rustCrate: "signal-cam",
  subprojectRepo: "aurora-ev-inverter-cam",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Holder Gouges: 0", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Cycle Time: 18m 40s", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["5-Axis Trochoidal", "DMLS Layer: 30_000 nm"],
  gridTitle: "Manufacturing Jobs",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "J1", item: "Coldplate 5-Axis Adaptive Roughing", metric: "10_000_000 nm Tool" },
    { id: "J2", item: "Busbar Insulator SLS Nylon-12 Print", metric: "100_000 nm Layer" },
  ],
};

export const CAM_PRINT_TABS: readonly CamPrintTabOption[] = [
  { id: "SUBTRACTIVE_CNC_CAM", label: "01 • 3/5-Axis Trochoidal CNC & Dexel Gouge (CAM_CNC)" },
  { id: "ADDITIVE_DIRECT_BREP_SLICER", label: "02 • Direct-B-Rep 3D Print Slicer (PRINT_3D)" },
];

export function buildCamPrintWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  activeTab: CamPrintTabId,
): CamPrintWorkbenchVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    activeTab,
    tabs: CAM_PRINT_TABS,
  };
}
