// Module: web/libs/academy/src/lib/quick-calc-hud/quick-calc-hud.logic.ts — Pure BigInt 4/5/6-Band, EIA-96 SMD, Divider & LED Calculators
// Invariants: Exact IEC 60062/60063 bigint milliohm (_mohm) and microvolt (_uv) arithmetic; zero floating-point drift on SI outputs.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type {
  DividerCalcResult,
  LedLimiterCalcResult,
  QuickCalcHudVm,
  ResistorBandColor,
  ResistorDecodeResult,
} from "./quick-calc-hud.model";

const DIGIT_MAP: Partial<Readonly<Record<ResistorBandColor, bigint>>> = {
  black: 0n, brown: 1n, red: 2n, orange: 3n, yellow: 4n,
  green: 5n, blue: 6n, violet: 7n, grey: 8n, white: 9n,
};

// Multiplier scaled in milliohms per base digit unit (silver=10 mΩ, gold=100 mΩ, black=1_000 mΩ = 1 Ω):
const MULT_MOHM_MAP: Readonly<Record<ResistorBandColor, bigint>> = {
  silver: 10n, gold: 100n, black: 1_000n, brown: 10_000n, red: 100_000n,
  orange: 1_000_000n, yellow: 10_000_000n, green: 100_000_000n,
  blue: 1_000_000_000n, violet: 10_000_000_000n, grey: 100_000_000_000n, white: 1_000_000_000_000n,
};

const TOLERANCE_PCT_MAP: Partial<Readonly<Record<ResistorBandColor, number>>> = {
  brown: 1, red: 2, green: 0.5, blue: 0.25, violet: 0.1, grey: 0.05, gold: 5, silver: 10,
};

const TEMPCO_PPM_MAP: Partial<Readonly<Record<ResistorBandColor, number>>> = {
  brown: 100, red: 50, orange: 15, yellow: 25, blue: 10, violet: 5,
};

const EIA96_MANTISSA: readonly number[] = [
  100, 102, 105, 107, 110, 113, 115, 118, 121, 124, 127, 130, 133, 137, 140, 143,
  147, 150, 154, 158, 162, 165, 169, 174, 178, 182, 187, 191, 196, 200, 205, 210,
  215, 221, 226, 232, 237, 243, 249, 255, 261, 267, 274, 280, 287, 294, 301, 309,
  316, 324, 332, 340, 348, 357, 365, 374, 383, 392, 402, 412, 422, 432, 442, 453,
  464, 475, 487, 499, 511, 523, 536, 549, 562, 576, 590, 604, 619, 634, 649, 665,
  681, 698, 715, 732, 750, 768, 787, 806, 825, 845, 866, 887, 909, 931, 953, 976,
];

// Multiplier in milliohms per EIA-96 3-digit integer (where 'A'=1x => 100 -> 100 Ω = 100_000 mΩ => * 1_000 mΩ):
const EIA96_MULT_MOHM: Readonly<Record<string, bigint>> = {
  Z: 1n, Y: 10n, R: 10n, X: 100n, S: 100n, A: 1_000n,
  B: 10_000n, H: 10_000n, C: 100_000n, D: 1_000_000n, E: 10_000_000n, F: 100_000_000n,
};

const E24_BASE: readonly bigint[] = [
  10n, 11n, 12n, 13n, 15n, 16n, 18n, 20n, 22n, 24n, 27n, 30n,
  33n, 36n, 39n, 43n, 47n, 51n, 56n, 62n, 68n, 75n, 82n, 91n,
];

export function formatResistanceMohm(mohm: bigint): string {
  if (mohm >= 1_000_000_000n) {
    return `${(mohm / 1_000_000_000n).toString(10)} MΩ (${mohm.toString(10)} mΩ)`;
  }
  if (mohm >= 1_000_000n) {
    const kWhole = mohm / 1_000_000n;
    const kRem = (mohm % 1_000_000n) / 100_000n;
    return kRem > 0n ? `${kWhole.toString(10)}.${kRem.toString(10)} kΩ` : `${kWhole.toString(10)} kΩ`;
  }
  return `${(mohm / 1_000n).toString(10)} Ω`;
}

export function decodeColorBandResistor(bands: readonly ResistorBandColor[]): ResistorDecodeResult {
  if (bands.length !== 4 && bands.length !== 5 && bands.length !== 6) {
    return {
      valid: false,
      nominalMohm: 0n,
      tolerancePct: 0,
      tempcoPpmPerK: null,
      formattedSummary: "Provide 4, 5, or 6 IEC color bands",
      tone: "warn",
    };
  }
  const d1 = DIGIT_MAP[bands[0]];
  const d2 = DIGIT_MAP[bands[1]];
  if (d1 === undefined || d2 === undefined) {
    return { valid: false, nominalMohm: 0n, tolerancePct: 0, tempcoPpmPerK: null, formattedSummary: "Invalid significant digit band", tone: "danger" };
  }

  let sigDigits = d1 * 10n + d2;
  let multIdx = 2;
  let tolIdx = 3;
  if (bands.length >= 5) {
    const d3 = DIGIT_MAP[bands[2]];
    if (d3 === undefined) {
      return { valid: false, nominalMohm: 0n, tolerancePct: 0, tempcoPpmPerK: null, formattedSummary: "Invalid 3rd digit band", tone: "danger" };
    }
    sigDigits = sigDigits * 10n + d3;
    multIdx = 3;
    tolIdx = 4;
  }

  const multMohm = MULT_MOHM_MAP[bands[multIdx]];
  const tolerancePct = TOLERANCE_PCT_MAP[bands[tolIdx]] ?? 20;
  const tempcoPpmPerK = bands.length === 6 ? (TEMPCO_PPM_MAP[bands[5]] ?? null) : null;
  const nominalMohm = sigDigits * multMohm;
  const tcrSuffix = tempcoPpmPerK !== null ? `, ${tempcoPpmPerK} ppm/K` : "";

  return {
    valid: true,
    nominalMohm,
    tolerancePct,
    tempcoPpmPerK,
    formattedSummary: `${formatResistanceMohm(nominalMohm)} ±${tolerancePct}%${tcrSuffix}`,
    tone: "ok",
  };
}

