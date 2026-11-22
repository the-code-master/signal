// Module: web/libs/pcb/src/lib/pcb-workbench/pcb-workbench.logic.ts — CU-P4-01 224G & HV PCB Workbench Logic
// Invariants: Zero Angular decorators; owns canonical PCB_WORKBENCH_DESCRIPTOR and studio tab catalog.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { PcbStudioTabId, PcbStudioTabOption, PcbWorkbenchVm } from "./pcb-workbench.model";

export const PCB_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "pcb",
  title: "224G & High-Voltage PCB Layout",
  subtitle: "i128/Shewchuk DRC, PNS Bus-Trunk Router, 3D EM/TDR & X-Ray Isolation",
  rustCrate: "signal-pcb-router",
  subprojectRepo: "aurora-ev-inverter-pcb",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "DRC: 0 Violations", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Layers: 12-Layer Heavy Cu", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Creepage: 8_400_000 nm", "Z0: 85 Ohm Diff"],
  gridTitle: "Critical Routed Nets",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "N1", item: "GATE_DRV_HS_U", metric: "120_000 nm" },
    { id: "N2", item: "PCIE6_TELEM_TX0_P", metric: "85_000 nm" },
  ],
};

export const PCB_STUDIO_TABS: readonly PcbStudioTabOption[] = [
  { id: "PNS_MEANDERS", label: "01 • PNS Router, Meanders & Pkg Delay (FEAT-012/015/017)" },
  { id: "SI_EM_RF_TDR", label: "02 • Weave, Back-Drill, RF, 3D EM & TDR (FEAT-011..089)" },
  { id: "STACKUP_CONSTRAINTS_XRAY", label: "03 • Pour, HDI, Constraints & X-Ray (FEAT-014/018/095/096)" },
];

export function buildPcbWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  activeTab: PcbStudioTabId,
): PcbWorkbenchVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    activeTab,
    tabs: PCB_STUDIO_TABS,
  };
}
