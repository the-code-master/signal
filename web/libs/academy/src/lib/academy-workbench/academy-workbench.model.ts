// Module: web/libs/academy/src/lib/academy-workbench/academy-workbench.model.ts — Academy Workbench Models
// Invariants: Integrates FEAT-113 Visual SOA Smoke Sandbox and FEAT-114 Quick-Calc HUD cleanly.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export interface AcademyWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activityBanner: string;
}
