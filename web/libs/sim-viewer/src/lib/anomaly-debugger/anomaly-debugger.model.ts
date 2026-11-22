// Module: web/libs/sim-viewer/src/lib/anomaly-debugger/anomaly-debugger.model.ts — FEAT-106 Waveform Anomaly Attribution Models
// Invariants: Anomaly capture windows stay in ES2020 bigint picoseconds (_ps).
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";

export type AnomalySymptomId = "SW_RINGING_180MHZ" | "LOAD_STEP_DROOP" | "MILLER_GLITCH";

export interface AnomalyAttributionCase {
  readonly id: AnomalySymptomId;
  readonly netName: string;
  readonly windowStartPs: bigint;
  readonly windowEndPs: bigint;
  readonly dominantRootCause: string;
  readonly culpritEntityId: string;
  readonly rankedMitigations: readonly string[];
}

export interface AnomalyDebuggerVm {
  readonly cases: readonly AnomalyAttributionCase[];
  readonly selectedCase: AnomalyAttributionCase;
  readonly appliedMitigation: string | null;
  readonly windowBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
