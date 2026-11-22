// Module: web/libs/ic/src/lib/ic-workbench/ic-workbench.model.ts — CU-P6-01 IC / ASIC Workbench Host Models
// Invariants: Organizes Analog/BSPDN, Digital SoC/STA/UCIe/Litho, and UPF/CPO/Cryo/Security studios cleanly.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type IcStudioTabId = "ANALOG_GMID_BSPDN" | "SOC_STA_UCIE_LITHO" | "UPF_CPO_CRYO_SEC";

export interface IcStudioTabOption {
  readonly id: IcStudioTabId;
  readonly label: string;
}

export interface IcWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activeTab: IcStudioTabId;
  readonly tabs: readonly IcStudioTabOption[];
}
