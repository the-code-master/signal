// Module: web/libs/library-studio/src/lib/datasheet-testbench/datasheet-testbench.logic.ts — FEAT-107 Testbench Overlay Logic
// Invariants: Computes SPICE-vs-Datasheet curve RMS error % across bigint bias/temp/freq operating points.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/18-universal-component-and-datasheet-repository.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { DatasheetTestbenchVm, ExtractedConditionCurve } from "./datasheet-testbench.model";

export const DEFAULT_TESTBENCH_CURVES: readonly ExtractedConditionCurve[] = [
  {
    mpn: "TPS546D24ARVFR",
    docBlake3: "b3:74c1802f9d11",
    generatedBranchId: "cow/tb-tps546d24a-typ-app",
    metricKind: "EFFICIENCY_VS_ILOAD_12V_TO_0V85",
    biasUvPoints: [12_000_000n, 12_000_000n, 12_000_000n, 12_000_000n],
    tempMkelvinPoints: [298_150n, 328_150n, 358_150n, 398_150n],
    freqHzPoints: [550_000n, 550_000n, 550_000n, 550_000n],
    datasheetRatios: [0.914, 0.908, 0.896, 0.879],
    spiceSimulatedRatios: [0.916, 0.906, 0.899, 0.876],
    rThetaJaAt100SqmmMkPerW: 11_400n,
    linkedSiliconErrataIds: ["SLVSE89B-PMBUS-SYNC"],
  },
  {
    mpn: "C3M0016120K",
    docBlake3: "b3:99e3b7104a8c",
    generatedBranchId: "cow/tb-c3m0016120k-dpt",
    metricKind: "RDS_ON_NORMALIZED_VS_TJ_800V",
    biasUvPoints: [800_000_000n, 800_000_000n, 800_000_000n, 800_000_000n],
    tempMkelvinPoints: [298_150n, 348_150n, 398_150n, 443_150n],
    freqHzPoints: [100_000n, 100_000n, 100_000n, 100_000n],
    datasheetRatios: [1.0, 1.18, 1.42, 1.69],
    spiceSimulatedRatios: [1.002, 1.176, 1.428, 1.681],
    rThetaJaAt100SqmmMkPerW: 38_000n,
    linkedSiliconErrataIds: ["WS-AN-KELVIN-SRC"],
  },
];

const OVERLAY_COLUMNS: readonly SigGridColumn[] = [
  { key: "opPoint", header: "Operating Condition (V / Tj / f)", mono: true },
  { key: "pdfVal", header: "Quarantined PDF Curve", mono: true },
  { key: "spiceVal", header: "Rust SPICE Testbench", mono: true },
  { key: "deltaPct", header: "Point Error", mono: true },
];

export function computeCurveOverlayRmsPct(
  refRatios: readonly number[],
  simRatios: readonly number[],
): number {
  const n = Math.min(refRatios.length, simRatios.length);
  if (n === 0) return 0;
  let sumSq = 0;
  for (let i = 0; i < n; i++) {
    const ref = refRatios[i];
    if (ref !== 0) {
      const rel = ((simRatios[i] - ref) / ref) * 100;
      sumSq += rel * rel;
    }
  }
  return Number(Math.sqrt(sumSq / n).toFixed(3));
}

export function buildDatasheetTestbenchVm(
  selectedMpn: string,
  presets: readonly ExtractedConditionCurve[] = DEFAULT_TESTBENCH_CURVES,
): DatasheetTestbenchVm {
  const selected = presets.find((p) => p.mpn === selectedMpn) ?? presets[0];
  const rmsPct = computeCurveOverlayRmsPct(
    selected.datasheetRatios,
    selected.spiceSimulatedRatios,
  );
  const rmsTone: SigStatusTone = rmsPct <= 1.0 ? "ok" : rmsPct <= 2.5 ? "warn" : "danger";
  const rmsBadge = `CoW ${selected.generatedBranchId} • Curve Overlay RMS Error: ${rmsPct}% (GOLD Gate <= 1.0%)`;

  const rThetaKPerW = (Number(selected.rThetaJaAt100SqmmMkPerW) / 1000).toFixed(1);
  const errataText = selected.linkedSiliconErrataIds.join(", ") || "None";
  const thermalErrataBadge = `RθJA(100mm²)=${rThetaKPerW} K/W • Linked Errata: ${errataText}`;

  const rows: SigGridRow[] = selected.datasheetRatios.map((pdfVal, idx) => {
    const spiceVal = selected.spiceSimulatedRatios[idx] ?? pdfVal;
    const vVolts = (selected.biasUvPoints[idx] / 1_000_000n).toString(10);
    const tjC = ((selected.tempMkelvinPoints[idx] - 273_150n) / 1_000n).toString(10);
    const fKhz = (selected.freqHzPoints[idx] / 1_000n).toString(10);
    const errPct = (((spiceVal - pdfVal) / pdfVal) * 100).toFixed(2);
    return {
      id: `${selected.mpn}-pt-${idx}`,
      opPoint: `${vVolts} V @ ${tjC} °C / ${fKhz} kHz`,
      pdfVal: pdfVal.toFixed(3),
      spiceVal: spiceVal.toFixed(3),
      deltaPct: `${Number(errPct) >= 0 ? "+" : ""}${errPct}%`,
    };
  });

  return {
    presets,
    selected,
    referenceCurveRmsErrorPct: rmsPct,
    rmsBadge,
    rmsTone,
    thermalErrataBadge,
    columns: OVERLAY_COLUMNS,
    rows,
  };
}
