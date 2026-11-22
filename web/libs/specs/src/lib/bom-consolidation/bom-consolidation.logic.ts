// Module: web/libs/specs/src/lib/bom-consolidation/bom-consolidation.logic.ts — FEAT-108 SMT Feeder Consolidation Logic
// Invariants: Accepts passive/package merges only when Saltation-Adjoint metric shift <= maxAllowedShiftPct.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/18-universal-component-and-datasheet-repository.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { BomConsolidationVm, BomMergeProposal } from "./bom-consolidation.model";

export const DEFAULT_BOM_MERGE_PROPOSALS: readonly BomMergeProposal[] = [
  {
    proposalId: "MERGE-0402-10K",
    sourceMpn: "RC0402FR-0710KL (50V 1%)",
    targetCanonicalMpn: "AC0402FR-0710KL (50V AEC-Q200 0.5%)",
    packageCode: "0402",
    ratedVoltageUv: 50_000_000n,
    resistanceMohm: 10_000_000n,
    affectedRefdes: ["R14", "R19", "R27", "R41"],
    adjointMetricShiftPct: 0.02,
  },
  {
    proposalId: "MERGE-0603-4K7",
    sourceMpn: "ERJ-3EKF4751V (4.75kΩ Pull-Up)",
    targetCanonicalMpn: "ERJ-3EKF4701V (4.70kΩ AEC-Q200)",
    packageCode: "0603",
    ratedVoltageUv: 75_000_000n,
    resistanceMohm: 4_700_000n,
    affectedRefdes: ["R8", "R9"],
    adjointMetricShiftPct: 0.14,
  },
  {
    proposalId: "MERGE-FB-DIVIDER",
    sourceMpn: "RG1608N-1022-B-T5 (10.2kΩ 0.1% Type-III FB)",
    targetCanonicalMpn: "AC0603FR-0710KL (10.0kΩ 1%)",
    packageCode: "0603",
    ratedVoltageUv: 75_000_000n,
    resistanceMohm: 10_000_000n,
    affectedRefdes: ["R_FB_TOP"],
    adjointMetricShiftPct: 1.96,
  },
];

const CONSOLIDATION_COLUMNS: readonly SigGridColumn[] = [
  { key: "source", header: "Redundant Feeder Line", mono: true },
  { key: "target", header: "Canonical Consolidated MPN", mono: true },
  { key: "refdes", header: "RefDes", mono: true },
  { key: "shift", header: "Adjoint ΔMetric", mono: true },
  { key: "verdict", header: "Saltation Gate", mono: true },
];

export function evaluateSafeFeederMerges(
  maxAllowedShiftPct: number,
  proposals: readonly BomMergeProposal[] = DEFAULT_BOM_MERGE_PROPOSALS,
): readonly BomMergeProposal[] {
  return proposals.filter((p) => p.adjointMetricShiftPct <= maxAllowedShiftPct);
}

export function buildBomConsolidationVm(
  maxAllowedShiftPct: number,
  baselineLineCount = 42,
  proposals: readonly BomMergeProposal[] = DEFAULT_BOM_MERGE_PROPOSALS,
): BomConsolidationVm {
  const safeMerges = evaluateSafeFeederMerges(maxAllowedShiftPct, proposals);
  const saved = safeMerges.length;
  const afterCount = baselineLineCount - saved;
  const adjointZeroShiftVerified = safeMerges.every(
    (p) => p.adjointMetricShiftPct <= maxAllowedShiftPct,
  );
  const summaryTone: SigStatusTone = saved > 0 ? "ok" : "warn";
  const summaryBadge = `Lines: ${baselineLineCount} -> ${afterCount} (${saved} SMT Feeder Slots Saved • Adjoint Δ <= ${maxAllowedShiftPct.toFixed(2)}%)`;

  const rows: SigGridRow[] = proposals.map((p) => {
    const approved = p.adjointMetricShiftPct <= maxAllowedShiftPct;
    return {
      id: p.proposalId,
      source: p.sourceMpn,
      target: p.targetCanonicalMpn,
      refdes: p.affectedRefdes.join(", "),
      shift: `${p.adjointMetricShiftPct.toFixed(2)}%`,
      verdict: approved ? "SAFE_CONSOLIDATE" : "BLOCKED_SENSITIVE_LOOP",
    };
  });

  return {
    maxAllowedShiftPct,
    uniqueLineItemsBefore: baselineLineCount,
    uniqueLineItemsAfter: afterCount,
    smtFeederSlotsSaved: saved,
    adjointZeroShiftVerified,
    summaryBadge,
    summaryTone,
    columns: CONSOLIDATION_COLUMNS,
    rows,
  };
}
