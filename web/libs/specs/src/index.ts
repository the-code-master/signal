// Module: web/libs/specs/src/index.ts — Public Barrel for @signal/specs
// Invariants: Sole public entrypoint for lazy route loading and CU-P2-04 PRD/BOM provenance tools.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { SpecsBomWorkbenchComponent } from "./lib/specs-bom-workbench/specs-bom-workbench.component";
export {
  buildSpecsBomWorkbenchVm,
  evaluateRequirementItem,
  verifyBomOfferProvenance,
} from "./lib/specs-bom-workbench/specs-bom-workbench.logic";
export type {
  BomDistributorOffer,
  BomMatrixLine,
  ExecutableRequirementItem,
  ProvenanceStatus,
  SpecsBomWorkbenchVm,
} from "./lib/specs-bom-workbench/specs-bom-workbench.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./specs.routes";
