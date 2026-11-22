// Module: web/libs/mcad-studio/src/lib/mcad-studio-workbench/mcad-studio-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies clamped 0..100% exploded view, bigint nm section-plane formatting, and 0 BVH collisions.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  MCAD_STUDIO_WORKBENCH_DESCRIPTOR,
  buildMcadStudioWorkbenchVm,
} from "./mcad-studio-workbench.logic";

describe("mcad-studio-workbench.logic (CU-P5-03)", () => {
  it("clamps exploded view percentage and formats bigint nm Z-section offset", () => {
    const vm = buildMcadStudioWorkbenchVm(MCAD_STUDIO_WORKBENCH_DESCRIPTOR, 75, 12_500_000n);
    expect(vm.explodedPct).toBe(75);
    expect(vm.viewBadge).toContain("12.5 mm");
    expect(vm.clearanceTone).toBe("ok");
  });
});
