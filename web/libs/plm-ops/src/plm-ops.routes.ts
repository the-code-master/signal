// Module: web/libs/plm-ops/src/plm-ops.routes.ts — Lazy Angular v22+ Routes for Native PLM: eBOM/mBOM, ECN/MCN & Clear-to-Build
// Invariants: Zero eager bundle cost; mounts PlmOpsWorkbenchComponent (CU-P5-05).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { PlmOpsWorkbenchComponent } from "./lib/plm-ops-workbench/plm-ops-workbench.component";
import { PLM_OPS_WORKBENCH_DESCRIPTOR } from "./lib/plm-ops-workbench/plm-ops-workbench.logic";

export const WORKBENCH_DESCRIPTOR = PLM_OPS_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: PlmOpsWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
