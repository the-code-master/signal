// Module: web/libs/library-studio/src/lib/datasheet-testbench/datasheet-testbench.logic.spec.ts — FEAT-107 Unit Tests
// Invariants: Verifies sub-1% GOLD RMS curve overlay computation and bigint operating condition formatting.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md
import { describe, expect, it } from "vitest";
import {
  buildDatasheetTestbenchVm,
  computeCurveOverlayRmsPct,
} from "./datasheet-testbench.logic";

describe("datasheet-testbench.logic (FEAT-107)", () => {
  it("computes GOLD-tier (< 1.0%) RMS error between quarantined PDF curves and Rust SPICE testbench", () => {
    const vm = buildDatasheetTestbenchVm("TPS546D24ARVFR");
    expect(vm.referenceCurveRmsErrorPct).toBeLessThan(1.0);
    expect(vm.rmsTone).toBe("ok");
    expect(vm.rows).toHaveLength(4);
    expect(vm.rows[0]?.["opPoint"]).toBe("12 V @ 25 °C / 550 kHz");
  });

  it("flags higher RMS discrepancy accurately when SPICE model diverges from reference points", () => {
    const rms = computeCurveOverlayRmsPct([1.0, 1.0], [1.05, 0.95]);
    expect(rms).toBe(5);
  });
});
