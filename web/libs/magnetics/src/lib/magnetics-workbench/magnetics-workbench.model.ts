// Module: web/libs/magnetics/src/lib/magnetics-workbench/magnetics-workbench.model.ts — FEAT-021 Magnetics Studio Models
// Invariants: Flux density (_ut), air-gap (_nm), switching frequency (_hz), and losses (_mw) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type WindingGeometryKind = "PLANAR_HEAVY_CU_INTERLEAVED" | "LITZ_BOBBIN_E_CORE";

export interface MagneticCoreCandidate {
  readonly coreId: string;
  readonly materialName: string;
  readonly bSatUt: bigint;
  readonly effectiveAreaSqNm: bigint;
}

export interface MagneticsWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly windingKind: WindingGeometryKind;
  readonly windingKinds: readonly WindingGeometryKind[];
  readonly airGapNm: bigint;
  readonly bPeakUt: bigint;
  readonly saturationMarginPct: number;
  readonly igseCoreLossMw: bigint;
  readonly dowellAcCopperLossMw: bigint;
  readonly lossBalanceBadge: string;
  readonly saturationTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
