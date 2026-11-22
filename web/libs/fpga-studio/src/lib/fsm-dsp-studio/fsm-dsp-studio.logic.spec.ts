// Module: web/libs/fpga-studio/src/lib/fsm-dsp-studio/fsm-dsp-studio.logic.spec.ts — Unit Tests
// Invariants: Verifies ~6.02 dB/bit Q(1.n) SQNR gain and Hamming-3 SEU-tolerant SystemVerilog enum synthesis.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildFsmDspStudioVm,
  computeFixedPointSqnrDb,
} from "./fsm-dsp-studio.logic";

describe("fsm-dsp-studio.logic (FEAT-057/060)", () => {
  it("increases FIR SQNR ~24.08 dB from Q(1.10) to Q(1.14) and emits Hamming-3 SV enum", () => {
    const q10 = computeFixedPointSqnrDb(10, 32);
    const q14 = computeFixedPointSqnrDb(14, 32);
    expect(q14 - q10).toBeGreaterThan(24.0);

    const vm = buildFsmDspStudioVm("HAMMING3_SEU_TOLERANT", 14);
    expect(vm.dspFsmTone).toBe("ok");
    expect(vm.rtlPreviewLine).toContain("6'b011101");
  });
});
