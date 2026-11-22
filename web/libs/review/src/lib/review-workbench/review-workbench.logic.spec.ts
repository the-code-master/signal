// Module: web/libs/review/src/lib/review-workbench/review-workbench.logic.spec.ts — Spatial Review Workbench Tests
// Invariants: Verifies spatial thread blocker counts and resolution toggle.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import { buildReviewWorkbenchVm, REVIEW_WORKBENCH_DESCRIPTOR } from "./review-workbench.logic";

describe("buildReviewWorkbenchVm", () => {
  it("tracks open spatial review threads and resolves blockers cleanly", () => {
    expect(buildReviewWorkbenchVm(REVIEW_WORKBENCH_DESCRIPTOR, false).openThreadBadge).toContain("1");
    expect(buildReviewWorkbenchVm(REVIEW_WORKBENCH_DESCRIPTOR, true).openThreadBadge).toContain("0");
  });
});
