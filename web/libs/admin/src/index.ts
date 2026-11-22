// Module: web/libs/admin/src/index.ts — Public Barrel for @signal/admin
// Invariants: Sole public entrypoint for lazy route loading, Access Review, FinOps, Policy Studio, Portfolio & Compliance.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./admin.routes";
export { AdminWorkbenchComponent } from "./lib/admin-workbench/admin-workbench.component";
export { buildAdminWorkbenchVm } from "./lib/admin-workbench/admin-workbench.logic";
export type { AdminWorkbenchVm, RepoAccessAuditRow } from "./lib/admin-workbench/admin-workbench.model";
export { AvlWhereUsedConsoleComponent } from "./lib/avl-where-used-console/avl-where-used-console.component";
export { buildAvlWhereUsedVm } from "./lib/avl-where-used-console/avl-where-used-console.logic";
export type { AvlBlastRadiusEntry, AvlWhereUsedVm } from "./lib/avl-where-used-console/avl-where-used-console.model";
export { FinOpsDashboardComponent } from "./lib/finops-dashboard/finops-dashboard.component";
export { buildFinOpsDashboardVm, evaluateFinOpsRollup } from "./lib/finops-dashboard/finops-dashboard.logic";
export type { BudgetAlertState, FinOpsDashboardVm, FinOpsRollupSummary } from "./lib/finops-dashboard/finops-dashboard.model";
export { PolicyStudioComponent } from "./lib/policy-studio/policy-studio.component";
export { buildPolicyStudioVm, isWasmPolicyCompliant } from "./lib/policy-studio/policy-studio.logic";
export type { PolicyPackDomain, PolicyStudioVm, WasmPolicyPackSpec } from "./lib/policy-studio/policy-studio.model";
export { PortfolioGraphComponent } from "./lib/portfolio-graph/portfolio-graph.component";
export { analyzePortfolioBlastRadius, buildPortfolioGraphVm } from "./lib/portfolio-graph/portfolio-graph.logic";
export type { BlastTriggerArtifact, PortfolioBlastSummary, PortfolioGraphVm } from "./lib/portfolio-graph/portfolio-graph.model";
export { ComplianceCenterComponent } from "./lib/compliance-center/compliance-center.component";
export { buildComplianceCenterVm, evaluateGdprSaltShred } from "./lib/compliance-center/compliance-center.logic";
export type {
  ComplianceCenterVm,
  ComplianceFrameworkId,
  GdprCryptoShredVerification,
} from "./lib/compliance-center/compliance-center.model";
