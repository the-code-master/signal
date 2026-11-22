// Module: web/libs/schematic/src/lib/live-sim-hud/live-sim-hud.logic.ts — Pure Rotary Potentiometer, Fault & Current-River HUD Math
// Invariants: Exact bigint milliohm wiper split; maps nanoamp branch telemetry to 120 FPS particle velocity & SOA thermal margin.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";
import type { LiveSimHudState, LiveSimHudVm } from "./live-sim-hud.model";

export function computePotentiometerLegsMohm(
  totalMohm: bigint,
  wiperPercent: number,
): { readonly lowerLegMohm: bigint; readonly upperLegMohm: bigint } {
  const clampedPct = BigInt(Math.max(0, Math.min(100, Math.round(wiperPercent))));
  const lowerLegMohm = (totalMohm * clampedPct) / 100n;
  const upperLegMohm = totalMohm - lowerLegMohm;
  return { lowerLegMohm, upperLegMohm };
}

export function computeParticleDensityTier(rmsCurrentNa: bigint, enabled: boolean): string {
  if (!enabled) {
    return "Current-River: Paused";
  }
  const milliAmps = rmsCurrentNa / 1_000_000n;
  if (milliAmps >= 10_000n) {
    return `Current-River: High-Flux 96 particles/net (${milliAmps.toString(10)} mA RMS)`;
  }
  if (milliAmps >= 1_000n) {
    return `Current-River: Nominal 48 particles/net (${milliAmps.toString(10)} mA RMS)`;
  }
  return `Current-River: Trickle 16 particles/net (${milliAmps.toString(10)} mA RMS)`;
}

export function buildLiveSimHudVm(state: LiveSimHudState): LiveSimHudVm {
  const { lowerLegMohm, upperLegMohm } = computePotentiometerLegsMohm(state.potTotalMohm, state.wiperPercent);
  const lowerOhm = lowerLegMohm / 1_000n;
  const upperOhm = upperLegMohm / 1_000n;
  const switchTone: SigStatusTone = state.switchClosed ? "ok" : "neutral";
  const faultTone: SigStatusTone = state.faultMode === "NONE" ? "ok" : "danger";

  const tjCelsius = (state.junctionTempMkelvin - 273_150n) / 1_000n;
  const maxCelsius = (state.maxJunctionTempMkelvin - 273_150n) / 1_000n;
  const marginCelsius = maxCelsius - tjCelsius;
  const soaTone: SigStatusTone = !state.soaThermalEnabled
    ? "neutral"
    : marginCelsius <= 0n
      ? "danger"
      : marginCelsius <= 25n
        ? "warn"
        : "ok";
  const soaThermalBadge = state.soaThermalEnabled
    ? `SOA Badge: Tj=${tjCelsius.toString(10)} °C (Margin ${marginCelsius.toString(10)} °C)`
    : "SOA Badge: Hidden";

  return {
    wiperResistanceText: `RV1 (${state.wiperPercent}%): R_low=${lowerOhm.toString(10)} Ω | R_high=${upperOhm.toString(10)} Ω`,
    upperLegMohm,
    lowerLegMohm,
    switchBadge: state.switchClosed ? "SW1: CLOSED (Conducting)" : "SW1: OPEN (Isolated)",
    switchTone,
    faultBadge: `Fault Injection: ${state.faultMode}`,
    faultTone,
    riverDensityBadge: computeParticleDensityTier(state.rmsCurrentNa, state.currentRiverEnabled),
    soaThermalBadge,
    soaTone,
  };
}
