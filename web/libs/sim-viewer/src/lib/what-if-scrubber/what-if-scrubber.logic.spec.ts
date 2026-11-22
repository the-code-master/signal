// Module: web/libs/sim-viewer/src/lib/what-if-scrubber/what-if-scrubber.logic.spec.ts — FEAT-085 What-If Scrubber Unit Tests
// Invariants: Verifies 5 ghost family curves, inverse inductance-ripple gradient, and <15ms latency tone.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildWhatIfScrubberVm } from "./what-if-scrubber.logic";

describe("WhatIfScrubber Saltation-Adjoint logic (FEAT-085)", () => {
  it("generates 5 ghost family curves with <15ms telemetry and physical dV/dL slope", () => {
    const vm = buildWhatIfScrubberVm(4_700n, 10, 6_400_000_000n);
    expect(vm.ghostCurves.length).toBe(5);
    expect(vm.activeParamNh).toBe(5_170n);
    expect(vm.latencyTone).toBe("ok");
    // Higher inductance -> lower ripple peak:
    expect(vm.ghostCurves[4].peakRippleMv).toBeLessThan(vm.ghostCurves[0].peakRippleMv);
  });
});
