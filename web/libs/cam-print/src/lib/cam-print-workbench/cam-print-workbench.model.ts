// Module: web/libs/cam-print/src/lib/cam-print-workbench/cam-print-workbench.model.ts — CAM & 3D Print Host Models
// Invariants: Organizes subtractive CNC CAM and additive Direct-B-Rep 3D Printing studios cleanly.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type CamPrintTabId = "SUBTRACTIVE_CNC_CAM" | "ADDITIVE_DIRECT_BREP_SLICER";

export interface CamPrintTabOption {
  readonly id: CamPrintTabId;
  readonly label: string;
}

export interface CamPrintWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activeTab: CamPrintTabId;
  readonly tabs: readonly CamPrintTabOption[];
}
