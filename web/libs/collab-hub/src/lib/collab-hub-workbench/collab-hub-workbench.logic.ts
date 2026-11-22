// Module: web/libs/collab-hub/src/lib/collab-hub-workbench/collab-hub-workbench.logic.ts — Collab Workbench Host Logic
// Invariants: Pure deterministic functions <=250 LOC; formats bigint _nm Jump-to-Canvas HUD coordinates.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import type { CanvasJumpEvent } from "../team-channels-huddle/team-channels-huddle.model";
import type { CollabHubWorkbenchVm } from "./collab-hub-workbench.model";

export const COLLAB_HUB_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "collab-hub",
  title: "5-Team Zero-Leak Collab Space, Status Tower & Auto Reviews",
  subtitle: "Design/Dev/SW/Mfg/Ops DEK channels, @entity threads, SFU huddles & Weekly/Gate Reviews (21-*)",
  rustCrate: "signal-pod",
  subprojectRepo: "aurora-ev-inverter-ops",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Overall RAG: GREEN", tone: "ok" },
    { id: "kpi-2", label: "Stage Gate", value: "DVT (0 Blockers)", tone: "ok" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: [
    "5 Team Channels (Hybrid PQC DEK)",
    "SFU Huddle turns:443 (<=15m TTL)",
    "Weekly + Executive + Gate Auto-Reviews",
  ],
  gridTitle: "Cross-Team Threads & Reviews",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "REV-W47", item: "Automated Weekly Review (DVT Readiness 99.2%)", metric: "ML-DSA-65 Signed" },
    { id: "THR-19", item: "@U1_SIC_HS thermal pad voiding check w/ Mfg", metric: "DESIGN <-> MFG" },
  ],
};

export function buildCollabHubWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  lastJump: CanvasJumpEvent | null,
): CollabHubWorkbenchVm {
  const jumpHudLabel =
    lastJump === null
      ? "Canvas Anchor Focus: Click any authorized @entity chip to jump"
      : `Canvas Anchor Focus: ${lastJump.entityId} (${lastJump.domain} @ ${lastJump.commitSha}) -> (${lastJump.xNm.toString()} nm, ${lastJump.yNm.toString()} nm)`;

  return {
    descriptor,
    cellIsolationBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo} • Zero-Leak DLP`,
    jumpHudLabel,
    hudSignals: [...descriptor.hudSignals, jumpHudLabel],
    lastJump,
  };
}
