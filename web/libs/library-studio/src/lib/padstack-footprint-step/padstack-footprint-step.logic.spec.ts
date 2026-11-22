// Module: web/libs/library-studio/src/lib/padstack-footprint-step/padstack-footprint-step.logic.spec.ts — FEAT-076 & FEAT-071 Tests
// Invariants: Verifies IPC-7525 >=0.66 stencil area ratio, backdrill stub reduction, and MOST > NOMINAL > LEAST courtyards.
// Spec: architecture/18-universal-component-and-datasheet-repository.md
import { describe, expect, it } from "vitest";
import {
  buildPadstackFootprintStepVm,
  computeIpc7351CourtyardSpanNm,
} from "./padstack-footprint-step.logic";

describe("PadstackSculptor & IPC-7351C / 3D STEP Verifier (FEAT-076, FEAT-071)", () => {
  it("computes monotonically ordered MOST > NOMINAL > LEAST courtyards and >=85% STEP overlap", () => {
    const most = computeIpc7351CourtyardSpanNm(5_000_000n, 5_000_000n, "MOST");
    const nom = computeIpc7351CourtyardSpanNm(5_000_000n, 5_000_000n, "NOMINAL");
    const least = computeIpc7351CourtyardSpanNm(5_000_000n, 5_000_000n, "LEAST");
    expect(most.courtyardWidthNm > nom.courtyardWidthNm).toBe(true);
    expect(nom.courtyardWidthNm > least.courtyardWidthNm).toBe(true);
    expect(least.stepOverlapRatio).toBeGreaterThanOrEqual(0.85);

    const vmBackdrill = buildPadstackFootprintStepVm("NOMINAL", true);
    const vmNoBackdrill = buildPadstackFootprintStepVm("NOMINAL", false);
    expect(vmBackdrill.padMetrics.ipc7525Passed).toBe(true);
    expect(vmBackdrill.padMetrics.backdrillStubNm < vmNoBackdrill.padMetrics.backdrillStubNm).toBe(true);
  });
});
