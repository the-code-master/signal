// Module: web/libs/cam-print/src/lib/cnc-toolpath-studio/cnc-toolpath-studio.model.ts — CAM_CNC 5-Axis Toolpath Models
// Invariants: Tool diameters (_nm), stepovers (_nm), RPM, feedrates (_nm_per_min), and cycle times (_ps) stay in ES2020 bigint.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/mcad_cam_print.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type CncPostProfile = "FANUC_31I_5X" | "SIEMENS_840D" | "HAAS_UMC750";

export interface CncOperationRow {
  readonly operationId: string;
  readonly strategy: "ADAPTIVE_2P5D" | "ADAPTIVE_3AXIS" | "INDEXED_3PLUS2" | "SIMULTANEOUS_5AXIS";
  readonly toolDiameterNm: bigint;
  readonly stepoverNm: bigint;
  readonly spindleRpm: bigint;
  readonly feedrateNmPerMin: bigint;
}

export interface CncToolpathStudioVm {
  readonly postProfile: CncPostProfile;
  readonly postProfiles: readonly CncPostProfile[];
  readonly scrubProgressPct: number;
  readonly scrubPresets: readonly number[];
  readonly dexelRemovedPct: number;
  readonly holderGougeCount: number;
  readonly estimatedCycleTimePs: bigint;
  readonly gcodeSftpDigest: string;
  readonly gougeBadge: string;
  readonly gougeTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
