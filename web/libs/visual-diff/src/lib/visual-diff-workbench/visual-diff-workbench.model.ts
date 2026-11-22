// Module: web/libs/visual-diff/src/lib/visual-diff-workbench/visual-diff-workbench.model.ts — Onion-Skin Semantic Diff Models
// Invariants: Geometric deltas stay in ES2020 bigint nanometers (_nm) across SCHEMATIC, PCB & CAD_MCAD.
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type DiffDomainFilter = "ALL" | "SCHEMATIC" | "PCB" | "CAD_MCAD";

export interface SemanticDiffRecord {
  readonly id: string;
  readonly domain: Exclude<DiffDomainFilter, "ALL">;
  readonly entityId: string;
  readonly changeKind: "ADDED" | "MODIFIED" | "REMOVED";
  readonly deltaMetricNm: bigint;
  readonly summary: string;
}

export interface VisualDiffWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activeDomain: DiffDomainFilter;
  readonly domains: readonly DiffDomainFilter[];
  readonly activeRevisionText: string;
  readonly diffColumns: readonly SigGridColumn[];
  readonly diffRows: readonly SigGridRow[];
}
