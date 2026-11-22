// Module: web/libs/sim-viewer/src/lib/jitter-studio/jitter-studio.logic.spec.ts — FEAT-084 SerDes Jitter Unit Tests
// Invariants: Verifies bigint femtosecond Dual-Dirac TJ@1e-15 formula and 112G/224G PAM4 horizontal eye opening UI.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildJitterStudioVm,
  computeDualDiracTjAt1e15Fs,
  presetJitterInput,
} from "./jitter-studio.logic";

describe("SerDes Jitter & PAM4 Bathtub Studio logic (FEAT-084)", () => {
  it("computes Dual-Dirac TJ at 1e-15 BER in bigint femtoseconds", () => {
    expect(computeDualDiracTjAt1e15Fs(1_000n, 2_000n)).toBe(17_883n);
    const vm224 = buildJitterStudioVm(presetJitterInput("224G_PAM4"));
    expect(vm224.tjAt1e15Fs).toBe(5_153n);
    expect(vm224.eyeOpeningUi).toBeGreaterThan(0.4);
    expect(vm224.eyeRows.length).toBe(3);
  });
});
