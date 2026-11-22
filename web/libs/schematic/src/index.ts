// Module: web/libs/schematic/src/index.ts — Public Barrel for @signal/schematic
// Invariants: Sole public entrypoint for lazy route loading and FEAT-001..FEAT-010 schematic panels.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { BeautifyBusComponent } from "./lib/beautify-bus/beautify-bus.component";
export { buildBeautifyBusVm, expandBusSliceExpression } from "./lib/beautify-bus/beautify-bus.logic";
export type { BeautifyBusVm, BeautifyPreviewInput, BusProtocolKind, BusSliceExpansion } from "./lib/beautify-bus/beautify-bus.model";
export { HierarchyVariantsComponent } from "./lib/hierarchy-variants/hierarchy-variants.component";
export {
  buildHierarchyVariantsVm,
  expandMultiChannelMatrix,
  resolveVariantStuffingRows,
} from "./lib/hierarchy-variants/hierarchy-variants.logic";
export type {
  AssemblyVariantId,
  HierarchyVariantsVm,
  MultiChannelRuleInput,
  NetPortalDescriptor,
  VariantBomItem,
} from "./lib/hierarchy-variants/hierarchy-variants.model";
export { LiveSimHudComponent } from "./lib/live-sim-hud/live-sim-hud.component";
export {
  buildLiveSimHudVm,
  computeParticleDensityTier,
  computePotentiometerLegsMohm,
} from "./lib/live-sim-hud/live-sim-hud.logic";
export type { LiveSimHudState, LiveSimHudVm, SchematicFaultMode } from "./lib/live-sim-hud/live-sim-hud.model";
export { PinswapEquationsComponent } from "./lib/pinswap-equations/pinswap-equations.component";
export {
  buildPinswapEquationsVm,
  evaluateParametricEquation,
  reannotateSpatialEntities,
} from "./lib/pinswap-equations/pinswap-equations.logic";
export type {
  EquationEvalResult,
  PinSwapCandidate,
  PinswapEquationsVm,
  RefdesRemapResult,
  SpatialComponentAnchor,
  SpatialSweepDirection,
} from "./lib/pinswap-equations/pinswap-equations.model";
export { SchematicWorkbenchComponent } from "./lib/schematic-workbench/schematic-workbench.component";
export { buildSchematicWorkbenchVm } from "./lib/schematic-workbench/schematic-workbench.logic";
export type { SchematicTabId, SchematicWorkbenchVm } from "./lib/schematic-workbench/schematic-workbench.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./schematic.routes";
