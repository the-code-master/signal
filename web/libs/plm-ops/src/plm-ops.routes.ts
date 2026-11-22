// Module: web/libs/plm-ops/src/plm-ops.routes.ts — Lazy Angular v22+ Routes for Native PLM: eBOM/mBOM, ECN/MCN & Clear-to-Build
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "plm-ops",
  title: "Native PLM: eBOM/mBOM, ECN/MCN & Clear-to-Build",
  subtitle: "Unified change control, qty-capped Deviations & CTB readiness (19-*)",
  rustCrate: "signal-plm",
  subprojectRepo: "aurora-ev-inverter-ops",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "CTB Readiness: 98.5%", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Open ECNs: 1", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Unit Cost: $142.80", "Deviations: 1 Active (450/500)"],
  gridTitle: "BOM & Change Orders",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "ECN-104", item: "Upgrade DC-link film cap ripple rating", metric: "CCB_APPROVED" },
    { id: "DEV-004", item: "Alternate NTC thermistor lot (50 units left)", metric: "QTY_BOUNDED" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
