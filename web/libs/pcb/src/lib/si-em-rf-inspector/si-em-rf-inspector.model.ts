// Module: web/libs/pcb/src/lib/si-em-rf-inspector/si-em-rf-inspector.model.ts — FEAT-011/013/016/019/087/089 SI/EM/RF Models
// Invariants: All lengths (_nm), frequencies (_hz), delays (_fs), and impedances (_mohm) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type RfTopologyPreset = "MICROSTRIP_50R" | "WILKINSON_2WAY_28GHZ" | "PATCH_ANTENNA_77GHZ";

export interface TdrSpatialSegment {
  readonly segmentId: string;
  readonly startDistanceNm: bigint;
  readonly endDistanceNm: bigint;
  readonly z0Mohm: bigint;
  readonly returnDetourNm: bigint;
}

export interface SiEmRfInspectorVm {
  readonly weaveRotated10Deg: boolean;
  readonly backdrillEnabled: boolean;
  readonly rfPreset: RfTopologyPreset;
  readonly rfPresets: readonly RfTopologyPreset[];
  readonly emPhaseDeg: number;
  readonly weaveSkewFs: bigint;
  readonly stubResonanceHz: bigint;
  readonly siGateBadge: string;
  readonly siGateTone: SigStatusTone;
  readonly rfEmBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
