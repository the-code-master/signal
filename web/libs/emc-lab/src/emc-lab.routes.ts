// Module: web/libs/emc-lab/src/emc-lab.routes.ts — Lazy Angular v22+ Routes for Pre-Compliance EMC / EMI Lab
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "emc-lab",
  title: "Pre-Compliance EMC / EMI Lab",
  subtitle: "CISPR-25 radiated/conducted emissions & return-path splits",
  rustCrate: "signal-sim",
  subprojectRepo: "aurora-ev-inverter-pcb",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "CISPR-25 Class 5: PASS", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Margin: +6.4 dBuV", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Peak: 31.6 dBuV @ 68 MHz", "CM Choke: Active"],
  gridTitle: "Scan Peaks",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "P1", item: "68.0 MHz Switching Harmonic", metric: "31.6 dBuV" },
    { id: "P2", item: "204.0 MHz Gate Ringing", metric: "27.1 dBuV" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
