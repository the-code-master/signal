// Module: web/libs/harness-3d/src/lib/harness-3d-workbench/harness-3d-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies bigint nm spline bend radius and signal-harness engine badge.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  HARNESS_3D_WORKBENCH_DESCRIPTOR,
  buildHarness3dWorkbenchVm,
} from "./harness-3d-workbench.logic";

describe("harness-3d-workbench.logic (CU-P5-03)", () => {
  it("builds harness 3D workbench VM with bigint nm bend radius", () => {
    const vm = buildHarness3dWorkbenchVm(HARNESS_3D_WORKBENCH_DESCRIPTOR);
    expect(vm.minBundleBendNm).toBe(42_000_000n);
    expect(vm.engineBadge).toContain("signal-harness");
  });
});
