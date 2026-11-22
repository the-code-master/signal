// Module: web/libs/harness-3d/src/lib/harness-3d-workbench/harness-3d-workbench.model.ts — Harness 3D Host Models
// Invariants: Bundle bend radii (_nm) stay in ES2020 bigint across routing harnesses.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export interface Harness3dWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly minBundleBendNm: bigint;
  readonly bendMarginBadge: string;
}
