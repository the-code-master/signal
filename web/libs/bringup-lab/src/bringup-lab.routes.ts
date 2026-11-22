// Module: web/libs/bringup-lab/src/bringup-lab.routes.ts — Lazy Angular v22+ Routes for Post-Silicon Lab Bench Digital Twin
// Invariants: Zero eager bundle cost; mounts BringupLabWorkbenchComponent (CU-P6-04).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { BringupLabWorkbenchComponent } from "./lib/bringup-lab-workbench/bringup-lab-workbench.component";
import { BRINGUP_LAB_WORKBENCH_DESCRIPTOR } from "./lib/bringup-lab-workbench/bringup-lab-workbench.logic";

export const WORKBENCH_DESCRIPTOR = BRINGUP_LAB_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: BringupLabWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
