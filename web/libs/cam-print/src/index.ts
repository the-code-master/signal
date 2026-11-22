// Module: web/libs/cam-print/src/index.ts — Public Barrel for @signal/cam-print
// Invariants: Sole public entrypoint for lazy route loading and CU-P5-04 CNC CAM + Direct-B-Rep Slicer modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./cam-print.routes";
export { CamPrintWorkbenchComponent } from "./lib/cam-print-workbench/cam-print-workbench.component";
export {
  CAM_PRINT_WORKBENCH_DESCRIPTOR,
  buildCamPrintWorkbenchVm,
} from "./lib/cam-print-workbench/cam-print-workbench.logic";
export type {
  CamPrintTabId,
  CamPrintWorkbenchVm,
} from "./lib/cam-print-workbench/cam-print-workbench.model";
export { CncToolpathStudioComponent } from "./lib/cnc-toolpath-studio/cnc-toolpath-studio.component";
export {
  DEFAULT_CNC_OPERATIONS,
  buildCncToolpathStudioVm,
  computeChipLoadNm,
} from "./lib/cnc-toolpath-studio/cnc-toolpath-studio.logic";
export type {
  CncOperationRow,
  CncPostProfile,
  CncToolpathStudioVm,
} from "./lib/cnc-toolpath-studio/cnc-toolpath-studio.model";
export { DirectBrepSlicerComponent } from "./lib/direct-brep-slicer/direct-brep-slicer.component";
export {
  buildDirectBrepSlicerVm,
  computeExactSliceLayerCount,
  defaultLayerHeightNmForProcess,
} from "./lib/direct-brep-slicer/direct-brep-slicer.logic";
export type {
  AdditiveProcessMode,
  DirectBrepSlicerVm,
} from "./lib/direct-brep-slicer/direct-brep-slicer.model";
