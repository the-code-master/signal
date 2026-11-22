// Module: web/libs/power-studio/src/index.ts — Public Barrel for @signal/power-studio
// Invariants: Sole public entrypoint for lazy route loading and FEAT-022..030 Power Studio modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { BodeBmsMotorComponent } from "./lib/bode-bms-motor/bode-bms-motor.component";
export {
  buildBodeBmsMotorVm,
  computeBmsPackVoltageUv,
  computePmsmDqTorqueNm,
} from "./lib/bode-bms-motor/bode-bms-motor.logic";
export type { BodeBmsMotorVm, LoopCompensatorTopology } from "./lib/bode-bms-motor/bode-bms-motor.model";
export { PdnCreepageIrHotloopComponent } from "./lib/pdn-creepage-ir-hotloop/pdn-creepage-ir-hotloop.component";
export {
  DEFAULT_PDN_BANDS,
  buildPdnCreepageIrHotloopVm,
  compute3dGeodesicCreepageNm,
  computeMillerShootThroughMarginUv,
} from "./lib/pdn-creepage-ir-hotloop/pdn-creepage-ir-hotloop.logic";
export type {
  PdnCreepageIrHotloopVm,
  PdnFrequencyBand,
} from "./lib/pdn-creepage-ir-hotloop/pdn-creepage-ir-hotloop.model";
export { PowerStudioWorkbenchComponent } from "./lib/power-studio-workbench/power-studio-workbench.component";
export {
  POWER_STUDIO_WORKBENCH_DESCRIPTOR,
  buildPowerStudioWorkbenchVm,
} from "./lib/power-studio-workbench/power-studio-workbench.logic";
export type {
  PowerStudioTabId,
  PowerStudioWorkbenchVm,
} from "./lib/power-studio-workbench/power-studio-workbench.model";
export { ThermalSequencingComponent } from "./lib/thermal-sequencing/thermal-sequencing.component";
export {
  DEFAULT_SEQ_RAILS,
  buildThermalSequencingVm,
  computePeakTjMkelvin,
  verifyMonotonicSequencing,
} from "./lib/thermal-sequencing/thermal-sequencing.logic";
export type {
  CoolingAssemblyKind,
  PowerSeqRailSlot,
  ThermalSequencingVm,
} from "./lib/thermal-sequencing/thermal-sequencing.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./power-studio.routes";
