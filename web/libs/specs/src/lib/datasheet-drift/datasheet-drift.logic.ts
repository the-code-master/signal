// Module: web/libs/specs/src/lib/datasheet-drift/datasheet-drift.logic.ts — FEAT-078 Datasheet Drift Evaluator
// Invariants: Computes pinout & bigint AbsMax (_uv/_mkelvin) regressions between quarantined datasheet revisions.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/18-universal-component-and-datasheet-repository.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { DatasheetDriftVm, DatasheetRevisionNotice } from "./datasheet-drift.model";

export const DEFAULT_DATASHEET_DRIFT_NOTICES: readonly DatasheetRevisionNotice[] = [
  {
    noticeId: "DRIFT-UCC21750-REVD",
    mpn: "UCC21750QDWR",
    oldRevisionLabel: "Rev C (2025-08)",
    newRevisionLabel: "Rev D (2026-10)",
    oldDocBlake3: "b3:8a14f902c7e1",
    newDocBlake3: "b3:f491d03be812",
    changedPinCount: 1,
    tightenedAbsMaxCount: 2,
    oldAbsMaxUv: 33_000_000n,
    newAbsMaxUv: 30_000_000n,
    oldTjMaxMkelvin: 423_150n,
    newTjMaxMkelvin: 413_150n,
    parameterDeltaSummaries: [
      "Pin 14 renamed NC -> CLAMP_FLT_N (ERC re-check required)",
      "VDD-VEE AbsMax tightened 33V -> 30V (-3_000_000 uV)",
      "Max Tj derated 150 °C -> 140 °C (-10_000 mK)",
    ],
  },
  {
    noticeId: "DRIFT-C3M0016120K-REV5",
    mpn: "C3M0016120K",
    oldRevisionLabel: "Rev 4",
    newRevisionLabel: "Rev 5",
    oldDocBlake3: "b3:21c07aa49910",
    newDocBlake3: "b3:99e3b7104a8c",
    changedPinCount: 0,
    tightenedAbsMaxCount: 1,
    oldAbsMaxUv: 1_200_000_000n,
    newAbsMaxUv: 1_200_000_000n,
    oldTjMaxMkelvin: 448_150n,
    newTjMaxMkelvin: 443_150n,
    parameterDeltaSummaries: [
      "Continuous pulsed SOA Tj cap tightened 175 °C -> 170 °C (-5_000 mK)",
      "C_oss(800V) curve updated +2.4 pF in ConditionCurveSet",
    ],
  },
];

const DRIFT_COLUMNS: readonly SigGridColumn[] = [
  { key: "mpn", header: "MPN", mono: true },
  { key: "rev", header: "Revision Shift", mono: true },
  { key: "deltas", header: "Quarantined AST Pin / AbsMax Delta" },
  { key: "status", header: "Lockfile Gate", mono: true },
];

export function computeAbsMaxRegressionUv(notice: DatasheetRevisionNotice): bigint {
  return notice.oldAbsMaxUv - notice.newAbsMaxUv;
}

export function buildDatasheetDriftVm(
  selectedNoticeId: string,
  acknowledgedIds: ReadonlySet<string>,
  notices: readonly DatasheetRevisionNotice[] = DEFAULT_DATASHEET_DRIFT_NOTICES,
): DatasheetDriftVm {
  const selectedNotice =
    notices.find((n) => n.noticeId === selectedNoticeId) ?? notices[0];
  const unack = notices.filter((n) => !acknowledgedIds.has(n.noticeId));
  const hasPinDrift = unack.some((n) => n.changedPinCount > 0);
  const summaryTone: SigStatusTone =
    unack.length === 0 ? "ok" : hasPinDrift ? "danger" : "warn";

  const summaryBadge =
    unack.length === 0
      ? "All Datasheet Revisions Re-Verified & Pinned in parts.lock.json"
      : `${unack.length} Unresolved Datasheet Drift Alert(s) • ERC/SOA Re-Check Required`;

  const vDropUv = computeAbsMaxRegressionUv(selectedNotice);
  const tjDropMk = selectedNotice.oldTjMaxMkelvin - selectedNotice.newTjMaxMkelvin;
  const hashCompareBadge = `${selectedNotice.oldDocBlake3} -> ${selectedNotice.newDocBlake3} • ΔVmax=-${vDropUv.toString(10)} uV • ΔTj=-${tjDropMk.toString(10)} mK`;

  const rows: SigGridRow[] = selectedNotice.parameterDeltaSummaries.map((delta, idx) => ({
    id: `${selectedNotice.noticeId}-${idx}`,
    mpn: selectedNotice.mpn,
    rev: `${selectedNotice.oldRevisionLabel} -> ${selectedNotice.newRevisionLabel}`,
    deltas: delta,
    status: acknowledgedIds.has(selectedNotice.noticeId) ? "LOCKED_REV_NEW" : "NEEDS_REVIEW",
  }));

  return {
    notices,
    selectedNotice,
    acknowledgedIds,
    unacknowledgedCount: unack.length,
    summaryBadge,
    summaryTone,
    hashCompareBadge,
    columns: DRIFT_COLUMNS,
    rows,
  };
}
