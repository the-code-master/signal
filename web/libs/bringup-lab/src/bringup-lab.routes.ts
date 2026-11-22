// Module: web/libs/bringup-lab/src/bringup-lab.routes.ts — Lazy Angular v22+ Routes for Post-Silicon Lab Bench Digital Twin
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "bringup-lab",
  title: "Post-Silicon Lab Bench Digital Twin",
  subtitle: "Live SCPI/oscilloscope overlay against SPICE waveforms (I-06)",
  rustCrate: "signal-sim",
  subprojectRepo: "aurora-ev-inverter-prd",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Twin Residual: 0.14%", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Calibrated: YES", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Bench: 4-Ch MSO + HV Diff", "Temp: 358_150 mK"],
  gridTitle: "Captured Channels",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "T1", item: "Lab Vds vs Sim Vds", metric: "Delta 0.14%" },
    { id: "T2", item: "Thermal Camera IR Map", metric: "Delta 0.6 K" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
