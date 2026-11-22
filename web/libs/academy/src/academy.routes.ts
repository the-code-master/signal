// Module: web/libs/academy/src/academy.routes.ts — Lazy Angular v22+ Routes for Signal Academy & Quick Engineering Calculators
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "academy",
  title: "Signal Academy & Quick Engineering Calculators",
  subtitle: "Interactive SOA smoke sandbox & IPC/RF/Thermal calculators",
  rustCrate: "signal-core",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "UI Profile: Adaptive", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "LTI 1.3: Ready", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["IPC-2152 Cu Calc", "Microstrip Z0 Calc"],
  gridTitle: "Interactive Modules",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "AC1", item: "SiC Switching Loss & Dead-Time Lab", metric: "Ready" },
    { id: "AC2", item: "Creepage vs Altitude IEC 60664-1", metric: "Ready" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
