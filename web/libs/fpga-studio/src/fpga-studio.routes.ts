// Module: web/libs/fpga-studio/src/fpga-studio.routes.ts — Lazy Angular v22+ Routes for FPGA RTL, Constraints & Pin Planner
// Invariants: Zero eager bundle cost; feeds immutable WorkbenchDescriptor into centralized WorkbenchStageComponent.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { WorkbenchStageComponent, type WorkbenchDescriptor } from "@signal/canvas-core";

export const WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "fpga-studio",
  title: "FPGA RTL, Constraints & Pin Planner",
  subtitle: "Bank I/O standards, flight-time skew & CDC verification",
  rustCrate: "signal-graph",
  subprojectRepo: "aurora-ev-inverter-fpga",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "WNS: +412 ps", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "CDC: 0 Unsafe", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Device: Kintex UltraScale+", "PWM Res: 125 ps"],
  gridTitle: "I/O Banks",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "B64", item: "LVDS_1V8 Resolver Bus", metric: "Matched +-250 fs" },
    { id: "B65", item: "LVCMOS_3V3 Gate Faults", metric: "Verified" },
  ],
};

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: WorkbenchStageComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
