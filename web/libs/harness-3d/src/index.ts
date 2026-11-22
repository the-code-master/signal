// Module: web/libs/harness-3d/src/index.ts — Public Barrel for @signal/harness-3d
// Invariants: Sole public entrypoint for lazy route loading and FEAT-020/061/062 Harness 3D modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./harness-3d.routes";
export { Harness3dWorkbenchComponent } from "./lib/harness-3d-workbench/harness-3d-workbench.component";
export {
  HARNESS_3D_WORKBENCH_DESCRIPTOR,
  buildHarness3dWorkbenchVm,
} from "./lib/harness-3d-workbench/harness-3d-workbench.logic";
export type { Harness3dWorkbenchVm } from "./lib/harness-3d-workbench/harness-3d-workbench.model";
export { MultiboardFlexNailboardComponent } from "./lib/multiboard-flex-nailboard/multiboard-flex-nailboard.component";
export {
  DEFAULT_NAILBOARD_BRANCHES,
  buildMultiboardFlexNailboardVm,
  computeOuterFiberCopperStrainPct,
} from "./lib/multiboard-flex-nailboard/multiboard-flex-nailboard.logic";
export type {
  HarnessWireBranch,
  MultiboardFlexNailboardVm,
} from "./lib/multiboard-flex-nailboard/multiboard-flex-nailboard.model";
