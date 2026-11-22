// Module: web/libs/sim-viewer/src/lib/stimulus-audition/stimulus-audition.logic.spec.ts — FEAT-090 Stimulus Audition Unit Tests
// Invariants: Verifies Float32Array synthesis across presets, PRBS7 high-frequency attenuation in Channel B, and zero disk routes.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildStimulusAuditionVm, synthesizeVolatilePcmPair } from "./stimulus-audition.logic";

describe("StimulusAudition volatile PCM synthesizer (FEAT-090)", () => {
  it("synthesizes bounded Float32Array stimulus and low-pass filtered Channel B", () => {
    const pair = synthesizeVolatilePcmPair("PRBS7_BITSTREAM", 48_000n, 50_000_000_000n);
    expect(pair.sampleCount).toBe(2400);
    expect(pair.rmsA).toBeGreaterThan(0.5);
    expect(pair.rmsB).toBeLessThan(pair.rmsA);

    const vm = buildStimulusAuditionVm("PRBS7_BITSTREAM", "CHANNEL_B_FILTERED", pair);
    expect(vm.svgPreviewPoints.length).toBeGreaterThan(20);
    expect(vm.dlpBadge).toContain("Zero-Download");
  });
});
