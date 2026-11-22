// Module: web/libs/collab-hub/src/lib/collab-hub-workbench/collab-hub-workbench.model.ts — Collab Workbench Host Models
// Invariants: Pure readonly TypeScript types; 64-bit _nm Jump-to-Canvas focus coordinates use bigint ONLY.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import type { CanvasJumpEvent } from "../team-channels-huddle/team-channels-huddle.model";

export interface CollabHubWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly cellIsolationBadge: string;
  readonly jumpHudLabel: string;
  readonly hudSignals: readonly string[];
  readonly lastJump: CanvasJumpEvent | null;
}
