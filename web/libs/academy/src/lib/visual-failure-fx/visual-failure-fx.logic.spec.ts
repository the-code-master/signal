// Module: web/libs/academy/src/lib/visual-failure-fx/visual-failure-fx.logic.spec.ts — FEAT-113 Visual Smoke FX Unit Tests
// Invariants: Verifies bigint SOA overstress ratios in EXPLORER/MAKER modes and 1-click Auto-Fix resolution.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildVisualFailureFxVm, DEFAULT_SOA_SCENARIOS } from "./visual-failure-fx.logic";

describe("VisualFailureFx SOA smoke diagnostics (FEAT-113)", () => {
  it("reports bigint _uv/_uw/_mkelvin overstress and clears violation on Auto-Fix", () => {
    const smoking = buildVisualFailureFxVm(DEFAULT_SOA_SCENARIOS[0], "EXPLORER", false);
    expect(smoking.fxTone).toBe("danger");
    expect(smoking.diagnosticExplanation).toContain("Why did R12_LED_LIM smoke?");
    expect(smoking.telemetryPills[2]).toContain("598150 mK");

    const fixed = buildVisualFailureFxVm(DEFAULT_SOA_SCENARIOS[0], "MAKER", true);
    expect(fixed.fxTone).toBe("ok");
    expect(fixed.fxBadge).toContain("CRCW2512100RFKEG");
  });
});
