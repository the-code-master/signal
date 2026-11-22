// Module: web/libs/code-ide/src/code-ide.routes.ts — Lazy Angular v22+ Routes for Embedded Firmware & BIOS Co-Sim IDE
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "code-ide",
  title: "Embedded Firmware & BIOS Co-Sim IDE",
  subtitle: "MISRA-C / Rust no_std motor-control loop co-simulated with SPICE",
  rustCrate: "signal-pod",
  subprojectRepo: "aurora-ev-inverter-firmware",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "FOC Loop: 50 kHz", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "WCET: 6.2 us", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Target: Cortex-R52 Lockstep", "Stack Margin: 44%"],
  gridTitle: "Firmware Modules",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "FW1", item: "foc_svm_deadtime_comp.rs", metric: "WCET 2.1 us" },
    { id: "FW2", item: "asil_d_watchdog_mgr.rs", metric: "WCET 0.8 us" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
