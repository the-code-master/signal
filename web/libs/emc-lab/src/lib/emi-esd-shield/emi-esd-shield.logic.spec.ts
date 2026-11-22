// Module: web/libs/emc-lab/src/lib/emi-esd-shield/emi-esd-shield.logic.spec.ts — Unit Tests
// Invariants: Verifies Middlebrook R-C damping stabilization, IEC 61000-4-2 TVS clamping, and TE101 cavity resonance.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildEmiEsdShieldVm,
  computeCavityTe101Hz,
  computeTvsClampedPeakUv,
} from "./emi-esd-shield.logic";

describe("emi-esd-shield.logic (FEAT-032/033/034)", () => {
  it("computes ~1.56 GHz TE101 enclosure mode, clamps 15kV ESD < 28V, and enforces >=6 dB Middlebrook margin", () => {
    const f101 = computeCavityTe101Hz(120_000_000n, 160_000_000n);
    expect(f101).toBeGreaterThan(1_500_000_000n);
    expect(f101).toBeLessThan(1_600_000_000n);
    expect(computeTvsClampedPeakUv("IEC_15KV_AIR")).toBeLessThan(28_000_000n);

    expect(buildEmiEsdShieldVm("IEC_8KV_CONTACT", false).summaryTone).toBe("warn");
    expect(buildEmiEsdShieldVm("IEC_8KV_CONTACT", true).summaryTone).toBe("ok");
  });
});
