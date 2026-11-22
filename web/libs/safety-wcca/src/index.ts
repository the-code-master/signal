// Module: web/libs/safety-wcca/src/index.ts — Public Barrel for @signal/safety-wcca
// Invariants: Sole public entrypoint for lazy route loading and FEAT-035..038, FEAT-040 & I-05 Aging modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { SafetyWccaWorkbenchComponent } from "./lib/safety-wcca-workbench/safety-wcca-workbench.component";
export {
  SAFETY_WCCA_WORKBENCH_DESCRIPTOR,
  buildSafetyWccaWorkbenchVm,
  evaluateAsilGatePassing,
} from "./lib/safety-wcca-workbench/safety-wcca-workbench.logic";
export type {
  AsilTargetLevel,
  SafetyWccaWorkbenchVm,
} from "./lib/safety-wcca-workbench/safety-wcca-workbench.model";
export { WccaRadhardDeratingAgingComponent } from "./lib/wcca-radhard-derating-aging/wcca-radhard-derating-aging.component";
export {
  buildWccaRadhardDeratingAgingVm,
  computeNbtiShiftUv,
  maxVoltageStressRatioForPolicy,
} from "./lib/wcca-radhard-derating-aging/wcca-radhard-derating-aging.logic";
export type {
  DeratingPolicyStandard,
  WccaRadhardDeratingAgingVm,
} from "./lib/wcca-radhard-derating-aging/wcca-radhard-derating-aging.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./safety-wcca.routes";