export function decodeEia96SmdCode(rawCode: string): ResistorDecodeResult {
  const code = rawCode.trim().toUpperCase();
  const match = /^(\d{2})([A-Z])$/.exec(code);
  if (!match) {
    return { valid: false, nominalMohm: 0n, tolerancePct: 1, tempcoPpmPerK: null, formattedSummary: "Expected EIA-96 code (e.g. 01C)", tone: "warn" };
  }
  const idx = Number(match[1]);
  const multMohm = EIA96_MULT_MOHM[match[2]];
  if (idx < 1 || idx > 96 || multMohm === undefined) {
    return { valid: false, nominalMohm: 0n, tolerancePct: 1, tempcoPpmPerK: null, formattedSummary: `Invalid EIA-96 index/multiplier '${code}'`, tone: "danger" };
  }
  const nominalMohm = BigInt(EIA96_MANTISSA[idx - 1]) * multMohm;
  return {
    valid: true,
    nominalMohm,
    tolerancePct: 1,
    tempcoPpmPerK: null,
    formattedSummary: `${code} -> ${formatResistanceMohm(nominalMohm)} ±1%`,
    tone: "ok",
  };
}

export function computeVoltageDivider(
  vInUv: bigint,
  rTopMohm: bigint,
  rBotMohm: bigint,
): DividerCalcResult {
  const sumMohm = rTopMohm + rBotMohm;
  if (sumMohm <= 0n) {
    return { vOutUv: 0n, branchCurrentNa: 0n, formattedSummary: "R_top + R_bot must be > 0" };
  }
  const vOutUv = (vInUv * rBotMohm) / sumMohm;
  const branchCurrentNa = (vInUv * 1_000_000n) / sumMohm;
  return {
    vOutUv,
    branchCurrentNa,
    formattedSummary: `V_out = ${(vOutUv / 1_000n).toString(10)} mV (${vOutUv.toString(10)} uV) | I_div = ${(branchCurrentNa / 1_000n).toString(10)} uA`,
  };
}

function findNearestE24Mohm(targetMohm: bigint): bigint {
  if (targetMohm <= 1_000n) return 1_000n;
  let decade = 100n;
  while (decade * 100n <= targetMohm) {
    decade *= 10n;
  }
  let best = E24_BASE[0] * decade;
  for (const m of E24_BASE) {
    const cand = m * decade;
    if (cand >= targetMohm) {
      best = cand;
      break;
    }
    best = E24_BASE[0] * decade * 10n;
  }
  return best;
}

export function computeLedSeriesResistor(
  vSupplyUv: bigint,
  vForwardUv: bigint,
  iTargetNa: bigint,
): LedLimiterCalcResult {
  if (vSupplyUv <= vForwardUv || iTargetNa <= 0n) {
    return { exactMohm: 0n, nearestE24Mohm: 0n, powerDissUw: 0n, formattedSummary: "V_supply must exceed V_forward" };
  }
  const dropUv = vSupplyUv - vForwardUv;
  const exactMohm = (dropUv * 1_000_000n) / iTargetNa;
  const nearestE24Mohm = findNearestE24Mohm(exactMohm);
  const powerDissUw = (dropUv * iTargetNa) / 1_000_000_000n;
  return {
    exactMohm,
    nearestE24Mohm,
    powerDissUw,
    formattedSummary: `R_exact=${formatResistanceMohm(exactMohm)} -> E24=${formatResistanceMohm(nearestE24Mohm)} (P=${(powerDissUw / 1_000n).toString(10)} mW)`,
  };
}

export function buildQuickCalcHudVm(
  bandsCsv: string,
  smdCode: string,
  vInUv: bigint,
  vSupplyUv: bigint,
): QuickCalcHudVm {
  const parsedBands = bandsCsv
    .split(",")
    .map((b) => b.trim().toLowerCase() as ResistorBandColor)
    .filter((b) => b.length > 0);
  return {
    bandResult: decodeColorBandResistor(parsedBands),
    smdResult: decodeEia96SmdCode(smdCode),
    dividerResult: computeVoltageDivider(vInUv, 30_000_000n, 10_000_000n),
    ledResult: computeLedSeriesResistor(vSupplyUv, 2_000_000n, 20_000_000n),
  };
}
