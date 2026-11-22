// Module: web/libs/dfm-panel/src/dfm-panel.routes.ts — Lazy Angular v22+ Routes for Signed Fab Capability & DFM Panelizer
// Invariants: Zero eager bundle cost; mounts DfmPanelWorkbenchComponent (CU-P4-04).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { DfmPanelWorkbenchComponent } from "./lib/dfm-panel-workbench/dfm-panel-workbench.component";
import { DFM_PANEL_WORKBENCH_DESCRIPTOR } from "./lib/dfm-panel-workbench/dfm-panel-workbench.logic";

export const WORKBENCH_DESCRIPTOR = DFM_PANEL_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: DfmPanelWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
