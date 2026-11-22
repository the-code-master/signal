// Module: web/libs/mcad-studio/src/lib/mcad-studio-workbench/mcad-studio-workbench.model.ts — MCAD Host Models
// Invariants: Section-plane offsets (_nm) and PCB-enclosure clearances (_nm) stay in ES2020 bigint.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import type { SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export interface McadStudioWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly explodedPct: number;
  readonly explodedPresets: readonly number[];
  readonly sectionPlaneOffsetNm: bigint;
  readonly minEnclosureClearanceNm: bigint;
  readonly collisionCount: number;
  readonly viewBadge: string;
  readonly clearanceTone: SigStatusTone;
}
