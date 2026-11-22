// Module: web/libs/specs/src/lib/bom-consolidation/bom-consolidation.model.ts — FEAT-108 BOM Feeder Consolidator Models
// Invariants: Models SMT passive/package merge groups with bigint _mohm/_uv ratings and Saltation-Adjoint shift caps.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, contracts/proto/signal/v1/ai_rag_formal.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface BomMergeProposal {
  readonly proposalId: string;
  readonly sourceMpn: string;
  readonly targetCanonicalMpn: string;
  readonly packageCode: string;
  readonly ratedVoltageUv: bigint;
  readonly resistanceMohm: bigint;
  readonly affectedRefdes: readonly string[];
  readonly adjointMetricShiftPct: number;
}

export interface BomConsolidationVm {
  readonly maxAllowedShiftPct: number;
  readonly uniqueLineItemsBefore: number;
  readonly uniqueLineItemsAfter: number;
  readonly smtFeederSlotsSaved: number;
  readonly adjointZeroShiftVerified: boolean;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
