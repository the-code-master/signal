// Module: web/libs/review/src/lib/live-review-deck/live-review-deck.logic.ts — FEAT-109 Live PDR/CDR/FDR Deck Logic
// Invariants: Clamps slide navigation per milestone and formats bigint nm viewport anchors with Tardos DLP badge.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  LiveReviewDeckVm,
  LiveReviewSlide,
  ReviewDeckMilestone,
} from "./live-review-deck.model";

export const DEFAULT_REVIEW_SLIDES: readonly LiveReviewSlide[] = [
  {
    slideId: "PDR-01",
    milestone: "PDR",
    title: "System Architecture & 800V SiC Power Stage Budget",
    workbenchSlug: "schematic",
    anchorXNm: 14_200_000n,
    anchorYNm: 8_500_000n,
    liveAssertionSummary: "PRD-001..008 Bound • Peak Eff 98.9% (Target >= 98.5%)",
    gatePassing: true,
  },
  {
    slideId: "PDR-02",
    milestone: "PDR",
    title: "Preliminary Thermal & Creepage Envelope",
    workbenchSlug: "sim-viewer",
    anchorXNm: 22_000_000n,
    anchorYNm: 11_000_000n,
    liveAssertionSummary: "IEC 60664-1 Pollution Deg 2 >= 8_000_000 nm Verified",
    gatePassing: true,
  },
  {
    slideId: "CDR-01",
    milestone: "CDR",
    title: "Saltation-Adjoint Worst-Case SOA & Monte Carlo Yield",
    workbenchSlug: "sim-viewer",
    anchorXNm: 18_750_000n,
    anchorYNm: 6_400_000n,
    liveAssertionSummary: "Cpk = 1.68 across 1,000 Latin-Hypercube Corners",
    gatePassing: true,
  },
  {
    slideId: "CDR-02",
    milestone: "CDR",
    title: "200-Point Red-Team & ECIA Franchise BOM Lock",
    workbenchSlug: "specs",
    anchorXNm: 9_100_000n,
    anchorYNm: 4_200_000n,
    liveAssertionSummary: "200/200 Red-Team Passing • 0 Unverified Spot-Brokers",
    gatePassing: true,
  },
  {
    slideId: "FDR-01",
    milestone: "FDR",
    title: "Final Tapeout / Fab Sign-Off & ML-DSA-65 Attestation",
    workbenchSlug: "review",
    anchorXNm: 31_500_000n,
    anchorYNm: 19_800_000n,
    liveAssertionSummary: "0 DRC / 0 LVS / Formal Equivalence UNSAT • Ready for SFTP Egress",
    gatePassing: true,
  },
];

const DECK_COLUMNS: readonly SigGridColumn[] = [
  { key: "slideId", header: "Slide", mono: true },
  { key: "title", header: "Live Interactive Canvas Briefing" },
  { key: "workbench", header: "Target Stage", mono: true },
  { key: "assertion", header: "Live Rust Telemetry", mono: true },
];

export function stepSlideIndex(current: number, delta: number, count: number): number {
  if (count <= 0) return 0;
  return (((current + delta) % count) + count) % count;
}

export function buildLiveReviewDeckVm(
  milestone: ReviewDeckMilestone,
  rawSlideIndex: number,
  allSlides: readonly LiveReviewSlide[] = DEFAULT_REVIEW_SLIDES,
): LiveReviewDeckVm {
  const slides = allSlides.filter((s) => s.milestone === milestone);
  const clampedIdx = slides.length > 0 ? Math.max(0, Math.min(rawSlideIndex, slides.length - 1)) : 0;
  const activeSlide = slides[clampedIdx] ?? allSlides[0];
  const gateTone: SigStatusTone = slides.every((s) => s.gatePassing) ? "ok" : "warn";

  const rows: SigGridRow[] = slides.map((s) => ({
    id: s.slideId,
    slideId: s.slideId,
    title: s.title,
    workbench: s.workbenchSlug,
    assertion: s.liveAssertionSummary,
  }));

  return {
    milestone,
    milestones: ["PDR", "CDR", "FDR"],
    slides,
    activeSlide,
    slideIndex: clampedIdx,
    positionBadge: `${milestone} Slide ${clampedIdx + 1} / ${slides.length}: ${activeSlide.title}`,
    dlpBadge: "Zero-Export Presentation Mode • Tardos Watermark Active",
    gateTone,
    anchorBadge: `Live Viewport Jump -> ${activeSlide.workbenchSlug} @ (${activeSlide.anchorXNm.toString(10)}, ${activeSlide.anchorYNm.toString(10)}) nm`,
    columns: DECK_COLUMNS,
    rows,
  };
}
