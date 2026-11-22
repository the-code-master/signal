// Module: web/libs/sim-viewer/src/lib/virtual-bench/virtual-bench.logic.spec.ts — FEAT-081 Virtual Bench Unit Tests
// Invariants: Verifies ENOB=(SINAD-1.76)/6.02, conformal Smith Γ=0 -> 50+j0 Ω mapping, and 4 instrument deck modes.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildVirtualBenchVm,
  computeEnobFromSinadDb,
  gammaToSmithImpedance,
} from "./virtual-bench.logic";

describe("VirtualBenchDeck logic (FEAT-081)", () => {
  it("computes IEEE 1241 ENOB from SINAD dB and exact Smith Z from reflection Γ", () => {
    expect(computeEnobFromSinadDb(74.0)).toBe(12);
    const matched = gammaToSmithImpedance(1_000_000_000n, 0, 0, 50);
    expect(matched.zRealOhm).toBe(50);
    expect(matched.zImagOhm).toBe(0);
    expect(matched.vswr).toBe(1);
  });

  it("builds distinct view-models for DSO, 32-Ch Logic, FFT, and Smith VNA", () => {
    expect(buildVirtualBenchVm("DSO_4CH", 1_000n, 400_000_000n).rows.length).toBe(4);
    expect(buildVirtualBenchVm("LOGIC_32CH", 1_000n, 400_000_000n).rows.length).toBe(3);
    expect(buildVirtualBenchVm("FFT_SPECTRUM", 10_000n, 400_000_000n).metrics[3].label).toContain("12 bits");
    expect(buildVirtualBenchVm("VNA_2PORT_SMITH", 10_000n, 400_000_000n).rows.length).toBe(3);
  });
});
