// Module: web/libs/fpga-studio/src/fpga-studio.routes.ts — Lazy Angular v22+ Routes for FPGA RTL, Constraints & Pin Planner
// Invariants: Zero eager bundle cost; mounts FpgaStudioWorkbenchComponent (CU-P6-02).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { FpgaStudioWorkbenchComponent } from "./lib/fpga-studio-workbench/fpga-studio-workbench.component";
import { FPGA_STUDIO_WORKBENCH_DESCRIPTOR } from "./lib/fpga-studio-workbench/fpga-studio-workbench.logic";

export const WORKBENCH_DESCRIPTOR = FPGA_STUDIO_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: FpgaStudioWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
