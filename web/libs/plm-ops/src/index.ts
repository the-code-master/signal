// Module: web/libs/plm-ops/src/index.ts — Public Barrel for @signal/plm-ops
// Invariants: Sole public entrypoint for lazy route loading and CU-P5-05 Multi-BOM + Change/Deviation/CTB modules.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { ChangeDeviationCtbTowerComponent } from "./lib/change-deviation-ctb-tower/change-deviation-ctb-tower.component";
export {
  DEFAULT_ASSEMBLY_WORK_STEPS,
  buildChangeDeviationCtbTowerVm,
  isDeviationReleasePermitted,
} from "./lib/change-deviation-ctb-tower/change-deviation-ctb-tower.logic";
export type {
  AssemblyInstructionStep,
  ChangeDeviationCtbTowerVm,
} from "./lib/change-deviation-ctb-tower/change-deviation-ctb-tower.model";
export { MultiBomPassportComponent } from "./lib/multi-bom-passport/multi-bom-passport.component";
export {
  DEFAULT_MULTI_BOM_ITEMS,
  buildMultiBomPassportVm,
  filterItemsForBomView,
} from "./lib/multi-bom-passport/multi-bom-passport.logic";
export type {
  MultiBomPassportVm,
  PlmBomItemRow,
  PlmBomViewMode,
} from "./lib/multi-bom-passport/multi-bom-passport.model";
export { PlmOpsWorkbenchComponent } from "./lib/plm-ops-workbench/plm-ops-workbench.component";
export {
  PLM_OPS_WORKBENCH_DESCRIPTOR,
  buildPlmOpsWorkbenchVm,
} from "./lib/plm-ops-workbench/plm-ops-workbench.logic";
export type {
  ErpConnectorPreset,
  PlmOpsWorkbenchVm,
} from "./lib/plm-ops-workbench/plm-ops-workbench.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./plm-ops.routes";
