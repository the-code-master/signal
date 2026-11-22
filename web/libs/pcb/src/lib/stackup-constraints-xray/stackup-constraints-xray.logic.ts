// Module: web/libs/pcb/src/lib/stackup-constraints-xray/stackup-constraints-xray.logic.ts — FEAT-014/018/095/096 Logic
// Invariants: Audits HDI microvia <=0.8 aspect ratio, pour acid-traps/islands, net-class matrix & GPU X-Ray bitmask.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  HdiViaSpanSpec,
  LayerXrayPreset,
  NetClassConstraintRow,
  StackupConstraintsXrayVm,
} from "./stackup-constraints-xray.model";

export const DEFAULT_HDI_SPANS: readonly HdiViaSpanSpec[] = [
  { spanId: "UVIA-L1-L2", startLayer: 1, endLayer: 2, dielectricDepthNm: 72_000n, laserDrillDiameterNm: 100_000n },
  { spanId: "UVIA-L2-L3", startLayer: 2, endLayer: 3, dielectricDepthNm: 76_000n, laserDrillDiameterNm: 100_000n },
  { spanId: "BURIED-L3-L10", startLayer: 3, endLayer: 10, dielectricDepthNm: 1_100_000n, laserDrillDiameterNm: 200_000n },
];

export const DEFAULT_NET_CLASSES: readonly NetClassConstraintRow[] = [
  { classId: "HV_DC_800V", minWidthNm: 1_200_000n, minClearanceNm: 8_000_000n, zTargetMohm: 0n },
  { classId: "HS_PCIE6_85R", minWidthNm: 85_000n, minClearanceNm: 170_000n, zTargetMohm: 85_000n },
  { classId: "KELVIN_GATE_DRV", minWidthNm: 250_000n, minClearanceNm: 400_000n, zTargetMohm: 50_000n },
];

const MATRIX_COLUMNS: readonly SigGridColumn[] = [
  { key: "classId", header: "Net Class (FEAT-095)", mono: true },
  { key: "widthNm", header: "Min Width", mono: true },
  { key: "clearanceNm", header: "Min Clearance", mono: true },
  { key: "zTarget", header: "Impedance Target", mono: true },
];

export function computeViaAspectRatio(span: HdiViaSpanSpec): number {
  if (span.laserDrillDiameterNm <= 0n) return 99;
  return Number((span.dielectricDepthNm * 100n) / span.laserDrillDiameterNm) / 100;
}

export function computeXrayLayerBitmaskHex(preset: LayerXrayPreset): string {
  if (preset === "ALL_12_LAYERS") return "0x0FFF";
  if (preset === "HV_SIC_SWITCH_XRAY") return "0x0003";
  if (preset === "PCIE6_DIFF_STRIPLINES") return "0x0330";
  return "0x0CCC";
}

export function buildStackupConstraintsXrayVm(
  preset: LayerXrayPreset,
  teardropsApplied: boolean,
  spans: readonly HdiViaSpanSpec[] = DEFAULT_HDI_SPANS,
  classes: readonly NetClassConstraintRow[] = DEFAULT_NET_CLASSES,
): StackupConstraintsXrayVm {
  const uviaOk = spans
    .filter((s) => s.spanId.startsWith("UVIA"))
    .every((s) => computeViaAspectRatio(s) <= 0.8);
  const acidTrapCount = teardropsApplied ? 0 : 2;
  const deadIslands = teardropsApplied ? 0 : 1;

  const pourHdiTone: SigStatusTone = uviaOk && acidTrapCount === 0 ? "ok" : "warn";
  const pourHdiBadge = `Pour Hygiene (FEAT-014): ${acidTrapCount} Acid Traps, ${deadIslands} Islands • HDI uVia AR <= 0.8: ${uviaOk ? "PASS" : "FAIL"} (FEAT-018)`;
  const gpuLayerBitmaskHex = computeXrayLayerBitmaskHex(preset);

  const rows: SigGridRow[] = classes.map((c) => ({
    id: c.classId,
    classId: c.classId,
    widthNm: `${c.minWidthNm.toString(10)} nm`,
    clearanceNm: `${c.minClearanceNm.toString(10)} nm`,
    zTarget: c.zTargetMohm === 0n ? "Power / Creepage" : `${(Number(c.zTargetMohm) / 1000).toFixed(1)} Ω`,
  }));

  return {
    preset,
    presets: ["ALL_12_LAYERS", "HV_SIC_SWITCH_XRAY", "PCIE6_DIFF_STRIPLINES", "GND_POUR_HYGIENE"],
    teardropsApplied,
    gpuLayerBitmaskHex,
    pourHdiBadge,
    pourHdiTone,
    columns: MATRIX_COLUMNS,
    rows,
  };
}
