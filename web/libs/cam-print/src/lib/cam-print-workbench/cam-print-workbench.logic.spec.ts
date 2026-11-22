// Module: web/libs/cam-print/src/lib/cam-print-workbench/cam-print-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies 2 CAM/Print studio tabs and signal-cam engine badge.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  CAM_PRINT_WORKBENCH_DESCRIPTOR,
  buildCamPrintWorkbenchVm,
} from "./cam-print-workbench.logic";

describe("cam-print-workbench.logic (CU-P5-04)", () => {
  it("exposes subtractive CNC and additive Direct-B-Rep slicer tabs", () => {
    const vm = buildCamPrintWorkbenchVm(
      CAM_PRINT_WORKBENCH_DESCRIPTOR,
      "ADDITIVE_DIRECT_BREP_SLICER",
    );
    expect(vm.tabs).toHaveLength(2);
    expect(vm.engineBadge).toContain("signal-cam");
  });
});
