// Module: web/libs/collab-hub/src/collab-hub.routes.ts — Lazy Angular v22+ Routes for 5-Team Zero-Leak Collab Space & Auto Reviews
// Invariants: Zero eager bundle cost; mounts CollabHubWorkbenchComponent (CU-P7-04).
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { CollabHubWorkbenchComponent } from "./lib/collab-hub-workbench/collab-hub-workbench.component";
import { COLLAB_HUB_WORKBENCH_DESCRIPTOR } from "./lib/collab-hub-workbench/collab-hub-workbench.logic";

export const WORKBENCH_DESCRIPTOR = COLLAB_HUB_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: CollabHubWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
