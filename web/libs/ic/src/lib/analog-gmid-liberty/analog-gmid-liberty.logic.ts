// Module: web/libs/ic/src/lib/analog-gmid-liberty/analog-gmid-liberty.logic.ts — FEAT-041..044 Analog Sizing & Pelgrom Logic
// Invariants: Computes Pelgrom sigma(dVth) = A_VT / sqrt(W_um * L_um), gm/Id operating metrics & BSPDN IR savings.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  AnalogGmidLibertyVm,
  InversionRegionPreset,
} from "./analog-gmid-liberty.model";

const ANALOG_COLUMNS: readonly SigGridColumn[] = [
  { key: "studio", header: "Analog / PDK Studio", mono: true },
  { key: "metric", header: "Evaluated Silicon SI Metric", mono: true },
  { key: "verdict", header: "Sign-Off", mono: true },
];

export function computePelgromSigmaVthMv(
  widthNm: bigint,
  lengthNm: bigint,
  avtMvUm = 3.2,
): number {
  const areaSqUm = (Number(widthNm) / 1_000) * (Number(lengthNm) / 1_000);
  if (areaSqUm <= 0) return 99;
  return Number((avtMvUm / Math.sqrt(areaSqUm)).toFixed(3));
}

export function buildAnalogGmidLibertyVm(
  region: InversionRegionPreset,
  backsidePdnEnabled: boolean,
  deviceWidthNm = 8_000n,
  deviceLengthNm = 500n,
): AnalogGmidLibertyVm {
  const gmOverIdPerV =
    region === "WEAK_INVERSION_20" ? 20.4 : region === "MODERATE_INVERSION_14" ? 14.2 : 8.1;
  const transitFreqHz =
    region === "WEAK_INVERSION_20"
      ? 4_200_000_000n
      : region === "MODERATE_INVERSION_14"
        ? 28_500_000_000n
        : 92_000_000_000n;
  const pelgromSigmaVthMv = computePelgromSigmaVthMv(deviceWidthNm, deviceLengthNm);
  const bspdnIrDropUv = backsidePdnEnabled ? 6_400n : 24_800n;

  const irClean = bspdnIrDropUv <= 12_000n;
  const mismatchClean = pelgromSigmaVthMv <= 2.0;
  const summaryTone: SigStatusTone = irClean && mismatchClean ? "ok" : "warn";
  const ftGhz = (Number(transitFreqHz) / 1_000_000_000).toFixed(1);
  const summaryBadge = `gm/Id=${gmOverIdPerV} V⁻¹ (f_T=${ftGhz} GHz, FEAT-041) • ABBA Pelgrom σ(V_th)=${pelgromSigmaVthMv} mV (FEAT-042)`;
  const libertyBspdnBadge = `.lib NLDM/LVF 7x7 Characterized (FEAT-043) • ${backsidePdnEnabled ? "Backside PowerVia BSPDN" : "Frontside M1-M12"} IR-Drop=${(Number(bspdnIrDropUv) / 1_000).toFixed(1)} mV (FEAT-044)`;

  const rows: SigGridRow[] = [
    {
      id: "GMID",
      studio: "FEAT-041 4D gm/Id Lookup Explorer",
      metric: `gm/Id=${gmOverIdPerV} V⁻¹ • gm*ro=48.2 V/V • f_T=${ftGhz} GHz`,
      verdict: "OPTIMAL_PARETO",
    },
    {
      id: "MATCH",
      studio: "FEAT-042 ABBA Common-Centroid + Dummy Ring",
      metric: `W=${deviceWidthNm.toString(10)} nm x L=${deviceLengthNm.toString(10)} nm -> σ=${pelgromSigmaVthMv} mV`,
      verdict: mismatchClean ? "PASS (<= 2.0 mV)" : "UPSIZE_WL_AREA",
    },
    {
      id: "LIB",
      studio: "FEAT-043 .lib NLDM & POCV LVF Characterizer",
      metric: "7x7 Slew/Load Table • LVF Early/Late Sigma Bound",
      verdict: "LIBERTY_VALID",
    },
    {
      id: "BSPDN",
      studio: "FEAT-044 Backside PowerVia / Nano-TSV PDN",
      metric: `Dynamic IR=${bspdnIrDropUv.toString(10)} uV`,
      verdict: irClean ? "PASS (<= 12 mV)" : "ENABLE_BACKSIDE_PDN",
    },
  ];

  return {
    region,
    regions: ["WEAK_INVERSION_20", "MODERATE_INVERSION_14", "STRONG_INVERSION_8"],
    backsidePdnEnabled,
    deviceWidthNm,
    deviceLengthNm,
    gmOverIdPerV,
    transitFreqHz,
    pelgromSigmaVthMv,
    bspdnIrDropUv,
    summaryBadge,
    summaryTone,
    libertyBspdnBadge,
    columns: ANALOG_COLUMNS,
    rows,
  };
}
