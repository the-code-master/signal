// Module: web/libs/academy/src/lib/visual-failure-fx/visual-failure-fx.logic.ts — Pure SOA Smoke Overstress & Auto-Fix Logic
// Invariants: Computes exact bigint overstress % across _uv, _uw, and _mkelvin; adapts pedagogy for EXPLORER/MAKER/PRO.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigUiProfile } from "@signal/design-system";
import type { SigStatusTone } from "@signal/ui-kit";
import type { SoaFailureScenario, VisualFailureFxVm } from "./visual-failure-fx.model";

export const DEFAULT_SOA_SCENARIOS: readonly SoaFailureScenario[] = [
  {
    id: "soa-res-smoke",
    entityId: "R12_LED_LIM",
    partSummary: "0402 100 Ω 1/16W Thick-Film Resistor",
    failureMode: "THERMAL_OVERPOWER_SMOKE",
    peakVoltageUv: 12_000_000n,
    ratedVoltageUv: 50_000_000n,
    peakPowerUw: 1_440_000n,
    ratedPowerUw: 62_500n,
    junctionTempMkelvin: 598_150n,
    maxTempMkelvin: 428_150n,
    upgradeMpn: "CRCW2512100RFKEG (2512 1W Pulse-Proof)",
    upgradeVoltageUv: 200_000_000n,
    upgradePowerUw: 2_000_000n,
  },
  {
    id: "soa-mosfet-avalanche",
    entityId: "Q3_LOW_SIDE",
    partSummary: "BSS138 50V N-Ch Trench MOSFET",
    failureMode: "VDS_AVALANCHE_BREAKDOWN",
    peakVoltageUv: 78_500_000n,
    ratedVoltageUv: 50_000_000n,
    peakPowerUw: 4_200_000n,
    ratedPowerUw: 360_000n,
    junctionTempMkelvin: 488_150n,
    maxTempMkelvin: 423_150n,
    upgradeMpn: "BSC093N15NS5 (150V OptiMOS-5 + Snubber)",
    upgradeVoltageUv: 150_000_000n,
    upgradePowerUw: 12_000_000n,
  },
];

export function buildVisualFailureFxVm(
  scenario: SoaFailureScenario,
  profile: SigUiProfile,
  autoFixed: boolean,
): VisualFailureFxVm {
  const effRatedV = autoFixed ? scenario.upgradeVoltageUv : scenario.ratedVoltageUv;
  const effRatedP = autoFixed ? scenario.upgradePowerUw : scenario.ratedPowerUw;
  const effTempMk = autoFixed ? 338_150n : scenario.junctionTempMkelvin;

  const vRatioPct = (scenario.peakVoltageUv * 100n) / effRatedV;
  const pRatioPct = (scenario.peakPowerUw * 100n) / effRatedP;
  const tjCelsius = (effTempMk - 273_150n) / 1_000n;
  const maxCelsius = (scenario.maxTempMkelvin - 273_150n) / 1_000n;

  const fxTone: SigStatusTone = autoFixed ? "ok" : "danger";
  const fxBadge = autoFixed
    ? `SOA Verified: ${scenario.entityId} -> ${scenario.upgradeMpn}`
    : `SMOKE FX ACTIVE: ${scenario.entityId} (${scenario.failureMode})`;

  const diagnosticExplanation = autoFixed
    ? `Upgraded ${scenario.entityId} to ${scenario.upgradeMpn}. Peak stress is now ${pRatioPct.toString(10)}% of rated power and ${vRatioPct.toString(10)}% of V_max.`
    : profile === "EXPLORER"
      ? `Why did ${scenario.entityId} smoke? It absorbed ${(scenario.peakPowerUw / 1_000n).toString(10)} mW (${pRatioPct.toString(10)}% of its rating), heating the die to ${tjCelsius.toString(10)} °C!`
      : `SOA violation on ${scenario.entityId}: P_peak=${scenario.peakPowerUw.toString(10)} uW (${pRatioPct.toString(10)}%), V_peak=${scenario.peakVoltageUv.toString(10)} uV (${vRatioPct.toString(10)}%), T_j=${tjCelsius.toString(10)} °C > ${maxCelsius.toString(10)} °C.`;

  return {
    scenario,
    profile,
    autoFixed,
    fxBadge,
    fxTone,
    telemetryPills: [
      `peak_voltage_uv: ${scenario.peakVoltageUv.toString(10)} uV (${vRatioPct.toString(10)}%)`,
      `peak_power_uw: ${scenario.peakPowerUw.toString(10)} uW (${pRatioPct.toString(10)}%)`,
      `junction_temp_mkelvin: ${effTempMk.toString(10)} mK (${tjCelsius.toString(10)} °C)`,
    ],
    diagnosticExplanation,
    actionButtonLabel: autoFixed ? "Revert to Faulted Part" : "Auto-Fix Component Rating",
  };
}
