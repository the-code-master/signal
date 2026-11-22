// Module: web/libs/fpga-studio/src/lib/fpga-studio-workbench/fpga-studio-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies FPGA part selection and signal-fpga engine badge.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  FPGA_STUDIO_WORKBENCH_DESCRIPTOR,
  buildFpgaStudioWorkbenchVm,
} from "./fpga-studio-workbench.logic";

describe("fpga-studio-workbench.logic (CU-P6-02)", () => {
  it("builds FPGA studio VM with multi-vendor die presets", () => {
    const vm = buildFpgaStudioWorkbenchVm(FPGA_STUDIO_WORKBENCH_DESCRIPTOR, "AGFB014_AGILEX7");
    expect(vm.deviceSummaryBadge).toContain("AGFB014_AGILEX7");
    expect(vm.engineBadge).toContain("signal-fpga");
  });
});
