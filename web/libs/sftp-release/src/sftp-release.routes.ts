// Module: web/libs/sftp-release/src/sftp-release.routes.ts — Lazy Angular v22+ Routes for Step-Up 2FA Write-Only SFTP Factory Egress
// Invariants: Zero eager bundle cost; mounts SftpReleaseWorkbenchComponent (CU-P4-05).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { SftpReleaseWorkbenchComponent } from "./lib/sftp-release-workbench/sftp-release-workbench.component";
import { SFTP_RELEASE_WORKBENCH_DESCRIPTOR } from "./lib/sftp-release-workbench/sftp-release-workbench.logic";

export const WORKBENCH_DESCRIPTOR = SFTP_RELEASE_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: SftpReleaseWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
