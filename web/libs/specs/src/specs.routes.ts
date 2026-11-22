// Module: web/libs/specs/src/specs.routes.ts — Lazy Angular v22+ Routes for Executable PRD & System Requirements
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "specs",
  title: "Executable PRD & System Requirements",
  subtitle: "Live requirement-to-simulation assertion links",
  rustCrate: "signal-graph",
  subprojectRepo: "aurora-ev-inverter-prd",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Linked Assertions: 48/48", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Passing: 100%", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Peak Eff >= 99.1%", "THD <= 1.2%"],
  gridTitle: "Executable Requirements",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "REQ-01", item: "Peak Inverter Efficiency >= 99.0%", metric: "99.18% PASS" },
    { id: "REQ-02", item: "Short-Circuit Turn-Off <= 1.5 us", metric: "1.12 us PASS" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
