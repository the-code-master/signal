// Module: web/libs/pcb/src/lib/si-em-rf-inspector/si-em-rf-inspector.logic.ts — FEAT-011/013/016/019/087/089 SI/EM/RF Logic
// Invariants: Computes fiber-weave skew, quarter-wave via-stub notch f_stub, RF dimensions & spatial TDR Z0(d) profile.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  RfTopologyPreset,
  SiEmRfInspectorVm,
  TdrSpatialSegment,
} from "./si-em-rf-inspector.model";

export const DEFAULT_TDR_PROFILE: readonly TdrSpatialSegment[] = [
  {
    segmentId: "SEG-BGA-BREAKOUT",
    startDistanceNm: 0n,
    endDistanceNm: 3_500_000n,
    z0Mohm: 84_200n,
    returnDetourNm: 0n,
  },
  {
    segmentId: "SEG-VIA-TRANSITION-L1-L10",
    startDistanceNm: 3_500_000n,
    endDistanceNm: 5_100_000n,
    z0Mohm: 78_600n,
    returnDetourNm: 180_000n,
  },
  {
    segmentId: "SEG-TRUNK-STRIPLINE",
    startDistanceNm: 5_100_000n,
    endDistanceNm: 48_000_000n,
    z0Mohm: 85_100n,
    returnDetourNm: 0n,
  },
];

const TDR_COLUMNS: readonly SigGridColumn[] = [
  { key: "segmentId", header: "Spatial TDR Span (FEAT-089)", mono: true },
  { key: "spanMm", header: "d [mm]", mono: true },
  { key: "z0", header: "Z0(d)", mono: true },
  { key: "returnPath", header: "Return Detour (FEAT-016)", mono: true },
];

export function computeFiberWeaveSkewFs(traceLengthNm: bigint, rotated10Deg: boolean): bigint {
  const mm = traceLengthNm / 1_000_000n;
  return rotated10Deg ? mm * 14n : mm * 185n;
}

export function computeViaStubQuarterWaveHz(stubLengthNm: bigint): bigint {
  if (stubLengthNm <= 0n) return 200_000_000_000n;
  const cOver4SqrtErNmHz = 38_500_000_000_000_000n;
  return cOver4SqrtErNmHz / stubLengthNm;
}

export function synthesizeRfQuarterWaveNm(preset: RfTopologyPreset): bigint {
  if (preset === "MICROSTRIP_50R") return 8_400_000n;
  if (preset === "WILKINSON_2WAY_28GHZ") return 1_620_000n;
  return 590_000n;
}

export function buildSiEmRfInspectorVm(
  weaveRotated10Deg: boolean,
  backdrillEnabled: boolean,
  rfPreset: RfTopologyPreset,
  emPhaseDeg: number,
  segments: readonly TdrSpatialSegment[] = DEFAULT_TDR_PROFILE,
): SiEmRfInspectorVm {
  const weaveSkewFs = computeFiberWeaveSkewFs(48_000_000n, weaveRotated10Deg);
  const stubLengthNm = backdrillEnabled ? 250_000n : 1_850_000n;
  const stubResonanceHz = computeViaStubQuarterWaveHz(stubLengthNm);
  const stubGhz = Number(stubResonanceHz / 100_000_000n) / 10;

  const siClean = weaveSkewFs <= 1_000n && stubResonanceHz >= 56_000_000_000n;
  const siGateTone: SigStatusTone = siClean ? "ok" : "warn";
  const siGateBadge = `Weave Skew: ${weaveSkewFs.toString(10)} fs (FEAT-011) • Via-Stub Notch: ${stubGhz} GHz (FEAT-013)`;

  const rfLenUm = (synthesizeRfQuarterWaveNm(rfPreset) / 1_000n).toString(10);
  const rfEmBadge = `RF (${rfPreset}): λ/4=${rfLenUm} um (FEAT-019) • 3D EM Poynting Phase=${emPhaseDeg}° (FEAT-087)`;

  const rows: SigGridRow[] = segments.map((s) => {
    const d0 = (Number(s.startDistanceNm) / 1_000_000).toFixed(1);
    const d1 = (Number(s.endDistanceNm) / 1_000_000).toFixed(1);
    const zOhm = (Number(s.z0Mohm) / 1_000).toFixed(1);
    const detourUm = s.returnDetourNm / 1_000n;
    return {
      id: s.segmentId,
      segmentId: s.segmentId,
      spanMm: `${d0}..${d1} mm`,
      z0: `${zOhm} Ω`,
      returnPath: detourUm === 0n ? "0 um (Continuous GND)" : `${detourUm.toString(10)} um (Stitch Via Needed)`,
    };
  });

  return {
    weaveRotated10Deg,
    backdrillEnabled,
    rfPreset,
    rfPresets: ["MICROSTRIP_50R", "WILKINSON_2WAY_28GHZ", "PATCH_ANTENNA_77GHZ"],
    emPhaseDeg,
    weaveSkewFs,
    stubResonanceHz,
    siGateBadge,
    siGateTone,
    rfEmBadge,
    columns: TDR_COLUMNS,
    rows,
  };
}
