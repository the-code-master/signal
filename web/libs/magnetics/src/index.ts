// Module: web/libs/magnetics/src/index.ts — Public Barrel for @signal/magnetics
// Invariants: Sole public entrypoint for lazy route loading and FEAT-021 magnetics studio.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { MagneticsWorkbenchComponent } from "./lib/magnetics-workbench/magnetics-workbench.component";
export {
  MAGNETICS_WORKBENCH_DESCRIPTOR,
  buildMagneticsWorkbenchVm,
  computePeakFluxUt,
} from "./lib/magnetics-workbench/magnetics-workbench.logic";
export type {
  MagneticCoreCandidate,
  MagneticsWorkbenchVm,
  WindingGeometryKind,
} from "./lib/magnetics-workbench/magnetics-workbench.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./magnetics.routes";
