// Module: web/libs/academy/src/lib/lab-runner/lab-runner.logic.spec.ts — Unit Tests (FEAT-112)
// Invariants: Verifies fault-injected rubric scoring, 100% compensation upgrade & signed LTI 1.3 receipt generation.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import { describe, expect, it } from "vitest";
import { buildLabRunnerVm } from "./lab-runner.logic";

describe("lab-runner.logic", () => {
  it("flags 85C thermal compensation failure at 75% and reaches 100% with signed LTI 1.3 passback after fix", () => {
    const initialVm = buildLabRunnerVm("CANVAS_LTI13", "THERMAL_DERATE_85C", false, 9007199254740993n);
    expect(initialVm.result.scorePct).toBe(75);
    expect(initialVm.rubricRows[3]?.["verdict"]).toBe("FAIL");

    const fixedVm = buildLabRunnerVm("MOODLE_LTI13", "NOMINAL", true, 9007199254740993n);
    expect(fixedVm.result.scorePct).toBe(100);
    expect(fixedVm.result.lti13GradePassbackReceipt).toContain("lti13.ags.moodle_lti13");
    expect(fixedVm.result.graderMlDsa65Signature.startsWith("ml-dsa-65:")).toBe(true);
  });
});
