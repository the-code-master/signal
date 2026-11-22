// Module: web/libs/sim-viewer/src/lib/sim-viewer-workbench/sim-viewer-workbench.logic.spec.ts — Sim-Viewer Workbench Tests
// Invariants: Verifies 6 simulation studio tabs and schematic cross-probe banner formatting.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import { buildSimViewerWorkbenchVm, SIM_VIEWER_WORKBENCH_DESCRIPTOR } from "./sim-viewer-workbench.logic";

describe("buildSimViewerWorkbenchVm", () => {
  it("exposes all 9 simulation analytics & neuro-symbolic tabs and tracks cross-probe net", () => {
    const vm = buildSimViewerWorkbenchVm(
      SIM_VIEWER_WORKBENCH_DESCRIPTOR,
      "WHAT_IF_ADJOINT",
      "GATE_DRV_HS_U",
      "rms(i(L1))",
    );
    expect(vm.tabs.length).toBe(9);
    expect(vm.crossProbeBadge).toContain("GATE_DRV_HS_U");
    expect(vm.crossProbeBadge).toContain("rms(i(L1))");
  });
});
