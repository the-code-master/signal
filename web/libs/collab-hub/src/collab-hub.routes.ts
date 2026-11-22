// Module: web/libs/collab-hub/src/collab-hub.routes.ts — Lazy Angular v22+ Routes for 5-Team Zero-Leak Collab Space & Auto Reviews
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "collab-hub",
  title: "5-Team Zero-Leak Collab Space & Auto Reviews",
  subtitle: "Design/Dev/SW/Mfg/Ops channels, @entity threads & Weekly/Gate Reviews (21-*)",
  rustCrate: "signal-pod",
  subprojectRepo: "aurora-ev-inverter-ops",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Overall RAG: GREEN", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Stage Gate: DVT", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["5 Team Channels Encrypted", "Weekly + Executive Auto-Reviews"],
  gridTitle: "Cross-Team Threads & Reviews",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "REV-W47", item: "Automated Weekly Review (DVT Readiness 98.5%)", metric: "ML-DSA-65 Signed" },
    { id: "THR-19", item: "@U1_SIC_HS thermal pad voiding check w/ Mfg", metric: "DESIGN <-> MFG" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
