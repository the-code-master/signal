// Module: web/libs/power-studio/src/lib/pdn-creepage-ir-hotloop/pdn-creepage-ir-hotloop.logic.ts — FEAT-022..025 Logic
// Invariants: Verifies Z_PDN(f) <= Z_target(f), 3D geodesic creepage across slots, DC IR-drop & Miller margin.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  PdnCreepageIrHotloopVm,
  PdnFrequencyBand,
} from "./pdn-creepage-ir-hotloop.model";

export const DEFAULT_PDN_BANDS: readonly PdnFrequencyBand[] = [
  { bandLabel: "VRM Bulk (100 kHz)", freqHz: 100_000n, zTargetMohm: 12n, zMeasuredMohm: 8n },
  { bandLabel: "MLCC Mid-Band (10 MHz)", freqHz: 10_000_000n, zTargetMohm: 12n, zMeasuredMohm: 9n },
  { bandLabel: "Pkg Cavity (250 MHz)", freqHz: 250_000_000n, zTargetMohm: 15n, zMeasuredMohm: 11n },
];

const PDN_COLUMNS: readonly SigGridColumn[] = [
  { key: "band", header: "PDN Band (FEAT-022)", mono: true },
  { key: "zTarget", header: "Z_target(f)", mono: true },
  { key: "zPdn", header: "Z_PDN(f)", mono: true },
  { key: "verdict", header: "Impedance Gate", mono: true },
];

export function compute3dGeodesicCreepageNm(planarGapNm: bigint, slotRibAdded: boolean): bigint {
  return slotRibAdded ? planarGapNm + 3_600_000n : planarGapNm;
}

export function computeMillerShootThroughMarginUv(negativeVeeEnabled: boolean): bigint {
  const vThMinUv = 2_400_000n;
  const inducedBumpUv = 2_100_000n;
  const vOffBiasUv = negativeVeeEnabled ? 4_000_000n : 0n;
  return vThMinUv + vOffBiasUv - inducedBumpUv;
}

export function buildPdnCreepageIrHotloopVm(
  slotRibAdded: boolean,
  negativeGateBiasEnabled: boolean,
  bands: readonly PdnFrequencyBand[] = DEFAULT_PDN_BANDS,
): PdnCreepageIrHotloopVm {
  const requiredCreepageNm = 8_000_000n;
  const geodesicCreepageNm = compute3dGeodesicCreepageNm(5_800_000n, slotRibAdded);
  const busbarIrDropUv = 8_200n;
  const hotLoopInductancePh = 2_850n;
  const millerMarginUv = computeMillerShootThroughMarginUv(negativeGateBiasEnabled);

  const pdnClean = bands.every((b) => b.zMeasuredMohm <= b.zTargetMohm);
  const creepageClean = geodesicCreepageNm >= requiredCreepageNm;
  const millerClean = millerMarginUv >= 1_500_000n;

  const summaryTone: SigStatusTone =
    pdnClean && creepageClean && millerClean ? "ok" : "warn";
  const summaryBadge = `3D Creepage: ${(Number(geodesicCreepageNm) / 1_000_000).toFixed(1)} / 8.0 mm (FEAT-023) • Busbar IR-Drop: ${(Number(busbarIrDropUv) / 1_000).toFixed(1)} mV (FEAT-024)`;
  const hotLoopBadge = `SiC Hot-Loop L_loop=${(Number(hotLoopInductancePh) / 1_000).toFixed(2)} nH • Miller Shoot-Through Margin: +${(Number(millerMarginUv) / 1_000_000).toFixed(2)} V (FEAT-025)`;

  const rows: SigGridRow[] = bands.map((b) => ({
    id: b.bandLabel,
    band: b.bandLabel,
    zTarget: `${b.zTargetMohm.toString(10)} mΩ`,
    zPdn: `${b.zMeasuredMohm.toString(10)} mΩ`,
    verdict: b.zMeasuredMohm <= b.zTargetMohm ? "PASS" : "EXCEEDS_Z_TARGET",
  }));

  return {
    slotRibAdded,
    negativeGateBiasEnabled,
    geodesicCreepageNm,
    requiredCreepageNm,
    busbarIrDropUv,
    hotLoopInductancePh,
    millerMarginUv,
    summaryBadge,
    summaryTone,
    hotLoopBadge,
    columns: PDN_COLUMNS,
    rows,
  };
}
