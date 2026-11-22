// Module: web/libs/schematic/src/schematic.routes.ts — Lazy Angular v22+ Routes for Schematic Capture & Hierarchy
// Invariants: Zero eager bundle cost; mounts SchematicWorkbenchComponent with FEAT-001..FEAT-010 panels.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { SchematicWorkbenchComponent } from "./lib/schematic-workbench/schematic-workbench.component";
import { SCHEMATIC_WORKBENCH_DESCRIPTOR } from "./lib/schematic-workbench/schematic-workbench.logic";

export const WORKBENCH_DESCRIPTOR = SCHEMATIC_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: SchematicWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
