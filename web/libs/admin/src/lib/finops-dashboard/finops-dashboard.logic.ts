// Module: web/libs/admin/src/lib/finops-dashboard/finops-dashboard.logic.ts — Compute Metering & Cap Logic (FEAT-116)
// Invariants: Pure deterministic functions <=250 LOC; evaluates 80% soft warn & 100% hard budget cap in bigint _micros_usd.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  BudgetAlertState,
  FinOpsDashboardVm,
  FinOpsRollupSummary,
  TeamFinOpsLedgerItem,
} from "./finops-dashboard.model";

export const SEEDED_TEAM_FINOPS: readonly TeamFinOpsLedgerItem[] = [
  {
    teamSlug: "power-electronics-ee",
    cpuCoreMsConsumed: 184_500_000n,
    gpuMsConsumed: 42_800_000n,
    aiTokensConsumed: 6_450_000n,
    accruedSpendMicrosUsd: 1_850_000_000n,
    hardCapMicrosUsd: 2_500_000_000n,
    medianCycleTimeHours: 14.2,
  },
  {
    teamSlug: "rf-serdes-3dem",
    cpuCoreMsConsumed: 96_200_000n,
    gpuMsConsumed: 88_400_000n,
    aiTokensConsumed: 3_120_000n,
    accruedSpendMicrosUsd: 2_150_000_000n,
    hardCapMicrosUsd: 2_500_000_000n,
    medianCycleTimeHours: 19.6,
  },
];

export const FINOPS_GRID_COLUMNS: readonly SigGridColumn[] = [
  { key: "team", header: "Engineering Team", mono: true },
  { key: "compute", header: "CPU-Core-ms / GPU-ms", mono: true },
  { key: "tokens", header: "AI Tokens", mono: true },
  { key: "spend", header: "Spend / Hard Cap (USD)", mono: true },
  { key: "velocity", header: "Median Cycle Time", mono: true },
];

export function formatMicrosUsd(micros: bigint): string {
  const dollars = micros / 1_000_000n;
  const cents = ((micros % 1_000_000n) / 10_000n).toString().padStart(2, "0");
  return `$${dollars.toString()}.${cents}`;
}

export function evaluateFinOpsRollup(items: readonly TeamFinOpsLedgerItem[]): FinOpsRollupSummary {
  const totalCpuCoreMs = items.reduce((a, b) => a + b.cpuCoreMsConsumed, 0n);
  const totalGpuMs = items.reduce((a, b) => a + b.gpuMsConsumed, 0n);
  const totalAiTokens = items.reduce((a, b) => a + b.aiTokensConsumed, 0n);
  const totalSpendMicrosUsd = items.reduce((a, b) => a + b.accruedSpendMicrosUsd, 0n);
  const totalHardCapMicrosUsd = items.reduce((a, b) => a + b.hardCapMicrosUsd, 0n);

  const basisPoints =
    totalHardCapMicrosUsd > 0n ? Number((totalSpendMicrosUsd * 10_000n) / totalHardCapMicrosUsd) : 0;
  const utilizationPct = basisPoints / 100;

  let alertState: BudgetAlertState = "WITHIN_BUDGET";
  if (totalSpendMicrosUsd >= totalHardCapMicrosUsd) {
    alertState = "HARD_CAP_100_PCT_TRIPPED";
  } else if (totalSpendMicrosUsd * 100n >= totalHardCapMicrosUsd * 80n) {
    alertState = "SOFT_WARN_80_PCT";
  }

  return {
    totalCpuCoreMs,
    totalGpuMs,
    totalAiTokens,
    totalSpendMicrosUsd,
    totalHardCapMicrosUsd,
    utilizationPct,
    alertState,
    hardBudgetCapTripped: alertState === "HARD_CAP_100_PCT_TRIPPED",
  };
}

export function buildFinOpsDashboardVm(
  items: readonly TeamFinOpsLedgerItem[],
  burstGpuMicrosUsd: bigint,
): FinOpsDashboardVm {
  const adjusted: TeamFinOpsLedgerItem[] = items.map((it, idx) =>
    idx === 1
      ? {
          ...it,
          gpuMsConsumed: it.gpuMsConsumed + burstGpuMicrosUsd / 10n,
          accruedSpendMicrosUsd: it.accruedSpendMicrosUsd + burstGpuMicrosUsd,
        }
      : it,
  );
  const rollup = evaluateFinOpsRollup(adjusted);
  const alertTone: SigStatusTone =
    rollup.alertState === "HARD_CAP_100_PCT_TRIPPED"
      ? "danger"
      : rollup.alertState === "SOFT_WARN_80_PCT"
        ? "warn"
        : "ok";

  const ledgerRows: SigGridRow[] = adjusted.map((it) => ({
    id: it.teamSlug,
    team: it.teamSlug,
    compute: `${it.cpuCoreMsConsumed.toString()} ms / ${it.gpuMsConsumed.toString()} ms`,
    tokens: it.aiTokensConsumed.toString(),
    spend: `${formatMicrosUsd(it.accruedSpendMicrosUsd)} / ${formatMicrosUsd(it.hardCapMicrosUsd)}`,
    velocity: `${it.medianCycleTimeHours} h/ECO`,
  }));

  return {
    rollup,
    alertTone,
    alertBadge: `${rollup.alertState} (${rollup.utilizationPct}% • ${formatMicrosUsd(rollup.totalSpendMicrosUsd)})`,
    ledgerColumns: FINOPS_GRID_COLUMNS,
    ledgerRows,
  };
}
