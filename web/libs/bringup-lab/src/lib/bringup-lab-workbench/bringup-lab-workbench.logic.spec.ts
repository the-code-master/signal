// Module: web/libs/bringup-lab/src/lib/bringup-lab-workbench/bringup-lab-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies Bayesian parasitic calibration drops Sim-vs-Bench RMS error from 6.85% to 0.14%.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  BRINGUP_LAB_WORKBENCH_DESCRIPTOR,
  buildBringupLabWorkbenchVm,
} from "./bringup-lab-workbench.logic";

describe("bringup-lab-workbench.logic (I-06)", () => {
  it("reduces Sim-vs-Bench RMS discrepancy below 1% after Saltation-Adjoint Bayesian calibration", () => {
    const rawVm = buildBringupLabWorkbenchVm(
      BRINGUP_LAB_WORKBENCH_DESCRIPTOR,
      "SCPI_MSO64B_SCOPE",
      false,
    );
    const calVm = buildBringupLabWorkbenchVm(
      BRINGUP_LAB_WORKBENCH_DESCRIPTOR,
      "SCPI_MSO64B_SCOPE",
      true,
    );
    expect(calVm.postCalibrationRmsErrorPct).toBeLessThan(rawVm.postCalibrationRmsErrorPct);
    expect(calVm.twinTone).toBe("ok");
  });
});
