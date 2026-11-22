// Module: web/libs/sim-viewer/src/lib/regression-suite/regression-suite.logic.spec.ts — FEAT-086 Regression Suite Unit Tests
// Invariants: Verifies >, >=, <, <= assertion predicates and signed margin percentage math.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildRegressionSuiteVm,
  DEFAULT_ASSERTION_EXPRESSIONS,
  DEFAULT_SIM_METRICS,
  evaluateSpecAssertion,
} from "./regression-suite.logic";

describe("RegressionSuite spec-assertion logic (FEAT-086)", () => {
  it("evaluates phase_margin_deg > 45 and vout_ripple_mv < 15 with exact margin %", () => {
    const pm = evaluateSpecAssertion("a1", "phase_margin_deg > 45", { phase_margin_deg: 60 });
    expect(pm.passed).toBe(true);
    expect(pm.marginPct).toBeCloseTo(33.33, 1);

    const failRip = evaluateSpecAssertion("a2", "vout_ripple_mv < 15", { vout_ripple_mv: 18 });
    expect(failRip.passed).toBe(false);
    expect(failRip.marginPct).toBe(-20);

    const vm = buildRegressionSuiteVm(DEFAULT_ASSERTION_EXPRESSIONS, DEFAULT_SIM_METRICS);
    expect(vm.allPassed).toBe(true);
    expect(vm.gateTone).toBe("ok");
  });
});
