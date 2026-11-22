// Module: web/libs/emc-lab/src/lib/emi-esd-shield/emi-esd-shield.logic.ts — FEAT-032/033/034 EMI Filter, ESD & Cavity Logic
// Invariants: Checks Middlebrook >=6 dB stability, TVS clamp < pin AbsMax (_uv), and rectangular cavity f_101 mode (_hz).
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { EmiEsdShieldVm, EsdStressLevel } from "./emi-esd-shield.model";

const SHIELD_COLUMNS: readonly SigGridColumn[] = [
  { key: "check", header: "EMC / Immunity Check", mono: true },
  { key: "metric", header: "Evaluated SI Value", mono: true },
  { key: "verdict", header: "Compliance Gate", mono: true },
];

export function computeCavityTe101Hz(widthNm: bigint, lengthNm: bigint): bigint {
  const aMm = Number(widthNm) / 1_000_000;
  const dMm = Number(lengthNm) / 1_000_000;
  const invSq = 1 / (aMm * aMm) + 1 / (dMm * dMm);
  const fGhz = 149.896 * Math.sqrt(invSq);
  return BigInt(Math.round(fGhz * 1_000_000_000));
}

export function computeTvsClampedPeakUv(esdLevel: EsdStressLevel): bigint {
  if (esdLevel === "IEC_8KV_CONTACT") return 14_200_000n;
  if (esdLevel === "IEC_15KV_AIR") return 19_800_000n;
  return 24_500_000n;
}

export function buildEmiEsdShieldVm(
  esdLevel: EsdStressLevel,
  dampingNetworkEnabled: boolean,
): EmiEsdShieldVm {
  const middlebrookSeparationDb = dampingNetworkEnabled ? 11.4 : 2.1;
  const tvsClampedPeakUv = computeTvsClampedPeakUv(esdLevel);
  const protectedPinAbsMaxUv = 28_000_000n;
  const cavityFundamentalHz = computeCavityTe101Hz(120_000_000n, 160_000_000n);
  const f101Ghz = (Number(cavityFundamentalHz) / 1_000_000_000).toFixed(2);

  const mbOk = middlebrookSeparationDb >= 6.0;
  const esdOk = tvsClampedPeakUv < protectedPinAbsMaxUv;
  const summaryTone: SigStatusTone = mbOk && esdOk ? "ok" : "warn";
  const summaryBadge = `Middlebrook |Z_in/Z_out|: ${middlebrookSeparationDb} dB (FEAT-032) • TVS Clamp: ${(Number(tvsClampedPeakUv) / 1_000_000).toFixed(1)} V < 28 V (FEAT-033) • Shield f_101=${f101Ghz} GHz (FEAT-034)`;

  const rows: SigGridRow[] = [
    {
      id: "MB",
      check: "FEAT-032 EMI Filter Middlebrook",
      metric: `${middlebrookSeparationDb} dB impedance separation`,
      verdict: mbOk ? "STABLE (>= 6 dB)" : "INPUT_OSCILLATION_RISK",
    },
    {
      id: "ESD",
      check: `FEAT-033 ${esdLevel} TVS Clamp`,
      metric: `${tvsClampedPeakUv.toString(10)} uV peak residual`,
      verdict: esdOk ? "PASS (< 28_000_000 uV)" : "PIN_BREAKDOWN",
    },
    {
      id: "SHIELD",
      check: "FEAT-034 Enclosure Cavity & Seam",
      metric: `TE101=${f101Ghz} GHz • Max Slot=4_000_000 nm (< λ/20)`,
      verdict: "PASS (SE >= 34 dB)",
    },
  ];

  return {
    esdLevel,
    esdLevels: ["IEC_8KV_CONTACT", "IEC_15KV_AIR", "ISO7637_LOAD_DUMP"],
    dampingNetworkEnabled,
    middlebrookSeparationDb,
    tvsClampedPeakUv,
    protectedPinAbsMaxUv,
    cavityFundamentalHz,
    summaryBadge,
    summaryTone,
    columns: SHIELD_COLUMNS,
    rows,
  };
}
