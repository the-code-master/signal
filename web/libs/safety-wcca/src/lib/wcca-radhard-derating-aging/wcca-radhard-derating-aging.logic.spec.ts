// Module: web/libs/safety-wcca/src/lib/wcca-radhard-derating-aging/wcca-radhard-derating-aging.logic.spec.ts — Unit Tests
// Invariants: Verifies 15-year t^0.25 NBTI threshold shift and ECSS 60% vs Automotive 80% stress derating.
// Spec: architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import { describe, expect, it } from "vitest";
import {
  buildWccaRadhardDeratingAgingVm,
  computeNbtiShiftUv,
} from "./wcca-radhard-derating-aging.logic";

describe("wcca-radhard-derating-aging.logic (FEAT-037/038/040 & I-05)", () => {
  it("models monotonically increasing 5..15 yr NBTI shift and flags 67% stress under strict ECSS 60% space cap", () => {
    expect(computeNbtiShiftUv(15)).toBeGreaterThan(computeNbtiShiftUv(5));
    expect(buildWccaRadhardDeratingAgingVm("ENTERPRISE_AUTOMOTIVE", 15).summaryTone).toBe("ok");
    expect(buildWccaRadhardDeratingAgingVm("ECSS_SPACE_CLASS_1", 15).summaryTone).toBe("warn");
  });
});
