// Module: web/libs/power-studio/src/lib/thermal-sequencing/thermal-sequencing.logic.ts — FEAT-029/030 Thermal & Sequencing Logic
// Invariants: Computes R_theta_JA junction temperature in bigint mK and verifies monotonic PGOOD sequencing.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  CoolingAssemblyKind,
  PowerSeqRailSlot,
  ThermalSequencingVm,
} from "./thermal-sequencing.model";

export const DEFAULT_SEQ_RAILS: readonly PowerSeqRailSlot[] = [
  { railName: "1. VDD_CORE_0V85", nominalUv: 850_000n, enableAtPs: 1_000_000_000n, pgValidAtPs: 2_400_000_000n },
  { railName: "2. VDD_PLL_1V8", nominalUv: 1_800_000n, enableAtPs: 2_600_000_000n, pgValidAtPs: 4_100_000_000n },
  { railName: "3. VDD_IO_3V3", nominalUv: 3_300_000n, enableAtPs: 4_300_000_000n, pgValidAtPs: 5_800_000_000n },
  { railName: "4. VCC_GATE_18V", nominalUv: 18_000_000n, enableAtPs: 6_200_000_000n, pgValidAtPs: 8_500_000_000n },
];

const SEQ_COLUMNS: readonly SigGridColumn[] = [
  { key: "rail", header: "Power Rail (FEAT-030)", mono: true },
  { key: "enMs", header: "EN Ramp Start", mono: true },
  { key: "pgMs", header: "PGOOD Valid", mono: true },
  { key: "backpower", header: "ESD Clamp / Body-Diode Check", mono: true },
];

export function computePeakTjMkelvin(
  coolingKind: CoolingAssemblyKind,
  ambientMkelvin = 338_150n,
  modulePowerW = 140n,
): bigint {
  const rThetaMkPerW =
    coolingKind === "WEG_LIQUID_COLD_PLATE"
      ? 380n
      : coolingKind === "FORCED_25_CFM_HEATSINK"
        ? 680n
        : 1_450n;
  return ambientMkelvin + modulePowerW * rThetaMkPerW;
}

export function verifyMonotonicSequencing(rails: readonly PowerSeqRailSlot[]): boolean {
  for (let i = 1; i < rails.length; i++) {
    if (rails[i].enableAtPs < rails[i - 1].pgValidAtPs) {
      return false;
    }
  }
  return true;
}

export function buildThermalSequencingVm(
  coolingKind: CoolingAssemblyKind,
  rails: readonly PowerSeqRailSlot[] = DEFAULT_SEQ_RAILS,
): ThermalSequencingVm {
  const maxRatedTjMkelvin = 448_150n;
  const peakJunctionMkelvin = computePeakTjMkelvin(coolingKind);
  const tjCelsius = Number((peakJunctionMkelvin - 273_150n) / 100n) / 10;
  const monotonic = verifyMonotonicSequencing(rails);

  const tjOk = peakJunctionMkelvin <= maxRatedTjMkelvin;
  const summaryTone: SigStatusTone = tjOk && monotonic ? "ok" : "danger";
  const summaryBadge = `Peak Tj=${tjCelsius} °C (${coolingKind}, Max 175 °C, FEAT-029) • Gantt PGOOD Interlock: ${monotonic ? "0 Back-Powering" : "VIOLATION"} (FEAT-030)`;

  const rows: SigGridRow[] = rails.map((r, idx) => {
    const enMs = (Number(r.enableAtPs) / 1_000_000_000).toFixed(1);
    const pgMs = (Number(r.pgValidAtPs) / 1_000_000_000).toFixed(1);
    const prevOk = idx === 0 || r.enableAtPs >= rails[idx - 1].pgValidAtPs;
    return {
      id: r.railName,
      rail: r.railName,
      enMs: `${enMs} ms`,
      pgMs: `${pgMs} ms`,
      backpower: prevOk ? "SAFE (Core Precedes IO)" : "BODY_DIODE_REVERSE_STRESS",
    };
  });

  return {
    coolingKind,
    coolingKinds: ["NATURAL_CONVECTION", "FORCED_25_CFM_HEATSINK", "WEG_LIQUID_COLD_PLATE"],
    peakJunctionMkelvin,
    maxRatedTjMkelvin,
    monotonicSequencingClean: monotonic,
    summaryBadge,
    summaryTone,
    columns: SEQ_COLUMNS,
    rows,
  };
}
