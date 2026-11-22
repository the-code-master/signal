// Module: web/libs/library-studio/src/lib/datasheet-testbench/datasheet-testbench.model.ts — FEAT-107 Testbench Wizard Models
// Invariants: Models ConditionCurveSet in exact bigint (_uv, _mkelvin, _hz) and SPICE vs PDF RMS overlay error.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, contracts/proto/signal/v1/ai_rag_formal.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface ExtractedConditionCurve {
  readonly mpn: string;
  readonly docBlake3: string;
  readonly generatedBranchId: string;
  readonly metricKind: string;
  readonly biasUvPoints: readonly bigint[];
  readonly tempMkelvinPoints: readonly bigint[];
  readonly freqHzPoints: readonly bigint[];
  readonly datasheetRatios: readonly number[];
  readonly spiceSimulatedRatios: readonly number[];
  readonly rThetaJaAt100SqmmMkPerW: bigint;
  readonly linkedSiliconErrataIds: readonly string[];
}

export interface DatasheetTestbenchVm {
  readonly presets: readonly ExtractedConditionCurve[];
  readonly selected: ExtractedConditionCurve;
  readonly referenceCurveRmsErrorPct: number;
  readonly rmsBadge: string;
  readonly rmsTone: SigStatusTone;
  readonly thermalErrataBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
