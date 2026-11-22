// Module: web/libs/schematic/src/lib/beautify-bus/beautify-bus.logic.spec.ts — FEAT-001 & FEAT-002 Unit Tests
// Invariants: Verifies DATA[31:0] MSB->LSB expansion, AXI4_LITE protocol expansion, >128 bit cap, and crossing delta math.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildBeautifyBusVm, expandBusSliceExpression } from "./beautify-bus.logic";

describe("Beautify & Typed Bus Bundle logic (FEAT-001, FEAT-002)", () => {
  it("expands vector slices DATA[31:0] and AXI4_LITE protocol bundles", () => {
    const vec = expandBusSliceExpression("DATA[31:0]", "RAW_VECTOR");
    expect(vec.valid).toBe(true);
    expect(vec.bitWidth).toBe(32);
    expect(vec.expandedNets[0]).toBe("DATA_31");
    expect(vec.expandedNets[31]).toBe("DATA_0");

    const axi = expandBusSliceExpression("CTRL_BUS", "AXI4_LITE");
    expect(axi.valid).toBe(true);
    expect(axi.bitWidth).toBe(16);
    expect(axi.expandedNets[0]).toBe("CTRL_BUS.AWADDR[31:0]");

    const tooWide = expandBusSliceExpression("WIDE[255:0]", "RAW_VECTOR");
    expect(tooWide.valid).toBe(false);
  });

  it("computes Sugiyama + A* crossing reduction and micrometer wirelength savings", () => {
    const vm = buildBeautifyBusVm(
      {
        sheetId: "sheet-power-stage",
        gridPitchNm: 254_000n,
        crossingCountBefore: 20,
        crossingCountAfter: 3,
        wirelengthSavedNm: 14_500_000n,
        applied: false,
      },
      "PWM[5:0]",
      "RAW_VECTOR",
    );
    expect(vm.crossingBadge).toBe("Preview: 20 -> 3 Crossings (-85%)");
    expect(vm.wirelengthBadge).toBe("Saved: 14500 um");
    expect(vm.netRows.length).toBe(6);
  });
});
