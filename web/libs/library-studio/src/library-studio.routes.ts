// Module: web/libs/library-studio/src/library-studio.routes.ts — Lazy Angular v22+ Routes for 3-Tier Universal Parts & Padstack Studio
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "library-studio",
  title: "3-Tier Universal Parts & Padstack Studio",
  subtitle: "Master Pin Table, IEEE+IEC symbols, IPC-7351 & Org AVL (18-*)",
  rustCrate: "signal-parts",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Pinned: parts.lock.json", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "GOLD Parts: 100%", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["CAS: BLAKE3 Verified", "AVL: Preferred"],
  gridTitle: "Locked Project Parts",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "P1", item: "urn:signal:part:ti:TPS546D24ARVFR", metric: "GOLD" },
    { id: "P2", item: "urn:signal:part:wolfspeed:C3M0016120K", metric: "GOLD" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
