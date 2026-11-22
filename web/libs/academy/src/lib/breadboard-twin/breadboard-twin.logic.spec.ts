// Module: web/libs/academy/src/lib/breadboard-twin/breadboard-twin.logic.spec.ts — FEAT-111 Breadboard Twin Tests
// Invariants: Verifies A1..E1 5-hole equivalence, E1 != F1 IC ravine isolation, and VCC<->GND short-circuit detection.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildBreadboardTwinVm,
  holeToCanonical5HoleStrip,
} from "./breadboard-twin.logic";

describe("BreadboardTwin 5-hole strip connectivity & short detector (FEAT-111)", () => {
  it("groups A1..E1 into one strip, isolates F1 across ravine, and catches VCC-GND shorts", () => {
    expect(holeToCanonical5HoleStrip("A1")).toBe("STRIP_AE_1");
    expect(holeToCanonical5HoleStrip("E1")).toBe("STRIP_AE_1");
    expect(holeToCanonical5HoleStrip("F1")).toBe("STRIP_FJ_1");

    const safeVm = buildBreadboardTwinVm(false);
    expect(safeVm.analysis.hasPowerGroundShort).toBe(false);
    expect(safeVm.analysis.tone).toBe("ok");

    const shortedVm = buildBreadboardTwinVm(true);
    expect(shortedVm.analysis.hasPowerGroundShort).toBe(true);
    expect(shortedVm.analysis.tone).toBe("danger");
  });
});
