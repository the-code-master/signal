// Module: web/libs/plm-ops/src/lib/multi-bom-passport/multi-bom-passport.logic.spec.ts — Unit Tests
// Invariants: Verifies eBOM vs mBOM vs sBOM view projection and exact bigint cost/carbon summation.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  buildMultiBomPassportVm,
  filterItemsForBomView,
} from "./multi-bom-passport.logic";

describe("multi-bom-passport.logic (CU-P5-05)", () => {
  it("projects 3 eBOM engineering lines, 4 full mBOM factory lines, and 3 field-replaceable sBOM spares", () => {
    expect(filterItemsForBomView("EBOM")).toHaveLength(3);
    expect(filterItemsForBomView("MBOM")).toHaveLength(4);
    expect(filterItemsForBomView("SBOM")).toHaveLength(3);

    const mbomVm = buildMultiBomPassportVm("MBOM");
    expect(mbomVm.totalCostMicrosUsd).toBe(157_500_000n);
    expect(mbomVm.dppTone).toBe("ok");
  });
});
