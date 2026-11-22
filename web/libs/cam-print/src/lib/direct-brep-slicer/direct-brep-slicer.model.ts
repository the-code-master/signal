// Module: web/libs/cam-print/src/lib/direct-brep-slicer/direct-brep-slicer.model.ts — PRINT_3D Direct-B-Rep Slicer Models
// Invariants: Layer heights (_nm), build Z heights (_nm), and print durations (_ps) stay in ES2020 bigint.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/mcad_cam_print.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type AdditiveProcessMode = "FDM_FFF" | "SLA_MSLA" | "SLS_MJF" | "DMLS_METAL_LPBF";

export interface DirectBrepSlicerVm {
  readonly process: AdditiveProcessMode;
  readonly processes: readonly AdditiveProcessMode[];
  readonly layerHeightNm: bigint;
  readonly totalLayerCount: number;
  readonly previewLayerIndex: number;
  readonly gyroidInfillPct: number;
  readonly treeSupportsEnabled: boolean;
  readonly thermalWarpPrecomp: boolean;
  readonly slicerBadge: string;
  readonly slicerTone: SigStatusTone;
  readonly dispatchBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
