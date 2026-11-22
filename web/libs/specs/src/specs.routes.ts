// Module: web/libs/specs/src/specs.routes.ts — Lazy Angular v22+ Routes for Executable PRD & System Requirements
// Invariants: Zero eager bundle cost; mounts SpecsBomWorkbenchComponent with FEAT-077 BOM Provenance Matrix.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { SpecsBomWorkbenchComponent } from "./lib/specs-bom-workbench/specs-bom-workbench.component";
import { SPECS_WORKBENCH_DESCRIPTOR } from "./lib/specs-bom-workbench/specs-bom-workbench.logic";

export const WORKBENCH_DESCRIPTOR = SPECS_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: SpecsBomWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
