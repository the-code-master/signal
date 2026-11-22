// Module: web/libs/schematic/src/lib/circuit-explainer/circuit-explainer.model.ts — FEAT-101 Circuit Explainer Models
// Invariants: Stage bounding boxes (_nm) and corner frequencies (_hz) stay in ES2020 bigint.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";

export interface FunctionalStageBlock {
  readonly stageId: string;
  readonly topologyName: string;
  readonly symbolicTransferFn: string;
  readonly cornerFreqHz: bigint;
  readonly minXNm: bigint;
  readonly minYNm: bigint;
  readonly maxXNm: bigint;
  readonly maxYNm: bigint;
}

export interface CircuitExplainerVm {
  readonly stages: readonly FunctionalStageBlock[];
  readonly selectedStage: FunctionalStageBlock;
  readonly signalFlowSummary: string;
  readonly bboxBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
