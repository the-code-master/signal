// Module: web/libs/library-studio/src/index.ts — Public Barrel for @signal/library-studio
// Invariants: Sole public entrypoint for lazy route loading and CU-P2-05 Universal Parts studios.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { CurveSdkSnippetsComponent } from "./lib/curve-sdk-snippets/curve-sdk-snippets.component";
export {
  buildCurveSdkSnippetsVm,
  computeRelativeRmsErrorPct,
  stampGoldenSnippetPreview,
} from "./lib/curve-sdk-snippets/curve-sdk-snippets.logic";
export type {
  CurveModelFamily,
  CurveSdkSnippetsVm,
  GoldenSnippetDescriptor,
  StampedSnippetPreview,
} from "./lib/curve-sdk-snippets/curve-sdk-snippets.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./library-studio.routes";
export { LibraryStudioWorkbenchComponent } from "./lib/library-studio-workbench/library-studio-workbench.component";
export { buildLibraryStudioWorkbenchVm } from "./lib/library-studio-workbench/library-studio-workbench.logic";
export type { LibraryStudioTabId, LibraryStudioWorkbenchVm } from "./lib/library-studio-workbench/library-studio-workbench.model";
export { PadstackFootprintStepComponent } from "./lib/padstack-footprint-step/padstack-footprint-step.component";
export {
  buildPadstackFootprintStepVm,
  computeIpc7351CourtyardSpanNm,
  evaluatePadstackMetrics,
} from "./lib/padstack-footprint-step/padstack-footprint-step.logic";
export type {
  Ipc7351Density,
  PadstackFootprintStepVm,
  PadstackMetricsResult,
  PadstackSculptorInput,
} from "./lib/padstack-footprint-step/padstack-footprint-step.model";
export { PinSymbolSplitterComponent } from "./lib/pin-symbol-splitter/pin-symbol-splitter.component";
export {
  buildPinSymbolSplitterVm,
  splitMasterPinsIntoBanks,
  verifyMasterPinBijection,
} from "./lib/pin-symbol-splitter/pin-symbol-splitter.logic";
export type {
  MasterPinRow,
  PartitionedSymbolBank,
  PinSymbolSplitterVm,
  SchematicSymbolStandard,
} from "./lib/pin-symbol-splitter/pin-symbol-splitter.model";
export { DatasheetTestbenchComponent } from "./lib/datasheet-testbench/datasheet-testbench.component";
export {
  DEFAULT_TESTBENCH_CURVES,
  buildDatasheetTestbenchVm,
  computeCurveOverlayRmsPct,
} from "./lib/datasheet-testbench/datasheet-testbench.logic";
export type {
  DatasheetTestbenchVm,
  ExtractedConditionCurve,
} from "./lib/datasheet-testbench/datasheet-testbench.model";
