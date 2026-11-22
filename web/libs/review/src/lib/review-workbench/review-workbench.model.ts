// Module: web/libs/review/src/lib/review-workbench/review-workbench.model.ts — Spatial Review Threads & Gate Console Models
// Invariants: Spatial review thread anchors use ES2020 bigint nanometers (_nm) (F-074).
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export interface SpatialReviewThread {
  readonly threadId: string;
  readonly entityAnchor: string;
  readonly anchorXNm: bigint;
  readonly anchorYNm: bigint;
  readonly comment: string;
  readonly resolved: boolean;
}

export interface ReviewWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly openThreadBadge: string;
  readonly threadColumns: readonly SigGridColumn[];
  readonly threadRows: readonly SigGridRow[];
}
