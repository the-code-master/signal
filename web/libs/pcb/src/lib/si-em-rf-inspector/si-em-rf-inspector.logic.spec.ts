// Module: web/libs/pcb/src/lib/si-em-rf-inspector/si-em-rf-inspector.logic.spec.ts — SI/EM/RF Unit Tests
// Invariants: Verifies 10° glass-weave skew mitigation, back-drill resonance shift above 56 GHz, and RF synthesis.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildSiEmRfInspectorVm,
  computeFiberWeaveSkewFs,
  computeViaStubQuarterWaveHz,
} from "./si-em-rf-inspector.logic";

describe("si-em-rf-inspector.logic (FEAT-011/013/016/019/087/089)", () => {
  it("reduces fiber-weave skew >10x with 10° rotation and pushes back-drilled stub notch >100 GHz", () => {
    expect(computeFiberWeaveSkewFs(48_000_000n, false)).toBe(8_880n);
    expect(computeFiberWeaveSkewFs(48_000_000n, true)).toBe(672n);
    expect(computeViaStubQuarterWaveHz(1_850_000n)).toBeLessThan(25_000_000_000n);
    expect(computeViaStubQuarterWaveHz(250_000n)).toBeGreaterThan(150_000_000_000n);

    const vm = buildSiEmRfInspectorVm(true, true, "WILKINSON_2WAY_28GHZ", 90);
    expect(vm.siGateTone).toBe("ok");
  });
});
