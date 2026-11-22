// Module: web/libs/visual-diff/src/visual-diff.routes.ts — Lazy Angular v22+ Routes for Topological Visual Diff
// Invariants: Zero eager bundle cost; mounts VisualDiffWorkbenchComponent with FEAT-097 History Scrubber.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { VisualDiffWorkbenchComponent } from "./lib/visual-diff-workbench/visual-diff-workbench.component";
import { VISUAL_DIFF_WORKBENCH_DESCRIPTOR } from "./lib/visual-diff-workbench/visual-diff-workbench.logic";

export const WORKBENCH_DESCRIPTOR = VISUAL_DIFF_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: VisualDiffWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
