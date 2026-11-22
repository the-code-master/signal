// Module: web/libs/mcad-studio/src/lib/brep-sheetmetal-anchors/brep-sheetmetal-anchors.model.ts — CAD_MCAD Feature & Anchor Models
// Invariants: All B-Rep depths (_nm), unfolded flat lengths (_nm), and ECAD<->MCAD sync latencies (_ps) stay in ES2020 bigint.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/mcad_cam_print.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface BRepTimelineNode {
  readonly featureId: string;
  readonly opKind: "EXTRUDE" | "FILLET" | "SHEET_METAL_FLANGE" | "GASKET_GROOVE";
  readonly primaryDepthNm: bigint;
  readonly linkedPcbAnchorId: string;
  readonly suppressed: boolean;
}

export interface BrepSheetmetalAnchorsVm {
  readonly kFactor: number;
  readonly kFactorPresets: readonly number[];
  readonly sketchRemainingDof: number;
  readonly unfoldedFlatLengthNm: bigint;
  readonly maxAnchorSyncLatencyPs: bigint;
  readonly anchorSyncBadge: string;
  readonly anchorSyncTone: SigStatusTone;
  readonly sheetMetalBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
