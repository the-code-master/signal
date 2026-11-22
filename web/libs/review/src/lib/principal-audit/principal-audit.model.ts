// Module: web/libs/review/src/lib/principal-audit/principal-audit.model.ts — FEAT-104 200-Point Red-Team Audit Models
// Invariants: Deterministic severity filtering and 1-click GraphOp auto-fix state.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type AuditSeverityFilter = "ALL" | "CRITICAL" | "HIGH" | "WARN";

export interface RedTeamAuditItem {
  readonly checkCode: number;
  readonly severity: Exclude<AuditSeverityFilter, "ALL">;
  readonly entityId: string;
  readonly diagnosis: string;
  readonly autofixOp: string;
}

export interface PrincipalAuditVm {
  readonly checksEvaluated: number;
  readonly activeFilter: AuditSeverityFilter;
  readonly filters: readonly AuditSeverityFilter[];
  readonly openCount: number;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
