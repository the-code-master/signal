// Module: web/libs/admin/src/lib/finops-dashboard/finops-dashboard.model.ts — Enterprise FinOps Models (FEAT-116)
// Invariants: Pure readonly TypeScript types; 64-bit _ms, token counts, and _micros_usd budgets use bigint ONLY.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type BudgetAlertState = "WITHIN_BUDGET" | "SOFT_WARN_80_PCT" | "HARD_CAP_100_PCT_TRIPPED";

export interface TeamFinOpsLedgerItem {
  readonly teamSlug: string;
  readonly cpuCoreMsConsumed: bigint;
  readonly gpuMsConsumed: bigint;
  readonly aiTokensConsumed: bigint;
  readonly accruedSpendMicrosUsd: bigint;
  readonly hardCapMicrosUsd: bigint;
  readonly medianCycleTimeHours: number;
}

export interface FinOpsRollupSummary {
  readonly totalCpuCoreMs: bigint;
  readonly totalGpuMs: bigint;
  readonly totalAiTokens: bigint;
  readonly totalSpendMicrosUsd: bigint;
  readonly totalHardCapMicrosUsd: bigint;
  readonly utilizationPct: number;
  readonly alertState: BudgetAlertState;
  readonly hardBudgetCapTripped: boolean;
}

export interface FinOpsDashboardVm {
  readonly rollup: FinOpsRollupSummary;
  readonly alertTone: SigStatusTone;
  readonly alertBadge: string;
  readonly ledgerColumns: readonly SigGridColumn[];
  readonly ledgerRows: readonly SigGridRow[];
}
