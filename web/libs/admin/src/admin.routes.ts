// Module: web/libs/admin/src/admin.routes.ts — Lazy Angular v22+ Routes for Enterprise BYO-Git, KMS PQC & Fleet Admin
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "admin",
  title: "Enterprise BYO-Git, KMS PQC & Fleet Admin",
  subtitle: "Hybrid ML-KEM-768 + X25519 keyring & 1-4->N K8s cell pool telemetry",
  rustCrate: "signal-operator",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "K8s Pools: 3-Tier Healthy", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "PQC DEK: Active", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["pool-web | pool-api | pool-rust-engine", "Lateral Drop: 100%"],
  gridTitle: "Cluster Node Pools",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "NP1", item: "pool-web (Angular v22 + Node 26)", metric: "2 Replicas" },
    { id: "NP2", item: "pool-rust-engine (Kata MicroVMs)", metric: "4->32 Extensible" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
