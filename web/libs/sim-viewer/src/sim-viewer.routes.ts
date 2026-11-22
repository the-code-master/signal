// Module: web/libs/sim-viewer/src/sim-viewer.routes.ts — Lazy Angular v22+ Routes for SPICE & Differentiable Adjoint Lab
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
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

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
