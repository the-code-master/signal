// Module: web/libs/power-studio/src/power-studio.routes.ts — Lazy Angular v22+ Routes for PDN & Power Integrity Studio
// Invariants: Zero eager bundle cost; mounts PowerStudioWorkbenchComponent (CU-P4-02 FEAT-022..030).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { PowerStudioWorkbenchComponent } from "./lib/power-studio-workbench/power-studio-workbench.component";
import { POWER_STUDIO_WORKBENCH_DESCRIPTOR } from "./lib/power-studio-workbench/power-studio-workbench.logic";

export const WORKBENCH_DESCRIPTOR = POWER_STUDIO_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: PowerStudioWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
