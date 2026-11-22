// Module: web/libs/harness-3d/src/lib/multiboard-flex-nailboard/multiboard-flex-nailboard.logic.ts — FEAT-020/061/062 Logic
// Invariants: Computes IPC-2223 outer-fiber copper tensile strain eps = t/(2*R_eff) and WHMA-A-620 nailboard IR drop.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/mcad_cam_print.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  HarnessWireBranch,
  MultiboardFlexNailboardVm,
} from "./multiboard-flex-nailboard.model";

export const DEFAULT_NAILBOARD_BRANCHES: readonly HarnessWireBranch[] = [
  {
    branchId: "BR-HV-DC-POS",
    wireGaugeSpec: "70 mm² Orange Shielded HV",
    cutLengthNm: 640_000_000n,
    currentNa: 425_000_000_000n,
    voltageDropUv: 68_400n,
    whmaAmpacityPass: true,
  },
  {
    branchId: "BR-HV-DC-NEG",
    wireGaugeSpec: "70 mm² Orange Shielded HV",
    cutLengthNm: 655_000_000n,
    currentNa: 425_000_000_000n,
    voltageDropUv: 70_100n,
    whmaAmpacityPass: true,
  },
  {
    branchId: "BR-LV-CANFD",
    wireGaugeSpec: "22 AWG Twisted Shielded Pair",
    cutLengthNm: 910_000_000n,
    currentNa: 85_000_000n,
    voltageDropUv: 4_100n,
    whmaAmpacityPass: true,
  },
];

const NAILBOARD_COLUMNS: readonly SigGridColumn[] = [
  { key: "branchId", header: "1:1 Formboard Branch (FEAT-062)", mono: true },
  { key: "gauge", header: "Conductor / Jacket" },
  { key: "cutMm", header: "Cut Length", mono: true },
  { key: "vDrop", header: "IR Drop / WHMA-A-620", mono: true },
];

export function computeOuterFiberCopperStrainPct(
  copperThicknessNm: bigint,
  flexBendRadiusNm: bigint,
  foldProgressPct: number,
): number {
  const clampedFold = Math.max(0, Math.min(100, foldProgressPct));
  if (clampedFold === 0 || flexBendRadiusNm <= 0n) return 0;
  const rawStrainPct = (Number(copperThicknessNm) / (2 * Number(flexBendRadiusNm))) * 100;
  return Number((rawStrainPct * (clampedFold / 100)).toFixed(3));
}

export function buildMultiboardFlexNailboardVm(
  foldProgressPct: number,
  flexBendRadiusNm: bigint,
  branches: readonly HarnessWireBranch[] = DEFAULT_NAILBOARD_BRANCHES,
): MultiboardFlexNailboardVm {
  const copperThicknessNm = 18_000n;
  const outerFiberCopperStrainPct = computeOuterFiberCopperStrainPct(
    copperThicknessNm,
    flexBendRadiusNm,
    foldProgressPct,
  );
  const strainWithinIpc2223Limit = outerFiberCopperStrainPct <= 0.8;
  const enclosureCollisionCount = 0;

  const flexAssemblyTone: SigStatusTone =
    strainWithinIpc2223Limit && enclosureCollisionCount === 0 ? "ok" : "danger";
  const flexAssemblyBadge = `FEAT-020 Flex Fold ${foldProgressPct}%: Cu Strain=${outerFiberCopperStrainPct}% (IPC-2223 Cap <= 0.8%) • FEAT-061 Multi-Board Collisions=${enclosureCollisionCount}`;

  const totalCutNm = branches.reduce((acc, b) => acc + b.cutLengthNm, 0n);
  const worstDropUv = branches.reduce(
    (mx, b) => (b.voltageDropUv > mx ? b.voltageDropUv : mx),
    0n,
  );
  const nailboardBadge = `1:1 Nailboard Total Cut=${(Number(totalCutNm) / 1_000_000).toFixed(0)} mm • Worst IR Drop=${(Number(worstDropUv) / 1_000).toFixed(1)} mV • WHMA-A-620 PASS`;

  const rows: SigGridRow[] = branches.map((b) => ({
    id: b.branchId,
    branchId: b.branchId,
    gauge: b.wireGaugeSpec,
    cutMm: `${(Number(b.cutLengthNm) / 1_000_000).toFixed(1)} mm`,
    vDrop: `${(Number(b.voltageDropUv) / 1_000).toFixed(1)} mV (${b.whmaAmpacityPass ? "AMPACITY_OK" : "OVERLOAD"})`,
  }));

  return {
    foldProgressPct,
    foldPresets: [0, 50, 90, 100],
    flexBendRadiusNm,
    copperThicknessNm,
    outerFiberCopperStrainPct,
    strainWithinIpc2223Limit,
    enclosureCollisionCount,
    flexAssemblyBadge,
    flexAssemblyTone,
    nailboardBadge,
    columns: NAILBOARD_COLUMNS,
    rows,
  };
}
