// Module: web/apps/signal-web/src/app/selection-query/selection-query.logic.spec.ts — FEAT-099 Selection Query Unit Tests
// Invariants: Verifies `type:cap AND voltage_rating < 2*net_max_v` derating filter and syntax error rejection.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { evaluateSelectionQuery } from "./selection-query.logic";

describe("SelectionQueryToolbar predicate evaluator (FEAT-099)", () => {
  it("matches derated capacitors via `type:cap AND voltage_rating < 2*net_max_v`", () => {
    const res = evaluateSelectionQuery("type:cap AND voltage_rating < 2*net_max_v");
    expect(res.valid).toBe(true);
    // C_IN_1 (25 < 32) and C_LINK_HV (1200 < 1600) match; C_GATE_DEC (50 < 36) does not!
    expect(res.matchedEntityIds).toEqual(["C_IN_1", "C_LINK_HV"]);

    const invalid = evaluateSelectionQuery("unknown_field < 5");
    expect(invalid.valid).toBe(false);
  });
});
