// Module: web/libs/review/src/lib/live-review-deck/live-review-deck.logic.spec.ts — FEAT-109 Unit Tests
// Invariants: Verifies milestone filtering, wrap-around slide stepping, and bigint nm viewport anchors.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import { describe, expect, it } from "vitest";
import { buildLiveReviewDeckVm, stepSlideIndex } from "./live-review-deck.logic";

describe("live-review-deck.logic (FEAT-109)", () => {
  it("filters slides by CDR milestone and formats bigint nm viewport anchors", () => {
    const vm = buildLiveReviewDeckVm("CDR", 1);
    expect(vm.slides).toHaveLength(2);
    expect(vm.activeSlide.slideId).toBe("CDR-02");
    expect(vm.anchorBadge).toContain("9100000, 4200000");
    expect(vm.gateTone).toBe("ok");
  });

  it("wraps slide index deterministically across forward and backward steps", () => {
    expect(stepSlideIndex(0, -1, 2)).toBe(1);
    expect(stepSlideIndex(1, 1, 2)).toBe(0);
  });
});
