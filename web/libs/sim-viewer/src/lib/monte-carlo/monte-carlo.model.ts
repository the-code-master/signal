// Module: web/libs/sim-viewer/src/lib/monte-carlo/monte-carlo.model.ts — FEAT-083 Monte Carlo Yield & Spearman Tornado Models
// Invariants: Pure readonly statistical view-models; zero Angular or DOM dependencies.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface HistogramBin {
  readonly binIndex: number;
  readonly lowerBound: number;
  readonly upperBound: number;
  readonly count: number;
  readonly barRectX: number;
  readonly barRectY: number;
  readonly barWidth: number;
  readonly barHeight: number;
}

export interface TornadoSensitivityRow {
  readonly entityId: string;
  readonly paramKey: string;
  readonly spearmanRho: number;
  readonly varianceSharePct: number;
}

export interface MonteCarloSummary {
  readonly sampleCount: number;
  readonly mean: number;
  readonly stdDev: number;
  readonly cpk: number;
  readonly passYieldPct: number;
}

export interface MonteCarloStudioVm {
  readonly summary: MonteCarloSummary;
  readonly cpkBadge: string;
  readonly yieldBadge: string;
  readonly cpkTone: SigStatusTone;
  readonly bins: readonly HistogramBin[];
  readonly tornadoColumns: readonly SigGridColumn[];
  readonly tornadoRows: readonly SigGridRow[];
}
