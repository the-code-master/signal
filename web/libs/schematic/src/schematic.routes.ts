// Module: web/libs/schematic/src/schematic.routes.ts — Lazy Angular v22+ Routes for Schematic Capture & Hierarchy
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "schematic",
  title: "Schematic Capture & Hierarchy",
  subtitle: "Multi-sheet hierarchical schematic, bus bundles & ERC",
  rustCrate: "signal-graph",
  subprojectRepo: "aurora-ev-inverter-schematic",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "ERC: 0 Errors", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Sheets: 14", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Net: HV_DC_BUS_800V", "Gate: U4_SIC_HS"],
  gridTitle: "Active Schematic Entities",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "U1", item: "C3M0016120K SiC MOSFET", metric: "800_000_000 uV" },
    { id: "U2", item: "UCC21750QDWR Isolated Gate Driver", metric: "5_700_000_000 uV" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
