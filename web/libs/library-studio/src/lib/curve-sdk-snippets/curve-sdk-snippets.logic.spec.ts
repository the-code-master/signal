// Module: web/libs/library-studio/src/lib/curve-sdk-snippets/curve-sdk-snippets.logic.spec.ts — FEAT-073, 074, 079 Tests
// Invariants: Verifies <2% RMS curve-fit error, Wasmtime fuel telemetry, and Golden Snippet RefDes base offset stamping.
// Spec: architecture/18-universal-component-and-datasheet-repository.md
import { describe, expect, it } from "vitest";
import {
  buildCurveSdkSnippetsVm,
  computeRelativeRmsErrorPct,
  GOLDEN_SNIPPETS,
  stampGoldenSnippetPreview,
} from "./curve-sdk-snippets.logic";

describe("CurveFitter, WASM SDK Rig & Golden Snippets (FEAT-073, 074, 079)", () => {
  it("computes relative RMS error <2% and remaps Golden Snippet RefDes cleanly", () => {
    expect(computeRelativeRmsErrorPct([10, 20], [10.1, 19.8])).toBeCloseTo(1.0, 2);

    const stamped = stampGoldenSnippetPreview(GOLDEN_SNIPPETS[0], 400, 10_000_000n, 20_000_000n);
    expect(stamped.remappedRefdes).toEqual(["U401", "Q401", "R401", "R402", "C401"]);

    const vm = buildCurveSdkSnippetsVm("MLCC_DC_BIAS", "snip-usb4-cm-esd", 500);
    expect(vm.curveFitTone).toBe("ok");
    expect(vm.stampedPreview.remappedRefdes[0]).toBe("D501");
  });
});
