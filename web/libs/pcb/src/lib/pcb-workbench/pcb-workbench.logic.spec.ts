// Module: web/libs/pcb/src/lib/pcb-workbench/pcb-workbench.logic.spec.ts — PCB Workbench Unit Tests
// Invariants: Verifies 3 PCB studio tabs and engine badge construction without Angular decorators.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import { PCB_WORKBENCH_DESCRIPTOR, buildPcbWorkbenchVm } from "./pcb-workbench.logic";

describe("pcb-workbench.logic (CU-P4-01)", () => {
  it("exposes 3 high-speed PCB studios and builds engine badge", () => {
    const vm = buildPcbWorkbenchVm(PCB_WORKBENCH_DESCRIPTOR, "SI_EM_RF_TDR");
    expect(vm.tabs).toHaveLength(3);
    expect(vm.activeTab).toBe("SI_EM_RF_TDR");
    expect(vm.engineBadge).toContain("signal-pcb-router");
  });
});
