// Module: web/libs/academy/src/academy.routes.ts — Lazy Angular v22+ Routes for Signal Academy & Quick Engineering Calculators
// Invariants: Zero eager bundle cost; mounts AcademyWorkbenchComponent with FEAT-113 & FEAT-114.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { AcademyWorkbenchComponent } from "./lib/academy-workbench/academy-workbench.component";
import { ACADEMY_WORKBENCH_DESCRIPTOR } from "./lib/academy-workbench/academy-workbench.logic";

export const WORKBENCH_DESCRIPTOR = ACADEMY_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: AcademyWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
