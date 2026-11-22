// Module: web/libs/academy/src/index.ts — Public Barrel for @signal/academy
// Invariants: Sole public entrypoint for lazy route loading and FEAT-111..FEAT-115 Academy & Open-Silicon tools.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./academy.routes";
export { AcademyWorkbenchComponent } from "./lib/academy-workbench/academy-workbench.component";
export { buildAcademyWorkbenchVm } from "./lib/academy-workbench/academy-workbench.logic";
export type { AcademyWorkbenchVm } from "./lib/academy-workbench/academy-workbench.model";
export { LabRunnerComponent } from "./lib/lab-runner/lab-runner.component";
export { buildLabRunnerVm, evaluateHeadlessLabRubric } from "./lib/lab-runner/lab-runner.logic";
export type { AutoGraderRunResult, LabFaultCondition, LabRunnerVm, LtiLmsPlatform } from "./lib/lab-runner/lab-runner.model";
export { OpenSiliconWizardComponent } from "./lib/open-silicon-wizard/open-silicon-wizard.component";
export { buildOpenSiliconWizardVm, validateOpenSiliconShuttle } from "./lib/open-silicon-wizard/open-silicon-wizard.logic";
export type {
  OpenSiliconPdkId,
  OpenSiliconWizardVm,
  ShuttlePrecheckReport,
  ShuttleTileSlot,
} from "./lib/open-silicon-wizard/open-silicon-wizard.model";
export { BreadboardTwinComponent } from "./lib/breadboard-twin/breadboard-twin.component";
export {
  analyzeBreadboardConnectivity,
  buildBreadboardTwinVm,
  holeToCanonical5HoleStrip,
} from "./lib/breadboard-twin/breadboard-twin.logic";
export type {
  BreadboardConnectivityAnalysis,
  BreadboardJumperWire,
  BreadboardTwinVm,
} from "./lib/breadboard-twin/breadboard-twin.model";
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
