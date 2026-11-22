// Module: web/libs/ic/src/ic.routes.ts — Lazy Angular v22+ Routes for Analog/Mixed-Signal IC & Cryo Layout
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "ic",
  title: "Analog/Mixed-Signal IC & Cryo Layout",
  subtitle: "GDSII/OASIS, IEEE 1801 UPF, Pelgrom mismatch & LVF STA",
  rustCrate: "signal-geom",
  subprojectRepo: "aurora-ev-inverter-ic-analog",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "LVS: CLEAN", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "PDK: 22nm BCD HV", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Boundary Halo: 20_000 nm", "UPF Domains: 4"],
  gridTitle: "Macro Cells",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "M1", item: "hv_isolated_sigma_delta_mod", metric: "CLEAN" },
    { id: "M2", item: "bandgap_ref_0p5ulp", metric: "CLEAN" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
