// Module: web/libs/sim-viewer/src/lib/sim-viewer-workbench/sim-viewer-workbench.model.ts — Sim-Viewer Workbench Models
// Invariants: Organizes CU-P1-03 + CU-P1-04 SPICE, Virtual Bench, Adjoint & Analytics studios cleanly.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type SimStudioTabId =
  | "VIRTUAL_BENCH"
  | "MONTE_CARLO"
  | "JITTER_STUDIO"
  | "WHAT_IF_ADJOINT"
  | "REGRESSION_SUITE"
  | "STIMULUS_AUDITION";

export interface SimStudioTabOption {
  readonly id: SimStudioTabId;
  readonly label: string;
}

export interface SimViewerWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activeTab: SimStudioTabId;
  readonly tabs: readonly SimStudioTabOption[];
  readonly crossProbeBadge: string;
}
