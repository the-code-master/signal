// Module: web/libs/collab-hub/src/lib/project-status-tower/project-status-tower.model.ts — Status Control Tower Models
// Invariants: Pure readonly TypeScript types; 64-bit _utc_ms timestamps use bigint ONLY.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { CollabTeamKind } from "../team-channels-huddle/team-channels-huddle.model";

export type RagHealthKind = "GREEN" | "AMBER" | "RED";
export type StageGateCode = "CONCEPT" | "PRD_FREEZE" | "EVT" | "DVT" | "PVT" | "TAPEOUT" | "MP";
export type DependencyHandshakeKind = "PROPOSED" | "ACCEPTED" | "AT_RISK" | "BLOCKED";

export interface TeamRagHealthCard {
  readonly team: CollabTeamKind;
  readonly autoHealth: RagHealthKind;
  readonly effectiveHealth: RagHealthKind;
  readonly tone: SigStatusTone;
  readonly openBlockers: number;
  readonly openWarnings: number;
  readonly overrideRationale: string | null;
}

export interface StageGateMilestoneItem {
  readonly gateId: string;
  readonly gateName: StageGateCode;
  readonly targetDateUtcMs: bigint;
  readonly health: RagHealthKind;
  readonly openBlockerCount: number;
}

export interface CrossTeamDependencyItem {
  readonly dependencyId: string;
  readonly producerTeam: CollabTeamKind;
  readonly consumerTeam: CollabTeamKind;
  readonly deliverableTitle: string;
  readonly dueAtUtcMs: bigint;
  readonly handshakeStatus: DependencyHandshakeKind;
}

export interface SignedDecisionEntry {
  readonly decisionId: string;
  readonly recordKind: "ADR" | "EDR";
  readonly title: string;
  readonly rationale: string;
  readonly decidedAtUtcMs: bigint;
  readonly mlDsa65Signature: string;
}

export interface ProjectStatusTowerVm {
  readonly overallHealth: RagHealthKind;
  readonly overallTone: SigStatusTone;
  readonly overallBadge: string;
  readonly teamCards: readonly TeamRagHealthCard[];
  readonly milestoneColumns: readonly SigGridColumn[];
  readonly milestoneRows: readonly SigGridRow[];
  readonly dependencyColumns: readonly SigGridColumn[];
  readonly dependencyRows: readonly SigGridRow[];
  readonly decisionColumns: readonly SigGridColumn[];
  readonly decisionRows: readonly SigGridRow[];
}
