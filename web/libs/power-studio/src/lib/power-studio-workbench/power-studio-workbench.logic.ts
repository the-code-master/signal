// Module: web/libs/power-studio/src/lib/power-studio-workbench/power-studio-workbench.logic.ts — Power Studio Host Logic
// Invariants: Zero Angular decorators; owns canonical POWER_STUDIO_WORKBENCH_DESCRIPTOR and tab catalog.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  PowerStudioTabId,
  PowerStudioTabOption,
  PowerStudioWorkbenchVm,
} from "./power-studio-workbench.model";

export const POWER_STUDIO_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "power-studio",
  title: "PDN & Power Integrity Studio",
  subtitle: "Target impedance Z_target(f), 3D creepage, SiC hot-loop, BMS, FOC & Thermal/Sequencing",
  rustCrate: "signal-power-integrity",
  subprojectRepo: "aurora-ev-inverter-pcb",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Max IR Drop: 8.2 mV", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Z_pdn < 12 mOhm", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["VDD_CORE: 850_000 uV", "Decap: 28x MLCC"],
  gridTitle: "Power Rails",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "R1", item: "VDD_DSP_0V85", metric: "850_000 uV" },
    { id: "R2", item: "VCC_GATE_18V", metric: "18_000_000 uV" },
  ],
};

export const POWER_STUDIO_TABS: readonly PowerStudioTabOption[] = [
  { id: "PDN_CREEPAGE_HOTLOOP", label: "01 • PDN, 3D Creepage, IR & Hot-Loop (FEAT-022..025)" },
  { id: "BODE_BMS_MOTOR", label: "02 • Bode Loop, BMS Pack & FOC Motor (FEAT-026..028)" },
  { id: "THERMAL_SEQUENCING", label: "03 • Thermal Cold-Plate & Gantt Seq (FEAT-029..030)" },
];

export function buildPowerStudioWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  activeTab: PowerStudioTabId,
): PowerStudioWorkbenchVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    activeTab,
    tabs: POWER_STUDIO_TABS,
  };
}
