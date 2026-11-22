// Module: web/libs/magnetics/src/magnetics.routes.ts — Lazy Angular v22+ Routes for Planar & Discrete Magnetics Studio
// Invariants: Zero eager bundle cost; mounts MagneticsWorkbenchComponent (CU-P4-02 FEAT-021).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { MagneticsWorkbenchComponent } from "./lib/magnetics-workbench/magnetics-workbench.component";
import { MAGNETICS_WORKBENCH_DESCRIPTOR } from "./lib/magnetics-workbench/magnetics-workbench.logic";

export const WORKBENCH_DESCRIPTOR = MAGNETICS_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: MagneticsWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
