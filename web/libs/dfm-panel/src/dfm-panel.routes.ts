// Module: web/libs/dfm-panel/src/dfm-panel.routes.ts — Lazy Angular v22+ Routes for Signed Fab Capability & DFM Panelizer
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "dfm-panel",
  title: "Signed Fab Capability & DFM Panelizer",
  subtitle: "Checks layout against ML-DSA-65 signed *.fabprofile.toml (F-135)",
  rustCrate: "signal-plm",
  subprojectRepo: "aurora-ev-inverter-pcb",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Fab Profile: VERIFIED", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Yield Est: 99.2%", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Min Trace: 75_000 nm", "Min Drill: 150_000 nm"],
  gridTitle: "DFM Checks",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "DFM1", item: "IPC-7525 Stencil Area Ratio", metric: "0.74 (>= 0.66)" },
    { id: "DFM2", item: "Heavy-Cu Acid Trap Audit", metric: "0 Traps" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
