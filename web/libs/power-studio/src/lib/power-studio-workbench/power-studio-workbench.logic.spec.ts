// Module: web/libs/power-studio/src/lib/power-studio-workbench/power-studio-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies 3 Power Studio tabs covering FEAT-022..030 and engine badge construction.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import {
  POWER_STUDIO_WORKBENCH_DESCRIPTOR,
  buildPowerStudioWorkbenchVm,
} from "./power-studio-workbench.logic";

describe("power-studio-workbench.logic (CU-P4-02)", () => {
  it("exposes 3 Power Integrity & Power Electronics tabs", () => {
    const vm = buildPowerStudioWorkbenchVm(POWER_STUDIO_WORKBENCH_DESCRIPTOR, "BODE_BMS_MOTOR");
    expect(vm.tabs).toHaveLength(3);
    expect(vm.activeTab).toBe("BODE_BMS_MOTOR");
    expect(vm.engineBadge).toContain("signal-power-integrity");
  });
});
