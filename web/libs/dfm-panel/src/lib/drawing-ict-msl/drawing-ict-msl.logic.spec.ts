// Module: web/libs/dfm-panel/src/lib/drawing-ict-msl/drawing-ict-msl.logic.spec.ts — Unit Tests
// Invariants: Verifies IEEE 1149.1 JTAG boundary-scan coverage boost and J-STD-020 MSL-5A 24h bake requirement.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildDrawingIctMslVm,
  computeCombinedTestCoveragePct,
} from "./drawing-ict-msl.logic";

describe("drawing-ict-msl.logic (FEAT-067/069/080)", () => {
  it("boosts ICT test coverage to 96.8% with JTAG and flags 36h exposure on MSL-5A (24h cap) for bake", () => {
    expect(computeCombinedTestCoveragePct(true)).toBe(96.8);
    expect(buildDrawingIctMslVm("MSL_3_168H", true, 36).summaryTone).toBe("ok");
    expect(buildDrawingIctMslVm("MSL_5A_24H", true, 36).summaryTone).toBe("warn");
  });
});
