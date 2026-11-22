// Module: web/libs/review/src/review.routes.ts — Lazy Angular v22+ Routes for Design Gate & Access Review Console
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "review",
  title: "Design Gate & Access Review Console",
  subtitle: "Live BYO-Git REPO_ACCESS_ENTRY matrix & 2-of-3 Break-Glass",
  rustCrate: "signal-git-sync",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Posture Score: 98/100", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Stale Grants: 0", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Provider: GitHub Enterprise", "Padme Vault: ON"],
  gridTitle: "Active Repo Access Roster",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "U1", item: "local-guest-0001 (Org Admin)", metric: "All 5 Repos" },
    { id: "U2", item: "local-pcb-contractor-0002", metric: "PCB Repo ONLY" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
