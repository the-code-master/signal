// Module: web/libs/schematic/src/lib/pinswap-equations/pinswap-equations.logic.spec.ts — FEAT-008..010 Unit Tests
// Invariants: Verifies spatial sweep RefDes ordering, hierarchical base steps, and safe recursive-descent equation solver.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildPinswapEquationsVm,
  DEFAULT_SPATIAL_ANCHORS,
  evaluateParametricEquation,
  reannotateSpatialEntities,
} from "./pinswap-equations.logic";

describe("Pin/Gate Swap, Spatial Reannotate & Parametric Equations (FEAT-008..010)", () => {
  it("sorts spatial anchors deterministically across sweep directions", () => {
    const lr = reannotateSpatialEntities(DEFAULT_SPATIAL_ANCHORS, "LEFT_TO_RIGHT_TOP_DOWN", 100);
    expect(lr.map((r) => `${r.oldRefdes}->${r.newRefdes}`)).toEqual([
      "R17->R1",
      "R42->R2",
      "R9->R3",
      "U9->U1",
    ]);

    const hier = reannotateSpatialEntities(DEFAULT_SPATIAL_ANCHORS, "HIERARCHICAL_SHEET_BLOCK", 100);
    expect(hier[0].newRefdes).toBe("R101");
    expect(hier[3].newRefdes).toBe("U201");
  });

  it("evaluates parametric feedback divider equations into exact bigint milliohms", () => {
    const res = evaluateParametricEquation("r_bot_mohm * (v_out_mv / v_ref_mv - 1)", {
      r_bot_mohm: 10_000_000,
      v_out_mv: 5_000,
      v_ref_mv: 800,
    });
    expect(res.valid).toBe(true);
    expect(res.siValue).toBe(52_500_000n);

    const bad = evaluateParametricEquation("r_bot_mohm / 0", { r_bot_mohm: 10_000_000 });
    expect(bad.valid).toBe(false);

    const vm = buildPinswapEquationsVm(true, "TOP_DOWN_LEFT_TO_RIGHT", "r_bot_mohm * (v_out_mv / v_ref_mv - 1)");
    expect(vm.swapSavingsBadge).toContain("15350 um Saved");
  });
});
