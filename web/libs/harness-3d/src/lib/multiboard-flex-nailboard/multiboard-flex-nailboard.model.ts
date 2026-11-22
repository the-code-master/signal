// Module: web/libs/harness-3d/src/lib/multiboard-flex-nailboard/multiboard-flex-nailboard.model.ts — FEAT-020/061/062 Models
// Invariants: Bend radii (_nm), wire cut lengths (_nm), and voltage drops (_uv) stay in ES2020 bigint.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/mcad_cam_print.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface HarnessWireBranch {
  readonly branchId: string;
  readonly wireGaugeSpec: string;
  readonly cutLengthNm: bigint;
  readonly currentNa: bigint;
  readonly voltageDropUv: bigint;
  readonly whmaAmpacityPass: boolean;
}

export interface MultiboardFlexNailboardVm {
  readonly foldProgressPct: number;
  readonly foldPresets: readonly number[];
  readonly flexBendRadiusNm: bigint;
  readonly copperThicknessNm: bigint;
  readonly outerFiberCopperStrainPct: number;
  readonly strainWithinIpc2223Limit: boolean;
  readonly enclosureCollisionCount: number;
  readonly flexAssemblyBadge: string;
  readonly flexAssemblyTone: SigStatusTone;
  readonly nailboardBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
