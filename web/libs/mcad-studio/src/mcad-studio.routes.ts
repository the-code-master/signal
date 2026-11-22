// Module: web/libs/mcad-studio/src/mcad-studio.routes.ts — Lazy Angular v22+ Routes for Mechanical Parametric B-Rep CAD & Sheet Metal
// Invariants: Zero eager bundle cost; mounts McadStudioWorkbenchComponent (CU-P5-03).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { McadStudioWorkbenchComponent } from "./lib/mcad-studio-workbench/mcad-studio-workbench.component";
import { MCAD_STUDIO_WORKBENCH_DESCRIPTOR } from "./lib/mcad-studio-workbench/mcad-studio-workbench.logic";

export const WORKBENCH_DESCRIPTOR = MCAD_STUDIO_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: McadStudioWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
