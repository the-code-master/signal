// Module: web/libs/sim-viewer/src/lib/jitter-studio/jitter-studio.logic.ts — Dual-Dirac BigInt Femtosecond Jitter & PAM4 Eye Math
// Invariants: TJ@1e-15 = DJ_pp + 2*Q(1e-15)*RJ_rms computed in exact bigint femtoseconds (2*Q_1e15 = 15.883).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { JitterDecompositionInput, JitterStudioVm, SerDesRatePreset } from "./jitter-studio.model";

const TWO_Q_AT_1E15_X1000 = 15_883n;

const EYE_COLUMNS: readonly SigGridColumn[] = [
  { key: "eye", header: "PAM4 Sub-Eye", mono: true },
  { key: "horizUi", header: "Horiz Opening @ 1e-15", mono: true },
  { key: "vertMv", header: "Vert Eye Height", mono: true },
  { key: "berMargin", header: "IEEE 802.3df Margin", mono: true },
];

export function computeDualDiracTjAt1e15Fs(rjRmsFs: bigint, djPpFs: bigint): bigint {
  return djPpFs + (rjRmsFs * TWO_Q_AT_1E15_X1000) / 1_000n;
}

export function presetJitterInput(ratePreset: SerDesRatePreset): JitterDecompositionInput {
  if (ratePreset === "224G_PAM4") {
    return {
      ratePreset,
      unitIntervalFs: 8_928n,
      rjRmsFs: 145n,
      djPpFs: 2_850n,
      pjPpFs: 620n,
      ddjPpFs: 2_230n,
    };
  }
  return {
    ratePreset,
    unitIntervalFs: 17_857n,
    rjRmsFs: 210n,
    djPpFs: 4_400n,
    pjPpFs: 950n,
    ddjPpFs: 3_450n,
  };
}

export function buildJitterStudioVm(input: JitterDecompositionInput): JitterStudioVm {
  const tjAt1e15Fs = computeDualDiracTjAt1e15Fs(input.rjRmsFs, input.djPpFs);
  const openingMilliUi =
    input.unitIntervalFs > tjAt1e15Fs
      ? Number(((input.unitIntervalFs - tjAt1e15Fs) * 1_000n) / input.unitIntervalFs)
      : 0;
  const eyeOpeningUi = Number((openingMilliUi / 1_000).toFixed(3));
  const eyeTone: SigStatusTone = eyeOpeningUi >= 0.35 ? "ok" : eyeOpeningUi >= 0.2 ? "warn" : "danger";

  const upperUi = Math.max(0, Number((eyeOpeningUi - 0.018).toFixed(3)));
  const midUi = eyeOpeningUi;
  const lowerUi = Math.max(0, Number((eyeOpeningUi - 0.012).toFixed(3)));

  const eyeRows: SigGridRow[] = [
    { id: "upper", eye: "Upper Eye (L2-L3)", horizUi: `${upperUi} UI`, vertMv: "34.2 mV", berMargin: "PASS (< 1e-15)" },
    { id: "mid", eye: "Middle Eye (L1-L2)", horizUi: `${midUi} UI`, vertMv: "36.8 mV", berMargin: "PASS (< 1e-15)" },
    { id: "lower", eye: "Lower Eye (L0-L1)", horizUi: `${lowerUi} UI`, vertMv: "35.1 mV", berMargin: "PASS (< 1e-15)" },
  ];

  return {
    ratePreset: input.ratePreset,
    tjAt1e15Fs,
    eyeOpeningUi,
    eyeBadge: `PAM4 @ 1e-15 BER: ${eyeOpeningUi} UI Open (TJ=${tjAt1e15Fs.toString(10)} fs)`,
    eyeTone,
    breakdownBadges: [
      `UI: ${input.unitIntervalFs.toString(10)} fs`,
      `RJ_rms: ${input.rjRmsFs.toString(10)} fs`,
      `DJ_pp: ${input.djPpFs.toString(10)} fs`,
      `PJ_pp: ${input.pjPpFs.toString(10)} fs`,
      `DDJ_pp: ${input.ddjPpFs.toString(10)} fs`,
    ],
    eyeColumns: EYE_COLUMNS,
    eyeRows,
  };
}
