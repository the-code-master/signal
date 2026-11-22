// Module: web/libs/review/src/lib/live-review-deck/live-review-deck.model.ts — FEAT-109 Zero-Export Review Deck Models
// Invariants: Models live interactive PDR/CDR/FDR deck slides with bigint nm viewport anchors and zero file export.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type ReviewDeckMilestone = "PDR" | "CDR" | "FDR";

export interface LiveReviewSlide {
  readonly slideId: string;
  readonly milestone: ReviewDeckMilestone;
  readonly title: string;
  readonly workbenchSlug: string;
  readonly anchorXNm: bigint;
  readonly anchorYNm: bigint;
  readonly liveAssertionSummary: string;
  readonly gatePassing: boolean;
}

export interface LiveReviewDeckVm {
  readonly milestone: ReviewDeckMilestone;
  readonly milestones: readonly ReviewDeckMilestone[];
  readonly slides: readonly LiveReviewSlide[];
  readonly activeSlide: LiveReviewSlide;
  readonly slideIndex: number;
  readonly positionBadge: string;
  readonly dlpBadge: string;
  readonly gateTone: SigStatusTone;
  readonly anchorBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
