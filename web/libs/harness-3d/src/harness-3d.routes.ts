// Module: web/libs/harness-3d/src/harness-3d.routes.ts — Lazy Angular v22+ Routes for 3D HV Wire Harness & Nailboard
// Invariants: Zero eager bundle cost; mounts Harness3dWorkbenchComponent (CU-P5-03).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { Harness3dWorkbenchComponent } from "./lib/harness-3d-workbench/harness-3d-workbench.component";
import { HARNESS_3D_WORKBENCH_DESCRIPTOR } from "./lib/harness-3d-workbench/harness-3d-workbench.logic";

export const WORKBENCH_DESCRIPTOR = HARNESS_3D_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: Harness3dWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
