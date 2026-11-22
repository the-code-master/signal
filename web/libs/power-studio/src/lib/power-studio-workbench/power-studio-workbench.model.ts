// Module: web/libs/power-studio/src/lib/power-studio-workbench/power-studio-workbench.model.ts — Power Studio Host Models
// Invariants: Organizes FEAT-022..030 PDN/Creepage/Hot-Loop, Bode/BMS/FOC, and Thermal/Sequencing studios cleanly.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type PowerStudioTabId = "PDN_CREEPAGE_HOTLOOP" | "BODE_BMS_MOTOR" | "THERMAL_SEQUENCING";

export interface PowerStudioTabOption {
  readonly id: PowerStudioTabId;
  readonly label: string;
}

export interface PowerStudioWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activeTab: PowerStudioTabId;
  readonly tabs: readonly PowerStudioTabOption[];
}
