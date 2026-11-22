// Module: web/libs/dfm-panel/src/lib/panel-dfa-fatigue-coating/panel-dfa-fatigue-coating.logic.spec.ts — Unit Tests
// Invariants: Verifies IPC-7525 stencil area ratio >=0.66 and MLCC depaneling microstrain limit <=500 uStrain.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildPanelDfaFatigueCoatingVm,
  computeDepanelingMicrostrain,
  computeStencilAreaRatio,
} from "./panel-dfa-fatigue-coating.logic";

describe("panel-dfa-fatigue-coating.logic (FEAT-063..066)", () => {
  it("computes 0.60 vs >=0.66 stencil area ratio and flags V-score MLCC flex strain near 1mm edge", () => {
    expect(computeStencilAreaRatio(280_000n, 280_000n, 100_000n)).toBe(0.7);
    expect(computeDepanelingMicrostrain("V_SCORE_GUILLOTINE", 1_000_000n)).toBeGreaterThan(500);
    expect(computeDepanelingMicrostrain("ROUTER_MOUSE_BITE_TABS", 2_000_000n)).toBeLessThanOrEqual(500);

    const vm = buildPanelDfaFatigueCoatingVm("ROUTER_MOUSE_BITE_TABS", 2_000_000n);
    expect(vm.mlccPeakMicrostrain).toBeLessThanOrEqual(500);
  });
});
