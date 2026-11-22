// Module: web/libs/review/src/review.routes.ts — Lazy Angular v22+ Routes for Design Gate & Spatial Review Console
// Invariants: Zero eager bundle cost; mounts ReviewWorkbenchComponent with FEAT-094, 098 & 119.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { ReviewWorkbenchComponent } from "./lib/review-workbench/review-workbench.component";
import { REVIEW_WORKBENCH_DESCRIPTOR } from "./lib/review-workbench/review-workbench.logic";

export const WORKBENCH_DESCRIPTOR = REVIEW_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: ReviewWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
