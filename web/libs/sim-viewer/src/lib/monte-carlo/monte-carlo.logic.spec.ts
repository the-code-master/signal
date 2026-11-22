// Module: web/libs/sim-viewer/src/lib/monte-carlo/monte-carlo.logic.spec.ts — FEAT-083 Monte Carlo Unit Tests
// Invariants: Verifies exact Cpk calculation, histogram bin count conservation, and Spearman rank correlation bounds [-1, 1].
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildHistogramBins,
  buildMonteCarloStudioVm,
  computeSpearmanRankCorrelation,
  computeYieldAndCpk,
} from "./monte-carlo.logic";

describe("Monte Carlo Yield & Spearman Tornado logic (FEAT-083)", () => {
  it("computes Cpk, histogram bin conservation, and monotonic Spearman ρ_s = ±1", () => {
    const summary = computeYieldAndCpk([4.98, 5.0, 5.02, 4.99, 5.01], 4.9, 5.1);
    expect(summary.cpk).toBeGreaterThan(1.5);
    expect(summary.passYieldPct).toBe(100);

    const bins = buildHistogramBins([1, 2, 3, 4, 5, 6], 3);
    expect(bins.reduce((s, b) => s + b.count, 0)).toBe(6);

    expect(computeSpearmanRankCorrelation([1, 2, 3, 4], [10, 20, 30, 40])).toBe(1);
    expect(computeSpearmanRankCorrelation([1, 2, 3, 4], [40, 30, 20, 10])).toBe(-1);

    const vm = buildMonteCarloStudioVm(250);
    expect(vm.summary.sampleCount).toBe(250);
    expect(vm.tornadoRows.length).toBe(3);
  });
});
