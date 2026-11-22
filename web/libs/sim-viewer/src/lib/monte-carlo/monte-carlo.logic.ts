// Module: web/libs/sim-viewer/src/lib/monte-carlo/monte-carlo.logic.ts — Pure Yield Cpk, Histogram & Spearman Rank Tornado Math
// Invariants: Deterministic Spearman rank correlation and process capability Cpk calculation without external stats libraries.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  HistogramBin,
  MonteCarloStudioVm,
  MonteCarloSummary,
  TornadoSensitivityRow,
} from "./monte-carlo.model";

const TORNADO_COLUMNS: readonly SigGridColumn[] = [
  { key: "entityParam", header: "Component Parameter", mono: true },
  { key: "spearman", header: "Spearman ρ_s", mono: true },
  { key: "variance", header: "Variance Share", mono: true },
];

export function computeYieldAndCpk(
  samples: readonly number[],
  lsl: number,
  usl: number,
): MonteCarloSummary {
  const n = samples.length;
  if (n === 0) {
    return { sampleCount: 0, mean: 0, stdDev: 0, cpk: 0, passYieldPct: 0 };
  }
  const mean = samples.reduce((a, b) => a + b, 0) / n;
  const variance =
    n > 1 ? samples.reduce((acc, x) => acc + (x - mean) * (x - mean), 0) / (n - 1) : 0;
  const stdDev = Math.sqrt(variance);
  const cpu = stdDev > 1e-12 ? (usl - mean) / (3 * stdDev) : 9.99;
  const cpl = stdDev > 1e-12 ? (mean - lsl) / (3 * stdDev) : 9.99;
  const cpk = Number(Math.min(cpu, cpl).toFixed(3));
  const passCount = samples.filter((x) => x >= lsl && x <= usl).length;
  const passYieldPct = Number(((passCount / n) * 100).toFixed(2));
  return {
    sampleCount: n,
    mean: Number(mean.toFixed(4)),
    stdDev: Number(stdDev.toFixed(4)),
    cpk,
    passYieldPct,
  };
}

export function buildHistogramBins(samples: readonly number[], binCount = 12): readonly HistogramBin[] {
  if (samples.length === 0 || binCount <= 0) return [];
  const minVal = Math.min(...samples);
  const maxVal = Math.max(...samples);
  const span = maxVal > minVal ? maxVal - minVal : 1;
  const step = span / binCount;
  const counts = new Array<number>(binCount).fill(0);
  for (const s of samples) {
    const rawIdx = Math.floor((s - minVal) / step);
    const idx = Math.max(0, Math.min(binCount - 1, rawIdx));
    counts[idx] += 1;
  }
  const peakCount = Math.max(...counts, 1);
  const plotW = 760;
  const barSlotW = plotW / binCount;
  return counts.map((count, i) => {
    const barHeight = Math.round((count / peakCount) * 124);
    return {
      binIndex: i,
      lowerBound: Number((minVal + i * step).toFixed(3)),
      upperBound: Number((minVal + (i + 1) * step).toFixed(3)),
      count,
      barRectX: Math.round(20 + i * barSlotW),
      barRectY: 144 - barHeight,
      barWidth: Math.max(8, Math.round(barSlotW - 6)),
      barHeight,
    };
  });
}

function rankVector(values: readonly number[]): readonly number[] {
  const indexed = values.map((v, idx) => ({ v, idx })).sort((a, b) => a.v - b.v);
  const ranks = new Array<number>(values.length);
  indexed.forEach((entry, rankIdx) => {
    ranks[entry.idx] = rankIdx + 1;
  });
  return ranks;
}

export function computeSpearmanRankCorrelation(
  x: readonly number[],
  y: readonly number[],
): number {
  const n = Math.min(x.length, y.length);
  if (n < 2) return 0;
  const rx = rankVector(x.slice(0, n));
  const ry = rankVector(y.slice(0, n));
  const meanR = (n + 1) / 2;
  let num = 0;
  let denX = 0;
  let denY = 0;
  for (let i = 0; i < n; i += 1) {
    const dx = rx[i] - meanR;
    const dy = ry[i] - meanR;
    num += dx * dy;
    denX += dx * dx;
    denY += dy * dy;
  }
  const den = Math.sqrt(denX * denY);
  return den > 1e-12 ? Number((num / den).toFixed(4)) : 0;
}

export function buildDefaultMonteCarloSamples(runCount: number): readonly number[] {
  const samples: number[] = [];
  for (let i = 0; i < runCount; i += 1) {
    // Deterministic Halton-like quasi-normal transform centered at 5.002 V (LSL=4.90V, USL=5.10V)
    const u1 = ((i * 73 + 19) % 251 + 1) / 252;
    const u2 = ((i * 137 + 43) % 257 + 1) / 258;
    const z = Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
    samples.push(5.002 + z * 0.021);
  }
  return samples;
}

export function buildMonteCarloStudioVm(runCount: number): MonteCarloStudioVm {
  const samples = buildDefaultMonteCarloSamples(runCount);
  const summary = computeYieldAndCpk(samples, 4.9, 5.1);
  const bins = buildHistogramBins(samples, 12);

  const rTopVec = samples.map((_, i) => 52_500 + ((i * 31) % 41 - 20) * 25);
  const vRefVec = samples.map((s, i) => 0.8 + (s - 5.0) * 0.14 + ((i % 5) - 2) * 0.0002);
  const l1Vec = samples.map((s, i) => 4.7 - (s - 5.0) * 0.08 + ((i % 7) - 3) * 0.004);

  const rawBars: TornadoSensitivityRow[] = [
    { entityId: "U1_VREF", paramKey: "v_bg_uv", spearmanRho: computeSpearmanRankCorrelation(vRefVec, samples), varianceSharePct: 68.4 },
    { entityId: "L1_PHASE", paramKey: "inductance_nh", spearmanRho: computeSpearmanRankCorrelation(l1Vec, samples), varianceSharePct: 21.9 },
    { entityId: "R_FB_TOP", paramKey: "resistance_mohm", spearmanRho: computeSpearmanRankCorrelation(rTopVec, samples), varianceSharePct: 9.7 },
  ].sort((a, b) => Math.abs(b.spearmanRho) - Math.abs(a.spearmanRho));

  const tornadoRows: SigGridRow[] = rawBars.map((b, idx) => ({
    id: `t-${idx}`,
    entityParam: `${b.entityId}.${b.paramKey}`,
    spearman: `${b.spearmanRho >= 0 ? "+" : ""}${b.spearmanRho.toFixed(3)}`,
    variance: `${b.varianceSharePct.toFixed(1)}%`,
  }));

  const cpkTone: SigStatusTone = summary.cpk >= 1.33 ? "ok" : summary.cpk >= 1.0 ? "warn" : "danger";
  return {
    summary,
    cpkBadge: `Cpk = ${summary.cpk.toFixed(2)} (N=${summary.sampleCount})`,
    yieldBadge: `Yield: ${summary.passYieldPct.toFixed(2)}% [4.90V..5.10V]`,
    cpkTone,
    bins,
    tornadoColumns: TORNADO_COLUMNS,
    tornadoRows,
  };
}
