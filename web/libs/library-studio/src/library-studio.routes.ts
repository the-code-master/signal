// Module: web/libs/library-studio/src/library-studio.routes.ts — Lazy Angular v22+ Routes for 3-Tier Universal Parts & Padstack Studio
// Invariants: Zero eager bundle cost; mounts LibraryStudioWorkbenchComponent with CU-P2-05 studios.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { LibraryStudioWorkbenchComponent } from "./lib/library-studio-workbench/library-studio-workbench.component";
import { LIBRARY_STUDIO_WORKBENCH_DESCRIPTOR } from "./lib/library-studio-workbench/library-studio-workbench.logic";

export const WORKBENCH_DESCRIPTOR = LIBRARY_STUDIO_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: LibraryStudioWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
