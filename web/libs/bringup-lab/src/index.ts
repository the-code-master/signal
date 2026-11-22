// Module: web/libs/bringup-lab/src/index.ts — Public Barrel for @signal/bringup-lab
// Invariants: Sole public entrypoint for lazy route loading and CU-P6-04 Digital Twin, Boot/HIL & Shmoo modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./bringup-lab.routes";
export { BootHilBoardmapComponent } from "./lib/boot-hil-boardmap/boot-hil-boardmap.component";
export {
  DEFAULT_BOARD_TESTPOINTS,
  buildBootHilBoardmapVm,
  hilRecoveryTimePsForScenario,
} from "./lib/boot-hil-boardmap/boot-hil-boardmap.logic";
export type {
  BoardTestpointLookup,
  BootHilBoardmapVm,
  HilFaultScenario,
} from "./lib/boot-hil-boardmap/boot-hil-boardmap.model";
export { BringupLabWorkbenchComponent } from "./lib/bringup-lab-workbench/bringup-lab-workbench.component";
export {
  BRINGUP_LAB_WORKBENCH_DESCRIPTOR,
  buildBringupLabWorkbenchVm,
} from "./lib/bringup-lab-workbench/bringup-lab-workbench.logic";
export type {
  BringupLabWorkbenchVm,
  LabCaptureSource,
} from "./lib/bringup-lab-workbench/bringup-lab-workbench.model";
export { ShmooPlotStudioComponent } from "./lib/shmoo-plot-studio/shmoo-plot-studio.component";
export {
  buildShmooPlotStudioVm,
  evaluateMaxClockAtCornerHz,
  generateShmooGrid,
} from "./lib/shmoo-plot-studio/shmoo-plot-studio.logic";
export type {
  ShmooCellPoint,
  ShmooPlotStudioVm,
} from "./lib/shmoo-plot-studio/shmoo-plot-studio.model";
