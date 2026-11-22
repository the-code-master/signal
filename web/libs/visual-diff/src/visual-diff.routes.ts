// Module: web/libs/visual-diff/src/visual-diff.routes.ts — Lazy Angular v22+ Routes for Topological Schematic, PCB & 3D B-Rep Diff
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "visual-diff",
  title: "Topological Schematic, PCB & 3D B-Rep Diff",
  subtitle: "Semantic entity diff across CoW branches & Super-Commits",
  rustCrate: "signal-graph",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Delta Ops: 4", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Rail Shorts: 0", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Base: main@100", "Head: main@101"],
  gridTitle: "Changed Entities",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "D1", item: "trace_pcie6_tx0_p", metric: "+85_000 nm route" },
    { id: "D2", item: "heatsink_fin_pitch", metric: "4_500_000 nm" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
