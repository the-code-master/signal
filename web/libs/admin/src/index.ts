// Module: web/libs/admin/src/index.ts — Public Barrel for @signal/admin
// Invariants: Sole public entrypoint for lazy route loading and Access Review / FEAT-075 AVL tools.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./admin.routes";
export { AdminWorkbenchComponent } from "./lib/admin-workbench/admin-workbench.component";
export { buildAdminWorkbenchVm } from "./lib/admin-workbench/admin-workbench.logic";
export type { AdminWorkbenchVm, RepoAccessAuditRow } from "./lib/admin-workbench/admin-workbench.model";
export { AvlWhereUsedConsoleComponent } from "./lib/avl-where-used-console/avl-where-used-console.component";
export { buildAvlWhereUsedVm } from "./lib/avl-where-used-console/avl-where-used-console.logic";
export type {
  AvlBlastRadiusEntry,
  AvlWhereUsedVm,
} from "./lib/avl-where-used-console/avl-where-used-console.model";
