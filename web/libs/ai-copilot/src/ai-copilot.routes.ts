// Module: web/libs/ai-copilot/src/ai-copilot.routes.ts — Lazy Angular v22+ Routes for Neuro-Symbolic AI Copilot & Quarantined RAG
// Invariants: Zero eager bundle cost; mounts AiCopilotWorkbenchComponent with ProofBundle & I-10 Import Wizard.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { Routes } from "@angular/router";
import { AiCopilotWorkbenchComponent } from "./lib/ai-copilot-workbench/ai-copilot-workbench.component";
import { AI_COPILOT_WORKBENCH_DESCRIPTOR } from "./lib/ai-copilot-workbench/ai-copilot-workbench.logic";

export const WORKBENCH_DESCRIPTOR = AI_COPILOT_WORKBENCH_DESCRIPTOR;

export const WORKBENCH_ROUTES: Routes = [
  {
    path: "",
    component: AiCopilotWorkbenchComponent,
    data: { workbench: WORKBENCH_DESCRIPTOR },
  },
];
