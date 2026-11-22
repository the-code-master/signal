// Module: web/libs/fpga-studio/src/index.ts — Public Barrel for @signal/fpga-studio
// Invariants: Sole public entrypoint for lazy route loading and CU-P6-02 FPGA I/O, SDC, FSM & DSP modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./fpga-studio.routes";
export { DieIoSdcPlannerComponent } from "./lib/die-io-sdc-planner/die-io-sdc-planner.component";
export {
  buildDieIoSdcPlannerVm,
  buildFpgaBanks,
} from "./lib/die-io-sdc-planner/die-io-sdc-planner.logic";
export type {
  DieIoSdcPlannerVm,
  FpgaIoBankRow,
} from "./lib/die-io-sdc-planner/die-io-sdc-planner.model";
export { FpgaStudioWorkbenchComponent } from "./lib/fpga-studio-workbench/fpga-studio-workbench.component";
export {
  FPGA_STUDIO_WORKBENCH_DESCRIPTOR,
  buildFpgaStudioWorkbenchVm,
} from "./lib/fpga-studio-workbench/fpga-studio-workbench.logic";
export type {
  FpgaPartPreset,
  FpgaStudioWorkbenchVm,
} from "./lib/fpga-studio-workbench/fpga-studio-workbench.model";
export { FsmDspStudioComponent } from "./lib/fsm-dsp-studio/fsm-dsp-studio.component";
export {
  buildFsmDspStudioVm,
  computeFixedPointSqnrDb,
} from "./lib/fsm-dsp-studio/fsm-dsp-studio.logic";
export type {
  FsmDspStudioVm,
  FsmEncodingStyle,
} from "./lib/fsm-dsp-studio/fsm-dsp-studio.model";
