// Module: web/libs/mcad-studio/src/mcad-studio.routes.ts — Lazy Angular v22+ Routes for Mechanical Parametric B-Rep CAD & Sheet Metal
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "mcad-studio",
  title: "Mechanical Parametric B-Rep CAD & Sheet Metal",
  subtitle: "Exact nm B-Rep feature tree, enclosure collision & ECAD anchors (19-*)",
  rustCrate: "signal-mcad",
  subprojectRepo: "aurora-ev-inverter-cad",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Interference: 0 Collisions", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Min Clearance: 2.5 mm", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Housing: Die-Cast AlSi10Mg", "Anchors: 12 M4 Bosses"],
  gridTitle: "Parametric Feature Tree",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "F1", item: "extrude_coldplate_base", metric: "14_000_000 nm" },
    { id: "F2", item: "fillet_seal_groove", metric: "1_500_000 nm" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
