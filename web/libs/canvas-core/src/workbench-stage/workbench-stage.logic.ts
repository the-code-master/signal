// Module: web/libs/canvas-core/src/workbench-stage/workbench-stage.logic.ts — Pure Workbench Stage View-Model Builder
// Invariants: Pre-computes badge strings and fallback descriptor outside HTML templates.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "./workbench-stage.model";

export interface WorkbenchStageVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
}

export function buildWorkbenchStageVm(descriptor: WorkbenchDescriptor): WorkbenchStageVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
  };
}
