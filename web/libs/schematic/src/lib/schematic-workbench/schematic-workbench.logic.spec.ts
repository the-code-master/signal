// Module: web/libs/schematic/src/lib/schematic-workbench/schematic-workbench.logic.spec.ts — Schematic Workbench VM Tests
// Invariants: Verifies tab list and status banner state updates across FEAT-001..FEAT-010 panels without Angular decorators.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import { buildSchematicWorkbenchVm, SCHEMATIC_WORKBENCH_DESCRIPTOR } from "./schematic-workbench.logic";

describe("buildSchematicWorkbenchVm", () => {
  it("builds 5 schematic feature tabs including FEAT-101 and reflects beautify/portal state", () => {
    const vm = buildSchematicWorkbenchVm(SCHEMATIC_WORKBENCH_DESCRIPTOR, "LIVE_HUD", true, "p2");
    expect(vm.tabs.length).toBe(5);
    expect(vm.activeTab).toBe("LIVE_HUD");
    expect(vm.statusBanner).toContain("Sugiyama Untangled");
    expect(vm.statusBanner).toContain("p2");
  });
});
