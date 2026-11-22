// Module: web/libs/collab-hub/src/lib/automated-review-studio/automated-review-studio.model.ts — Auto-Review Models
// Invariants: Pure readonly TypeScript types; 64-bit _fs STA slack and _utc_ms timestamps use bigint ONLY.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { RagHealthKind, StageGateCode } from "../project-status-tower/project-status-tower.model";

export type ReviewCadenceKind = "WEEKLY_ENGINEERING" | "MONTHLY_EXECUTIVE" | "STAGE_GATE_STATUS";

export interface ReviewDeckSlide {
  readonly slideIndex: number;
  readonly heading: string;
  readonly deterministicCitation: string;
  readonly narrativeLine: string;
}

export interface CompiledReviewPacket {
  readonly reviewId: string;
  readonly cadence: ReviewCadenceKind;
  readonly stageGate: StageGateCode;
  readonly overallHealth: RagHealthKind;
  readonly openDrcErcViolations: number;
  readonly worstSpiceSoaMarginPct: number;
  readonly worstStaSlackFs: bigint;
  readonly clearToBuildUnits: number;
  readonly securityPostureScore: number;
  readonly archivedOpsRepoPath: string;
  readonly mlDsa65Signature: string;
  readonly generatedAtUtcMs: bigint;
  readonly slides: readonly ReviewDeckSlide[];
  readonly gateApproved: boolean;
  readonly quorumAuditSeq: bigint | null;
  readonly opsRepoCommitSha: string | null;
}

export interface SignOffAttemptOutcome {
  readonly packet: CompiledReviewPacket;
  readonly burnedNonces: readonly string[];
  readonly statusMessage: string;
  readonly rejected: boolean;
}

export interface AutomatedReviewStudioVm {
  readonly cadences: readonly ReviewCadenceKind[];
  readonly activePacket: CompiledReviewPacket;
  readonly presentationMode: boolean;
  readonly activeSlide: ReviewDeckSlide;
  readonly slideCounterLabel: string;
  readonly kpiColumns: readonly SigGridColumn[];
  readonly kpiRows: readonly SigGridRow[];
  readonly signOffTone: SigStatusTone;
  readonly signOffBadge: string;
  readonly lastSignOffStatus: string;
}
