// Module: web/libs/safety-wcca/src/safety-wcca.routes.ts — Lazy Angular v22+ Routes for ISO 26262 FMEDA & WCCA Studio
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "safety-wcca",
  title: "ISO 26262 FMEDA & WCCA Studio",
  subtitle: "Single-point fault metrics (SPFM >= 99%) & Monte-Carlo WCCA",
  rustCrate: "signal-sim",
  subprojectRepo: "aurora-ev-inverter-prd",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "ASIL-D SPFM: 99.4%", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "FIT: 4.8 FIT", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["LFM: 92.1%", "WCCA: 6-Sigma PASS"],
  gridTitle: "Safety Mechanisms",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "SM1", item: "Dual Desat + OC Crowbar", metric: "99.6%" },
    { id: "SM2", item: "Redundant Resolver ADC", metric: "99.1%" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
