// Module: web/libs/power-studio/src/power-studio.routes.ts — Lazy Angular v22+ Routes for PDN & Power Integrity Studio
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "power-studio",
  title: "PDN & Power Integrity Studio",
  subtitle: "Target impedance Z_target(f), loop inductance & DC IR-drop",
  rustCrate: "signal-sim",
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

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
