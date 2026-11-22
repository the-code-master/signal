// Module: web/libs/cam-print/src/cam-print.routes.ts — Lazy Angular v22+ Routes for CNC 3/5-Axis CAM & Additive 3D Print Slicer
// Invariants: Zero eager bundle cost; mounts CamPrintWorkbenchComponent (CU-P5-04).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { CamPrintWorkbenchComponent } from "./lib/cam-print-workbench/cam-print-workbench.component";
import { CAM_PRINT_WORKBENCH_DESCRIPTOR } from "./lib/cam-print-workbench/cam-print-workbench.logic";

export const WORKBENCH_DESCRIPTOR = CAM_PRINT_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: CamPrintWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
