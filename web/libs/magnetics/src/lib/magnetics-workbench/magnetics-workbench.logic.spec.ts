// Module: web/libs/magnetics/src/lib/magnetics-workbench/magnetics-workbench.logic.spec.ts — FEAT-021 Unit Tests
// Invariants: Verifies B-H saturation margin, air-gap flux reduction, and interleaved planar Dowell AC loss savings.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  MAGNETICS_WORKBENCH_DESCRIPTOR,
  buildMagneticsWorkbenchVm,
  computePeakFluxUt,
} from "./magnetics-workbench.logic";

describe("magnetics-workbench.logic (FEAT-021)", () => {
  it("reduces peak flux density as air-gap increases and lowers AC copper loss with P-S-P-S interleaving", () => {
    expect(computePeakFluxUt(400_000n)).toBeLessThan(computePeakFluxUt(150_000n));
    const planar = buildMagneticsWorkbenchVm(
      MAGNETICS_WORKBENCH_DESCRIPTOR,
      "PLANAR_HEAVY_CU_INTERLEAVED",
      400_000n,
    );
    const bobbin = buildMagneticsWorkbenchVm(
      MAGNETICS_WORKBENCH_DESCRIPTOR,
      "LITZ_BOBBIN_E_CORE",
      400_000n,
    );
    expect(planar.saturationTone).toBe("ok");
    expect(planar.dowellAcCopperLossMw).toBeLessThan(bobbin.dowellAcCopperLossMw);
  });
});
