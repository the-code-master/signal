// Module: web/libs/emc-lab/src/index.ts — Public Barrel for @signal/emc-lab
// Invariants: Sole public entrypoint for lazy route loading and FEAT-031..034 + FEAT-039 EMC modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./emc-lab.routes";
export { EmcLabWorkbenchComponent } from "./lib/emc-lab-workbench/emc-lab-workbench.component";
export {
  DEFAULT_EMC_PEAKS,
  EMC_LAB_WORKBENCH_DESCRIPTOR,
  buildEmcLabWorkbenchVm,
  isPiezoAcousticAudibleRisk,
  limitDbuvForStandard,
} from "./lib/emc-lab-workbench/emc-lab-workbench.logic";
export type {
  EmcHarmonicPeak,
  EmcLabWorkbenchVm,
  EmcStandardMask,
} from "./lib/emc-lab-workbench/emc-lab-workbench.model";
export { EmiEsdShieldComponent } from "./lib/emi-esd-shield/emi-esd-shield.component";
export {
  buildEmiEsdShieldVm,
  computeCavityTe101Hz,
  computeTvsClampedPeakUv,
} from "./lib/emi-esd-shield/emi-esd-shield.logic";
export type { EmiEsdShieldVm, EsdStressLevel } from "./lib/emi-esd-shield/emi-esd-shield.model";
