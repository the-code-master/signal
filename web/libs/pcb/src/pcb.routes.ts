// Module: web/libs/pcb/src/pcb.routes.ts — Lazy Angular v22+ Routes for 224G & High-Voltage PCB Layout
// Invariants: Zero eager bundle cost; mounts PcbWorkbenchComponent (CU-P4-01).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { PcbWorkbenchComponent } from "./lib/pcb-workbench/pcb-workbench.component";
import { PCB_WORKBENCH_DESCRIPTOR } from "./lib/pcb-workbench/pcb-workbench.logic";

export const WORKBENCH_DESCRIPTOR = PCB_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: PcbWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
