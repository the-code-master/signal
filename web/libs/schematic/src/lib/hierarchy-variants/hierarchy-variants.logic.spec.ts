// Module: web/libs/schematic/src/lib/hierarchy-variants/hierarchy-variants.logic.spec.ts — FEAT-005..007 Unit Tests
// Invariants: Verifies Repeat(N) RefDes step offsets, cross-sheet bigint nm portal coordinates, and DNP/Alt-MPN variant resolution.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildHierarchyVariantsVm,
  DEFAULT_VARIANT_BOM,
  expandMultiChannelMatrix,
  resolveVariantStuffingRows,
} from "./hierarchy-variants.logic";

describe("Hierarchy Portals, Repeat(N), and Assembly Variants (FEAT-005..007)", () => {
  it("expands Repeat(3) multi-channel RefDes offsets and net templates", () => {
    const rows = expandMultiChannelMatrix({
      blockId: "PHASE",
      repeatCount: 3,
      refdesBaseStep: 100,
      netTemplate: "PH_{ch}_OUT",
    });
    expect(rows.length).toBe(3);
    expect(rows[0]["hsSwitch"]).toBe("Q101");
    expect(rows[2]["hsSwitch"]).toBe("Q301");
    expect(rows[2]["phaseNet"]).toBe("PH_3_OUT");
  });

  it("resolves DNP curtain masks and alternate MPNs per assembly variant", () => {
    const cost = resolveVariantStuffingRows(DEFAULT_VARIANT_BOM, "COST_REDUCED");
    expect(cost.populatedCount).toBe(1);
    expect(cost.dnpCount).toBe(2);
    expect(cost.rows[0]["resolvedMpn"]).toContain("C3M0021120K");

    const vm = buildHierarchyVariantsVm("p2", 4, "DEV_BENCH_400V");
    expect(vm.selectedPortalSummary).toContain("22100000");
    expect(vm.channelRows.length).toBe(4);
    expect(vm.variantSummaryBadge).toBe("DEV_BENCH_400V: 3 Populated / 0 DNP");
  });
});
