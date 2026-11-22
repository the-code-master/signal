// Module: web/libs/specs/src/lib/datasheet-drift/datasheet-drift.model.ts — FEAT-078 Datasheet Revision Drift Models
// Invariants: Tracks Rev A -> Rev B pinout, AbsMax (_uv/_mkelvin bigint), and derating deltas for parts.lock.json.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, contracts/proto/signal/v1/ai_rag_formal.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface DatasheetRevisionNotice {
  readonly noticeId: string;
  readonly mpn: string;
  readonly oldRevisionLabel: string;
  readonly newRevisionLabel: string;
  readonly oldDocBlake3: string;
  readonly newDocBlake3: string;
  readonly changedPinCount: number;
  readonly tightenedAbsMaxCount: number;
  readonly oldAbsMaxUv: bigint;
  readonly newAbsMaxUv: bigint;
  readonly oldTjMaxMkelvin: bigint;
  readonly newTjMaxMkelvin: bigint;
  readonly parameterDeltaSummaries: readonly string[];
}

export interface DatasheetDriftVm {
  readonly notices: readonly DatasheetRevisionNotice[];
  readonly selectedNotice: DatasheetRevisionNotice;
  readonly acknowledgedIds: ReadonlySet<string>;
  readonly unacknowledgedCount: number;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly hashCompareBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
