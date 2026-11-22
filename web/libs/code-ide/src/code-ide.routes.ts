// Module: web/libs/code-ide/src/code-ide.routes.ts — Lazy Angular v22+ Routes for Embedded Firmware & BIOS Co-Sim IDE
// Invariants: Zero eager bundle cost; mounts CodeIdeWorkbenchComponent (CU-P6-03).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { CodeIdeWorkbenchComponent } from "./lib/code-ide-workbench/code-ide-workbench.component";
import { CODE_IDE_WORKBENCH_DESCRIPTOR } from "./lib/code-ide-workbench/code-ide-workbench.logic";

export const WORKBENCH_DESCRIPTOR = CODE_IDE_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: CodeIdeWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
