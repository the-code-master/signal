// Module: web/libs/academy/src/index.ts — Public Barrel for @signal/academy
// Invariants: Sole public entrypoint for lazy route loading and FEAT-113 / FEAT-114 Academy tools.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./academy.routes";
export { AcademyWorkbenchComponent } from "./lib/academy-workbench/academy-workbench.component";
export { buildAcademyWorkbenchVm } from "./lib/academy-workbench/academy-workbench.logic";
export type { AcademyWorkbenchVm } from "./lib/academy-workbench/academy-workbench.model";
export { QuickCalcHudComponent } from "./lib/quick-calc-hud/quick-calc-hud.component";
export {
  buildQuickCalcHudVm,
  computeLedSeriesResistor,
  computeVoltageDivider,
  decodeColorBandResistor,
  decodeEia96SmdCode,
  formatResistanceMohm,
} from "./lib/quick-calc-hud/quick-calc-hud.logic";
export type {
  DividerCalcResult,
  LedLimiterCalcResult,
  QuickCalcHudVm,
  ResistorBandColor,
  ResistorDecodeResult,
  SchematicInsertPayload,
} from "./lib/quick-calc-hud/quick-calc-hud.model";
export { VisualFailureFxComponent } from "./lib/visual-failure-fx/visual-failure-fx.component";
export { buildVisualFailureFxVm, DEFAULT_SOA_SCENARIOS } from "./lib/visual-failure-fx/visual-failure-fx.logic";
export type {
  AutoFixUpgradeEvent,
  SoaFailureScenario,
  VisualFailureFxVm,
} from "./lib/visual-failure-fx/visual-failure-fx.model";
