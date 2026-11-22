// Module: web/libs/mcad-studio/src/index.ts — Public Barrel for @signal/mcad-studio
// Invariants: Sole public entrypoint for lazy route loading and CU-P5-03 B-Rep/Sheet-Metal/Anchor modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { BrepSheetmetalAnchorsComponent } from "./lib/brep-sheetmetal-anchors/brep-sheetmetal-anchors.component";
export {
  DEFAULT_BREP_FEATURES,
  buildBrepSheetmetalAnchorsVm,
  computeSheetMetalUnfoldedNm,
} from "./lib/brep-sheetmetal-anchors/brep-sheetmetal-anchors.logic";
export type {
  BRepTimelineNode,
  BrepSheetmetalAnchorsVm,
} from "./lib/brep-sheetmetal-anchors/brep-sheetmetal-anchors.model";
export { McadStudioWorkbenchComponent } from "./lib/mcad-studio-workbench/mcad-studio-workbench.component";
export {
  MCAD_STUDIO_WORKBENCH_DESCRIPTOR,
  buildMcadStudioWorkbenchVm,
} from "./lib/mcad-studio-workbench/mcad-studio-workbench.logic";
export type { McadStudioWorkbenchVm } from "./lib/mcad-studio-workbench/mcad-studio-workbench.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./mcad-studio.routes";
