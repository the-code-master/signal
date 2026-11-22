// Module: web/libs/schematic/src/lib/live-sim-hud/live-sim-hud.logic.spec.ts — FEAT-003 & FEAT-004 Unit Tests
// Invariants: Verifies exact bigint milliohm wiper conservation, particle flux tiers, and SOA junction temperature margin.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildLiveSimHudVm, computePotentiometerLegsMohm } from "./live-sim-hud.logic";

describe("Live Simulation HUD logic (FEAT-003, FEAT-004)", () => {
  it("conserves total milliohm resistance across potentiometer wiper positions", () => {
    const split = computePotentiometerLegsMohm(10_000_000n, 35);
    expect(split.lowerLegMohm).toBe(3_500_000n);
    expect(split.upperLegMohm).toBe(6_500_000n);
    expect(split.lowerLegMohm + split.upperLegMohm).toBe(10_000_000n);
  });

  it("maps branch RMS nanoamps and SOA millikelvin margin to semantic status tones", () => {
    const vm = buildLiveSimHudVm({
      potTotalMohm: 10_000_000n,
      wiperPercent: 75,
      switchClosed: true,
      faultMode: "SHORT_CIRCUIT",
      currentRiverEnabled: true,
      soaThermalEnabled: true,
      rmsCurrentNa: 18_500_000_000n,
      junctionTempMkelvin: 438_150n,
      maxJunctionTempMkelvin: 448_150n,
    });
    expect(vm.faultTone).toBe("danger");
    expect(vm.riverDensityBadge).toContain("High-Flux 96 particles/net (18500 mA RMS)");
    expect(vm.soaTone).toBe("warn");
    expect(vm.soaThermalBadge).toBe("SOA Badge: Tj=165 °C (Margin 10 °C)");
  });
});
