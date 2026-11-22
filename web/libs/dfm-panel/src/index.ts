// Module: web/libs/dfm-panel/src/index.ts — Public Barrel for @signal/dfm-panel
// Invariants: Sole public entrypoint for lazy route loading and FEAT-063..069 + FEAT-080 DFM modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./dfm-panel.routes";
export { DfmPanelWorkbenchComponent } from "./lib/dfm-panel-workbench/dfm-panel-workbench.component";
export {
  DFM_PANEL_WORKBENCH_DESCRIPTOR,
  buildDfmPanelWorkbenchVm,
  computeUnitShouldCostMicroUsd,
} from "./lib/dfm-panel-workbench/dfm-panel-workbench.logic";
export type {
  DfmPanelWorkbenchVm,
  ProductionVolumeTier,
} from "./lib/dfm-panel-workbench/dfm-panel-workbench.model";
export { DrawingIctMslComponent } from "./lib/drawing-ict-msl/drawing-ict-msl.component";
export {
  buildDrawingIctMslVm,
  computeCombinedTestCoveragePct,
  floorLifeHoursForMsl,
} from "./lib/drawing-ict-msl/drawing-ict-msl.logic";
export type {
  DrawingIctMslVm,
  Jstd020MslLevel,
} from "./lib/drawing-ict-msl/drawing-ict-msl.model";
export { PanelDfaFatigueCoatingComponent } from "./lib/panel-dfa-fatigue-coating/panel-dfa-fatigue-coating.component";
export {
  buildPanelDfaFatigueCoatingVm,
  computeDepanelingMicrostrain,
  computeStencilAreaRatio,
} from "./lib/panel-dfa-fatigue-coating/panel-dfa-fatigue-coating.logic";
export type {
  DepanelingMethod,
  PanelDfaFatigueCoatingVm,
} from "./lib/panel-dfa-fatigue-coating/panel-dfa-fatigue-coating.model";
