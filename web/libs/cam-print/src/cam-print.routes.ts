// Module: web/libs/cam-print/src/cam-print.routes.ts — Lazy Angular v22+ Routes for CNC 3/5-Axis CAM & Additive 3D Print Slicer
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
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
    { id: "J1", item: "Coldplate 5-Axis Adaptive Roughing", metric: "6_000_000 nm Tool" },
    { id: "J2", item: "Busbar Insulator SLS Nylon-12 Print", metric: "100_000 nm Layer" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
