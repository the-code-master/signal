// Module: web/libs/ic/src/lib/ic-workbench/ic-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies 3 IC/ASIC studio tabs and signal-ic engine badge.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import { IC_WORKBENCH_DESCRIPTOR, buildIcWorkbenchVm } from "./ic-workbench.logic";

describe("ic-workbench.logic (CU-P6-01)", () => {
  it("exposes 3 IC/ASIC studios covering FEAT-041..050, I-07, F-125, and F-148", () => {
    const vm = buildIcWorkbenchVm(IC_WORKBENCH_DESCRIPTOR, "SOC_STA_UCIE_LITHO");
    expect(vm.tabs).toHaveLength(3);
    expect(vm.engineBadge).toContain("signal-ic");
  });
});
