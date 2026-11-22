// Module: web/libs/library-studio/src/lib/padstack-footprint-step/padstack-footprint-step.model.ts — FEAT-076 & FEAT-071 Models
// Invariants: All padstack, stencil, drill, backdrill, and IPC-7351C dimensions stay in ES2020 bigint nanometers (_nm).
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type Ipc7351Density = "MOST" | "NOMINAL" | "LEAST";

export interface PadstackSculptorInput {
  readonly copperWidthNm: bigint;
  readonly copperHeightNm: bigint;
  readonly drillDiameterNm: bigint;
  readonly stencilThicknessNm: bigint;
  readonly pasteContractionNm: bigint;
  readonly boardThicknessNm: bigint;
  readonly signalLayerDepthNm: bigint;
  readonly backdrillEnabled: boolean;
}

export interface PadstackMetricsResult {
  readonly annularRingNm: bigint;
  readonly ipc7525AreaRatio: number;
  readonly backdrillStubNm: bigint;
  readonly ipc7525Passed: boolean;
}

export interface PadstackFootprintStepVm {
  readonly density: Ipc7351Density;
  readonly padMetrics: PadstackMetricsResult;
  readonly padstackBadge: string;
  readonly padstackTone: SigStatusTone;
  readonly courtyardBadge: string;
  readonly stepOverlapBadge: string;
  readonly stepOverlapTone: SigStatusTone;
  readonly layerColumns: readonly SigGridColumn[];
  readonly layerRows: readonly SigGridRow[];
}
