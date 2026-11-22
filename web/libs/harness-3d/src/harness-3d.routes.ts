// Module: web/libs/harness-3d/src/harness-3d.routes.ts — Lazy Angular v22+ Routes for 3D HV Wire Harness & Nailboard
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "harness-3d",
  title: "3D HV Wire Harness & Nailboard",
  subtitle: "3D bundle bend-radius check, voltage drop & formboard flatten",
  rustCrate: "signal-mcad",
  subprojectRepo: "aurora-ev-inverter-cad",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Min Bend: 42_000_000 nm", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Shield Continuity: 100%", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Connector: TE HVP800", "Mass: 1.42 kg"],
  gridTitle: "Harness Bundles",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "HB1", item: "HV_DC_INPUT_70MM2", metric: "Orange Shielded" },
    { id: "HB2", item: "LV_CAN_FD_TWISTED", metric: "120 Ohm Term" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
