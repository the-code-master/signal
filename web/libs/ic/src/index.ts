// Module: web/libs/ic/src/index.ts — Public Barrel for @signal/ic
// Invariants: Sole public entrypoint for lazy route loading and CU-P6-01 IC/ASIC studios.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./ic.routes";
export { AnalogGmidLibertyComponent } from "./lib/analog-gmid-liberty/analog-gmid-liberty.component";
export {
  buildAnalogGmidLibertyVm,
  computePelgromSigmaVthMv,
} from "./lib/analog-gmid-liberty/analog-gmid-liberty.logic";
export type {
  AnalogGmidLibertyVm,
  InversionRegionPreset,
} from "./lib/analog-gmid-liberty/analog-gmid-liberty.model";
export { IcWorkbenchComponent } from "./lib/ic-workbench/ic-workbench.component";
export { IC_WORKBENCH_DESCRIPTOR, buildIcWorkbenchVm } from "./lib/ic-workbench/ic-workbench.logic";
export type { IcStudioTabId, IcWorkbenchVm } from "./lib/ic-workbench/ic-workbench.model";
export { SocStaUcieLithoComponent } from "./lib/soc-sta-ucie-litho/soc-sta-ucie-litho.component";
export {
  buildSocStaUcieLithoVm,
  computePostEcoWnsFs,
} from "./lib/soc-sta-ucie-litho/soc-sta-ucie-litho.logic";
export type { SocStaUcieLithoVm } from "./lib/soc-sta-ucie-litho/soc-sta-ucie-litho.model";
export { UpfCpoCryoSecComponent } from "./lib/upf-cpo-cryo-sec/upf-cpo-cryo-sec.component";
export {
  buildUpfCpoCryoSecVm,
  isTvlaSideChannelLeaking,
} from "./lib/upf-cpo-cryo-sec/upf-cpo-cryo-sec.logic";
export type { UpfCpoCryoSecVm } from "./lib/upf-cpo-cryo-sec/upf-cpo-cryo-sec.model";
