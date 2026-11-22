// Module: web/libs/emc-lab/src/emc-lab.routes.ts — Lazy Angular v22+ Routes for Pre-Compliance EMC / EMI Lab
// Invariants: Zero eager bundle cost; mounts EmcLabWorkbenchComponent (CU-P4-03).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { EmcLabWorkbenchComponent } from "./lib/emc-lab-workbench/emc-lab-workbench.component";
import { EMC_LAB_WORKBENCH_DESCRIPTOR } from "./lib/emc-lab-workbench/emc-lab-workbench.logic";

export const WORKBENCH_DESCRIPTOR = EMC_LAB_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: EmcLabWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
