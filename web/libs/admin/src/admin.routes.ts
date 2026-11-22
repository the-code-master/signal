// Module: web/libs/admin/src/admin.routes.ts — Lazy Angular v22+ Routes for Enterprise BYO-Git, KMS PQC & Fleet Admin
// Invariants: Zero eager bundle cost; mounts AdminWorkbenchComponent with vcs-auditor & FEAT-075 AVL Where-Used.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { AdminWorkbenchComponent } from "./lib/admin-workbench/admin-workbench.component";
import { ADMIN_WORKBENCH_DESCRIPTOR } from "./lib/admin-workbench/admin-workbench.logic";

export const WORKBENCH_DESCRIPTOR = ADMIN_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: AdminWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
