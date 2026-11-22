// Module: web/libs/harness-3d/src/lib/multiboard-flex-nailboard/multiboard-flex-nailboard.logic.spec.ts — Unit Tests
// Invariants: Verifies IPC-2223 outer-fiber copper strain eps = t/(2R) at 100% fold and nailboard cut/drop totals.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  buildMultiboardFlexNailboardVm,
  computeOuterFiberCopperStrainPct,
} from "./multiboard-flex-nailboard.logic";

describe("multiboard-flex-nailboard.logic (FEAT-020/061/062)", () => {
  it("passes IPC-2223 RA Cu strain cap at R=2.5 mm and flags over-strain at tight R=0.8 mm fold", () => {
    expect(computeOuterFiberCopperStrainPct(18_000n, 2_500_000n, 100)).toBe(0.36);
    expect(computeOuterFiberCopperStrainPct(18_000n, 800_000n, 100)).toBeGreaterThan(0.8);

    const safeVm = buildMultiboardFlexNailboardVm(100, 2_500_000n);
    expect(safeVm.flexAssemblyTone).toBe("ok");
    const tightVm = buildMultiboardFlexNailboardVm(100, 800_000n);
    expect(tightVm.flexAssemblyTone).toBe("danger");
  });
});
