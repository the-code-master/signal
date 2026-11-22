// Module: web/libs/safety-wcca/src/safety-wcca.routes.ts — Lazy Angular v22+ Routes for ISO 26262 FMEDA & WCCA Studio
// Invariants: Zero eager bundle cost; mounts SafetyWccaWorkbenchComponent (CU-P4-03).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { SafetyWccaWorkbenchComponent } from "./lib/safety-wcca-workbench/safety-wcca-workbench.component";
import { SAFETY_WCCA_WORKBENCH_DESCRIPTOR } from "./lib/safety-wcca-workbench/safety-wcca-workbench.logic";

export const WORKBENCH_DESCRIPTOR = SAFETY_WCCA_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: SafetyWccaWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
