// Module: web/libs/sim-viewer/src/lib/breaking-point/breaking-point.model.ts — FEAT-102 Stress Breaking-Point Models
// Invariants: Ambient temperature (_mkelvin) and surge voltage (_uv) stay in ES2020 bigint.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface ComponentStressEnvelope {
  readonly entityId: string;
  readonly failureMechanism: string;
  readonly ratedSurgeUv: bigint;
  readonly maxTjMkelvin: bigint;
  readonly thermalRisePerVoltMk: bigint;
}

export interface BreakingPointVm {
  readonly ambientTempMkelvin: bigint;
  readonly supplySurgeUv: bigint;
  readonly firstFailingEntityId: string;
  readonly failureMechanism: string;
  readonly minHeadroomPct: number;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
