// Module: web/libs/sim-viewer/src/lib/waveform-canvas/waveform-canvas.logic.spec.ts — Arrow Waveform & Cursor Unit Tests
// Invariants: Verifies <=4096 bucket cap enforcement (F-069), binary BigInt64 roundtrip, and Δt / 1/Δt / slew rate math.
// Spec: architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import { describe, expect, it } from "vitest";
import {
  computeDualCursorReadout,
  createDefaultBuckWaveformBytes,
  encodeArrowWaveformFrame,
  parseDecimatedArrowFrame,
} from "./waveform-canvas.logic";

describe("Arrow IPC decimated waveform parser & dual-cursor math", () => {
  it("parses binary Arrow frames and rejects >4096 display buckets", () => {
    const validBytes = createDefaultBuckWaveformBytes();
    const parsed = parseDecimatedArrowFrame(validBytes);
    expect(parsed.valid).toBe(true);
    expect(parsed.bucketCount).toBe(64);
    expect(parsed.svgPolylinePoints.length).toBeGreaterThan(20);

    const oversizedHeader = new Uint8Array(8);
    oversizedHeader.set([0x53, 0x47, 0x41, 0x57], 0);
    new DataView(oversizedHeader.buffer).setUint32(4, 5000, true);
    const rejected = parseDecimatedArrowFrame(oversizedHeader);
    expect(rejected.valid).toBe(false);
    expect(rejected.errorReason).toContain("ARROW_BUCKET_CAP_EXCEEDED");
  });

  it("computes exact bigint Δt, frequency 1/Δt, and dV/dt slew rate", () => {
    const bytes = encodeArrowWaveformFrame([
      { timePs: 100_000n, minUv: 0n, maxUv: 10_000_000n, sampleUv: 0n },
      { timePs: 600_000n, minUv: 0n, maxUv: 800_000_000n, sampleUv: 800_000_000n },
    ]);
    const parsed = parseDecimatedArrowFrame(bytes);
    const readout = computeDualCursorReadout(parsed.buckets, 0, 1);
    expect(readout.deltaTimePs).toBe(500_000n);
    expect(readout.frequencyHz).toBe(2_000_000n); // 2 MHz
    expect(readout.slewRateText).toBe("1600 V/ns");
  });
});
