// Module: web/libs/review/src/lib/review-workbench/review-workbench.logic.ts — Spatial Review Threads & Gate Console Logic
// Invariants: Zero Angular decorators; formats spatial bigint nm review anchors (F-074).
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { ReviewWorkbenchVm, SpatialReviewThread } from "./review-workbench.model";

export const REVIEW_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "review",
  title: "Design Gate & Spatial Review Console",
  subtitle: "Spatial canvas threads (F-074), Follow-Me (FEAT-094), Stage-Gate (FEAT-098) & Partner Portal (FEAT-119)",
  rustCrate: "signal-git-sync",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Posture Score: 98/100", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Stale Grants: 0", tone: "ok" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Spatial Pin: @Q1_SIC", "Follow-Me: Ready"],
  gridTitle: "Spatial Review Threads",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "U1", item: "local-guest-0001 (Org Admin)", metric: "All 5 Repos" },
    { id: "U2", item: "local-pcb-contractor-0002", metric: "PCB Repo ONLY" },
  ],
};

export const DEFAULT_SPATIAL_THREADS: readonly SpatialReviewThread[] = [
  { threadId: "THR-01", entityAnchor: "@Q1_SIC_HS", anchorXNm: 14_200_000n, anchorYNm: 8_500_000n, comment: "Verify creepage slot >= 8.0 mm under drain tab", resolved: false },
  { threadId: "THR-02", entityAnchor: "@U2_GATE_DRV", anchorXNm: 18_900_000n, anchorYNm: 9_100_000n, comment: "Kelvin source return verified < 1.2 nH", resolved: true },
];

const THREAD_COLUMNS: readonly SigGridColumn[] = [
  { key: "threadId", header: "Thread", mono: true },
  { key: "anchor", header: "Spatial Entity & Anchor (nm)", mono: true },
  { key: "comment", header: "Engineering Review Comment" },
  { key: "state", header: "Status", mono: true },
];

export function buildReviewWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  resolveAll: boolean,
): ReviewWorkbenchVm {
  const threads = DEFAULT_SPATIAL_THREADS.map((t) =>
    resolveAll ? { ...t, resolved: true } : t,
  );
  const openCount = threads.filter((t) => !t.resolved).length;
  const threadRows: SigGridRow[] = threads.map((t) => ({
    id: t.threadId,
    threadId: t.threadId,
    anchor: `${t.entityAnchor} (${t.anchorXNm.toString(10)}, ${t.anchorYNm.toString(10)})`,
    comment: t.comment,
    state: t.resolved ? "RESOLVED" : "OPEN_BLOCKER",
  }));

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    openThreadBadge: `Open Spatial Blockers: ${openCount}`,
    threadColumns: THREAD_COLUMNS,
    threadRows,
  };
}
