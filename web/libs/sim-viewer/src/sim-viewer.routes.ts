// Module: web/libs/sim-viewer/src/sim-viewer.routes.ts — Lazy Angular v22+ Routes for SPICE & Differentiable Adjoint Lab
// Invariants: Zero eager bundle cost; mounts SimViewerWorkbenchComponent with CU-P1-03 + CU-P1-04 studios.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { SimViewerWorkbenchComponent } from "./lib/sim-viewer-workbench/sim-viewer-workbench.component";
import { SIM_VIEWER_WORKBENCH_DESCRIPTOR } from "./lib/sim-viewer-workbench/sim-viewer-workbench.logic";

export const WORKBENCH_DESCRIPTOR = SIM_VIEWER_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: SimViewerWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
