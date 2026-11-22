// Module: web/apps/signal-web/src/app/app.routes.ts — URL-Scoped /org/:org_slug/project/:proj_slug Lazy Route Tree
// Invariants: Enforces tenant/project URL scoping (F-097); zero /export or /download routes; lazy-loads all 25 workbenches.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/15-application-security-and-project-isolation.md
import type { Routes } from "@angular/router";

export const APP_ROUTES: Routes = [
  {
    path: "",
    pathMatch: "full",
    redirectTo: "org/signal-design-inc/project/aurora-ev-inverter/schematic",
  },
  {
    path: "org/:org_slug/project/:proj_slug",
    children: [
      { path: "", pathMatch: "full", redirectTo: "schematic" },
      { path: "schematic", loadChildren: () => import("@signal/schematic").then((m) => m.WORKBENCH_ROUTES) },
      { path: "sim-viewer", loadChildren: () => import("@signal/sim-viewer").then((m) => m.WORKBENCH_ROUTES) },
      { path: "pcb", loadChildren: () => import("@signal/pcb").then((m) => m.WORKBENCH_ROUTES) },
      { path: "magnetics", loadChildren: () => import("@signal/magnetics").then((m) => m.WORKBENCH_ROUTES) },
      { path: "power-studio", loadChildren: () => import("@signal/power-studio").then((m) => m.WORKBENCH_ROUTES) },
      { path: "emc-lab", loadChildren: () => import("@signal/emc-lab").then((m) => m.WORKBENCH_ROUTES) },
      { path: "safety-wcca", loadChildren: () => import("@signal/safety-wcca").then((m) => m.WORKBENCH_ROUTES) },
      { path: "ic", loadChildren: () => import("@signal/ic").then((m) => m.WORKBENCH_ROUTES) },
      { path: "fpga-studio", loadChildren: () => import("@signal/fpga-studio").then((m) => m.WORKBENCH_ROUTES) },
      { path: "code-ide", loadChildren: () => import("@signal/code-ide").then((m) => m.WORKBENCH_ROUTES) },
      { path: "bringup-lab", loadChildren: () => import("@signal/bringup-lab").then((m) => m.WORKBENCH_ROUTES) },
      { path: "harness-3d", loadChildren: () => import("@signal/harness-3d").then((m) => m.WORKBENCH_ROUTES) },
      { path: "dfm-panel", loadChildren: () => import("@signal/dfm-panel").then((m) => m.WORKBENCH_ROUTES) },
      { path: "library-studio", loadChildren: () => import("@signal/library-studio").then((m) => m.WORKBENCH_ROUTES) },
      { path: "specs", loadChildren: () => import("@signal/specs").then((m) => m.WORKBENCH_ROUTES) },
      { path: "ai-copilot", loadChildren: () => import("@signal/ai-copilot").then((m) => m.WORKBENCH_ROUTES) },
      { path: "visual-diff", loadChildren: () => import("@signal/visual-diff").then((m) => m.WORKBENCH_ROUTES) },
      { path: "review", loadChildren: () => import("@signal/review").then((m) => m.WORKBENCH_ROUTES) },
      { path: "sftp-release", loadChildren: () => import("@signal/sftp-release").then((m) => m.WORKBENCH_ROUTES) },
      { path: "academy", loadChildren: () => import("@signal/academy").then((m) => m.WORKBENCH_ROUTES) },
      { path: "admin", loadChildren: () => import("@signal/admin").then((m) => m.WORKBENCH_ROUTES) },
      { path: "mcad-studio", loadChildren: () => import("@signal/mcad-studio").then((m) => m.WORKBENCH_ROUTES) },
      { path: "cam-print", loadChildren: () => import("@signal/cam-print").then((m) => m.WORKBENCH_ROUTES) },
      { path: "plm-ops", loadChildren: () => import("@signal/plm-ops").then((m) => m.WORKBENCH_ROUTES) },
      { path: "collab-hub", loadChildren: () => import("@signal/collab-hub").then((m) => m.WORKBENCH_ROUTES) },
    ],
  },
];
