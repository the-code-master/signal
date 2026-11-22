// Module: web/libs/pcb/src/lib/pcb-workbench/pcb-workbench.model.ts — CU-P4-01 224G & HV PCB Workbench Host Models
// Invariants: Organizes PNS/Meanders, SI/EM/RF/TDR, and Stackup/Constraints/X-Ray studios cleanly.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type PcbStudioTabId = "PNS_MEANDERS" | "SI_EM_RF_TDR" | "STACKUP_CONSTRAINTS_XRAY";

export interface PcbStudioTabOption {
  readonly id: PcbStudioTabId;
  readonly label: string;
}

export interface PcbWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activeTab: PcbStudioTabId;
  readonly tabs: readonly PcbStudioTabOption[];
}
