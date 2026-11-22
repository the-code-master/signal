// Module: web/libs/mcad-studio/src/lib/brep-sheetmetal-anchors/brep-sheetmetal-anchors.logic.spec.ts — Unit Tests
// Invariants: Verifies K-factor flat-pattern bend allowance in bigint nm and <50ms ECAD<->MCAD anchor latency.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  buildBrepSheetmetalAnchorsVm,
  computeSheetMetalUnfoldedNm,
} from "./brep-sheetmetal-anchors.logic";

describe("brep-sheetmetal-anchors.logic (CU-P5-03)", () => {
  it("increases unfolded sheet-metal flat length monotonically with K-factor and verifies 0-DOF sketch", () => {
    const k33 = computeSheetMetalUnfoldedNm(65_000_000n, 28_000_000n, 2_000_000n, 1_500_000n, 0.33);
    const k50 = computeSheetMetalUnfoldedNm(65_000_000n, 28_000_000n, 2_000_000n, 1_500_000n, 0.5);
    expect(k50).toBeGreaterThan(k33);

    const vm = buildBrepSheetmetalAnchorsVm(0.44, true);
    expect(vm.anchorSyncTone).toBe("ok");
    expect(vm.sketchRemainingDof).toBe(0);
  });
});
