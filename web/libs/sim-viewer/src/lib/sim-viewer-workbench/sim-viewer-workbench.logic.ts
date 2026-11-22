// Module: web/libs/sim-viewer/src/lib/sim-viewer-workbench/sim-viewer-workbench.logic.ts — Pure Sim-Viewer Workbench Logic
// Invariants: Zero Angular decorators; owns canonical SIM_VIEWER_WORKBENCH_DESCRIPTOR and studio tab catalog.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { SimStudioTabId, SimStudioTabOption, SimViewerWorkbenchVm } from "./sim-viewer-workbench.model";

export const SIM_VIEWER_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "sim-viewer",
  title: "SPICE & Differentiable Adjoint Lab",
  subtitle: "Ruiz MNA KLU/cuDSS + Filippov Saltation Adjoint waveforms",
  rustCrate: "signal-diff-sim",
  subprojectRepo: "aurora-ev-inverter-schematic",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Epoch: #1 (0.5-ULP)", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Solver: KLU+Revolve", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["dI/dt: 42 A/ns", "Saltation: Active"],
  gridTitle: "Simulation Channels",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "CH1", item: "V(HV_SW_PHASE_U)", metric: "800_120_000 uV" },
    { id: "CH2", item: "I(L_PHASE_U)", metric: "425_000_000 uA" },
  ],
};

export const SIM_STUDIO_TABS: readonly SimStudioTabOption[] = [
  { id: "VIRTUAL_BENCH", label: "01 • Virtual Bench (FEAT-081)" },
  { id: "MONTE_CARLO", label: "02 • Monte Carlo Yield (FEAT-083)" },
  { id: "JITTER_STUDIO", label: "03 • SerDes Jitter (FEAT-084)" },
  { id: "WHAT_IF_ADJOINT", label: "04 • What-If Adjoint (FEAT-085)" },
  { id: "REGRESSION_SUITE", label: "05 • Spec Regression (FEAT-086)" },
  { id: "STIMULUS_AUDITION", label: "06 • WebAudio A/B (FEAT-090)" },
  { id: "BREAKING_POINT", label: "07 • Breaking Point (FEAT-102)" },
  { id: "PARETO_FRONTIER", label: "08 • Pareto NSGA-III (FEAT-103)" },
  { id: "ANOMALY_DEBUGGER", label: "09 • Anomaly Root-Cause (FEAT-106)" },
];

export function buildSimViewerWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  activeTab: SimStudioTabId,
  probedNet: string,
  activeFormula: string,
): SimViewerWorkbenchVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    activeTab,
    tabs: SIM_STUDIO_TABS,
    crossProbeBadge: `Cross-Probe: ${probedNet} • Expr: ${activeFormula}`,
  };
}
