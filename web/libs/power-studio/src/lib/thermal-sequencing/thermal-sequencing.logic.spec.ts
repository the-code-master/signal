// Module: web/libs/power-studio/src/lib/thermal-sequencing/thermal-sequencing.logic.spec.ts — Unit Tests
// Invariants: Verifies cold-plate vs natural-convection Tj limits and PGOOD sequencing interlock checks.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  DEFAULT_SEQ_RAILS,
  buildThermalSequencingVm,
  computePeakTjMkelvin,
  verifyMonotonicSequencing,
} from "./thermal-sequencing.logic";

describe("thermal-sequencing.logic (FEAT-029/030)", () => {
  it("flags thermal overload on natural convection at 140W and passes on WEG cold-plate with monotonic PGOOD", () => {
    expect(computePeakTjMkelvin("NATURAL_CONVECTION")).toBeGreaterThan(448_150n);
    expect(computePeakTjMkelvin("WEG_LIQUID_COLD_PLATE")).toBeLessThan(400_000n);
    expect(verifyMonotonicSequencing(DEFAULT_SEQ_RAILS)).toBe(true);

    const vm = buildThermalSequencingVm("WEG_LIQUID_COLD_PLATE");
    expect(vm.summaryTone).toBe("ok");
  });
});
