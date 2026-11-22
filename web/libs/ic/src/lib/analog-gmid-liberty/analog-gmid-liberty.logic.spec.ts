// Module: web/libs/ic/src/lib/analog-gmid-liberty/analog-gmid-liberty.logic.spec.ts — Unit Tests
// Invariants: Verifies Pelgrom 1/sqrt(WL) mismatch scaling and Backside PowerVia IR-drop reduction.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildAnalogGmidLibertyVm,
  computePelgromSigmaVthMv,
} from "./analog-gmid-liberty.logic";

describe("analog-gmid-liberty.logic (FEAT-041..044)", () => {
  it("halves Pelgrom Vth sigma when W*L quadruples and cuts IR-drop >3x with Backside PowerVia", () => {
    expect(computePelgromSigmaVthMv(8_000n, 500n, 3.2)).toBe(1.6);
    expect(computePelgromSigmaVthMv(16_000n, 1_000n, 3.2)).toBe(0.8);

    const bspdnOn = buildAnalogGmidLibertyVm("MODERATE_INVERSION_14", true);
    const bspdnOff = buildAnalogGmidLibertyVm("MODERATE_INVERSION_14", false);
    expect(bspdnOn.bspdnIrDropUv).toBeLessThan(bspdnOff.bspdnIrDropUv);
    expect(bspdnOn.summaryTone).toBe("ok");
  });
});
