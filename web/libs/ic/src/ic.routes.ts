// Module: web/libs/ic/src/ic.routes.ts — Lazy Angular v22+ Routes for Analog/Mixed-Signal IC & Cryo Layout
// Invariants: Zero eager bundle cost; mounts IcWorkbenchComponent (CU-P6-01).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { IcWorkbenchComponent } from "./lib/ic-workbench/ic-workbench.component";
import { IC_WORKBENCH_DESCRIPTOR } from "./lib/ic-workbench/ic-workbench.logic";

export const WORKBENCH_DESCRIPTOR = IC_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: IcWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
