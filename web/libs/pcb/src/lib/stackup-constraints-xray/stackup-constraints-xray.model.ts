// Module: web/libs/pcb/src/lib/stackup-constraints-xray/stackup-constraints-xray.model.ts — FEAT-014/018/095/096 Models
// Invariants: Via dimensions (_nm), net-class clearances (_nm), and impedances (_mohm) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type LayerXrayPreset = "ALL_12_LAYERS" | "HV_SIC_SWITCH_XRAY" | "PCIE6_DIFF_STRIPLINES" | "GND_POUR_HYGIENE";

export interface HdiViaSpanSpec {
  readonly spanId: string;
  readonly startLayer: number;
  readonly endLayer: number;
  readonly dielectricDepthNm: bigint;
  readonly laserDrillDiameterNm: bigint;
}

export interface NetClassConstraintRow {
  readonly classId: string;
  readonly minWidthNm: bigint;
  readonly minClearanceNm: bigint;
  readonly zTargetMohm: bigint;
}

export interface StackupConstraintsXrayVm {
  readonly preset: LayerXrayPreset;
  readonly presets: readonly LayerXrayPreset[];
  readonly teardropsApplied: boolean;
  readonly gpuLayerBitmaskHex: string;
  readonly pourHdiBadge: string;
  readonly pourHdiTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
