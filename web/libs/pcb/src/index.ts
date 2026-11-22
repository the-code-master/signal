// Module: web/libs/pcb/src/index.ts — Public Barrel for @signal/pcb
// Invariants: Sole public entrypoint for lazy route loading and CU-P4-01 PCB studios.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { PcbWorkbenchComponent } from "./lib/pcb-workbench/pcb-workbench.component";
export { PCB_WORKBENCH_DESCRIPTOR, buildPcbWorkbenchVm } from "./lib/pcb-workbench/pcb-workbench.logic";
export type { PcbStudioTabId, PcbWorkbenchVm } from "./lib/pcb-workbench/pcb-workbench.model";
export { PnsRoutingMeandersComponent } from "./lib/pns-routing-meanders/pns-routing-meanders.component";
export {
  DEFAULT_PCIE6_CHANNELS,
  buildPnsRoutingMeandersVm,
  computeRequiredTromboneLoops,
  computeTotalFlightDelayFs,
} from "./lib/pns-routing-meanders/pns-routing-meanders.logic";
export type {
  DiffPairFlightChannel,
  PnsRouterMode,
  PnsRoutingMeandersVm,
} from "./lib/pns-routing-meanders/pns-routing-meanders.model";
export { SiEmRfInspectorComponent } from "./lib/si-em-rf-inspector/si-em-rf-inspector.component";
export {
  DEFAULT_TDR_PROFILE,
  buildSiEmRfInspectorVm,
  computeFiberWeaveSkewFs,
  computeViaStubQuarterWaveHz,
  synthesizeRfQuarterWaveNm,
} from "./lib/si-em-rf-inspector/si-em-rf-inspector.logic";
export type {
  RfTopologyPreset,
  SiEmRfInspectorVm,
  TdrSpatialSegment,
} from "./lib/si-em-rf-inspector/si-em-rf-inspector.model";
export { StackupConstraintsXrayComponent } from "./lib/stackup-constraints-xray/stackup-constraints-xray.component";
export {
  DEFAULT_HDI_SPANS,
  DEFAULT_NET_CLASSES,
  buildStackupConstraintsXrayVm,
  computeViaAspectRatio,
  computeXrayLayerBitmaskHex,
} from "./lib/stackup-constraints-xray/stackup-constraints-xray.logic";
export type {
  HdiViaSpanSpec,
  LayerXrayPreset,
  NetClassConstraintRow,
  StackupConstraintsXrayVm,
} from "./lib/stackup-constraints-xray/stackup-constraints-xray.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./pcb.routes";
