// Module: web/libs/power-studio/src/lib/pdn-creepage-ir-hotloop/pdn-creepage-ir-hotloop.logic.spec.ts — Unit Tests
// Invariants: Verifies 3D geodesic creepage slot extension and -4V negative bias Miller shoot-through immunity.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildPdnCreepageIrHotloopVm,
  compute3dGeodesicCreepageNm,
  computeMillerShootThroughMarginUv,
} from "./pdn-creepage-ir-hotloop.logic";

describe("pdn-creepage-ir-hotloop.logic (FEAT-022..025)", () => {
  it("satisfies 8.0 mm IEC 60664-1 3D geodesic creepage with milled slot and +4.3V Miller margin", () => {
    expect(compute3dGeodesicCreepageNm(5_800_000n, false)).toBeLessThan(8_000_000n);
    expect(compute3dGeodesicCreepageNm(5_800_000n, true)).toBe(9_400_000n);
    expect(computeMillerShootThroughMarginUv(true)).toBe(4_300_000n);

    const vm = buildPdnCreepageIrHotloopVm(true, true);
    expect(vm.summaryTone).toBe("ok");
  });
});
