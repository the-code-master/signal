// Module: web/libs/review/src/lib/stage-gate-checklist/stage-gate-checklist.model.ts — FEAT-098 Stage-Gate Models
// Invariants: Evaluates EVT, DVT, PVT & TAPEOUT hardware readiness gates cleanly.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, architecture/17-*.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type HardwareStageGate = "EVT" | "DVT" | "PVT" | "TAPEOUT";

export interface StageGateCriterion {
  readonly id: string;
  readonly gate: HardwareStageGate;
  readonly criterion: string;
  readonly passed: boolean;
}

export interface StageGateChecklistVm {
  readonly activeGate: HardwareStageGate;
  readonly gates: readonly HardwareStageGate[];
  readonly readyToSignOff: boolean;
  readonly signedOff: boolean;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
