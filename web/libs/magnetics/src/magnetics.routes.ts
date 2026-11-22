// Module: web/libs/magnetics/src/magnetics.routes.ts — Lazy Angular v22+ Routes for Planar & Discrete Magnetics Studio
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "magnetics",
  title: "Planar & Discrete Magnetics Studio",
  subtitle: "Core loss (iGSE),litz proximity & air-gap fringing",
  rustCrate: "signal-sim",
  subprojectRepo: "aurora-ev-inverter-pcb",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Core: Ferroxcube 3C95", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Temp Rise: +18.4 K", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["B_peak: 240 mT", "L_mag: 140 uH"],
  gridTitle: "Winding Stack",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "W1", item: "Primary 12T Planar Cu", metric: "2_400 uOhm" },
    { id: "W2", item: "Aux Sense 3T", metric: "18_000 uOhm" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
