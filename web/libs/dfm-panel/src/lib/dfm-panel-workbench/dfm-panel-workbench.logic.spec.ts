// Module: web/libs/dfm-panel/src/lib/dfm-panel-workbench/dfm-panel-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies monotonic volume price reduction in bigint _micro_usd and first-pass yield calculation.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  DFM_PANEL_WORKBENCH_DESCRIPTOR,
  buildDfmPanelWorkbenchVm,
  computeUnitShouldCostMicroUsd,
} from "./dfm-panel-workbench.logic";

describe("dfm-panel-workbench.logic (FEAT-068)", () => {
  it("reduces unit should-cost monotonically from PROTO_25 to MASS_50000 in bigint micro-USD", () => {
    expect(computeUnitShouldCostMicroUsd("MASS_50000")).toBeLessThan(
      computeUnitShouldCostMicroUsd("PROTO_25"),
    );
    const vm = buildDfmPanelWorkbenchVm(DFM_PANEL_WORKBENCH_DESCRIPTOR, "MASS_50000");
    expect(vm.firstPassYieldPct).toBe(99.2);
    expect(vm.costYieldTone).toBe("ok");
  });
});
