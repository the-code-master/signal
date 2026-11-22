// Module: web/libs/pcb/src/pcb.routes.ts — Lazy Angular v22+ Routes for 224G & High-Voltage PCB Layout
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "pcb",
  title: "224G & High-Voltage PCB Layout",
  subtitle: "i128/Shewchuk DRC, creepage geodesics & Euler bends",
  rustCrate: "signal-geom",
  subprojectRepo: "aurora-ev-inverter-pcb",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "DRC: 0 Violations", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Layers: 12-Layer Heavy Cu", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Creepage: 8_400_000 nm", "Z0: 85 Ohm Diff"],
  gridTitle: "Critical Routed Nets",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "N1", item: "GATE_DRV_HS_U", metric: "120_000 nm" },
    { id: "N2", item: "PCIE6_TELEM_TX0_P", metric: "85_000 nm" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
