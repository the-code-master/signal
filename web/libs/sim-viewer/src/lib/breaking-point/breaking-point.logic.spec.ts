// Module: web/libs/sim-viewer/src/lib/breaking-point/breaking-point.logic.spec.ts — FEAT-102 Breaking-Point Tests
// Invariants: Verifies exact identification of the first component hitting negative SOA headroom under surge.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildBreakingPointVm } from "./breaking-point.logic";

describe("BreakingPoint stress-envelope explorer (FEAT-102)", () => {
  it("identifies D_TVS_CLAMP as the first component hitting avalanche breakdown at 900V surge", () => {
    const nominal = buildBreakingPointVm(358_150n, 800_000_000n);
    expect(nominal.minHeadroomPct).toBeGreaterThan(0);

    const overSurge = buildBreakingPointVm(358_150n, 900_000_000n);
    expect(overSurge.firstFailingEntityId).toBe("D_TVS_CLAMP");
    expect(overSurge.minHeadroomPct).toBeLessThan(0);
    expect(overSurge.summaryTone).toBe("danger");
  });
});
