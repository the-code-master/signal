// Module: web/libs/visual-diff/src/lib/history-time-scrubber/history-time-scrubber.logic.spec.ts — FEAT-097 Tests
// Invariants: Verifies Super-Commit checkpoint clamping and onion-skin blend formatting.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildHistoryTimeScrubberVm } from "./history-time-scrubber.logic";

describe("HistoryTimeScrubber logic (FEAT-097)", () => {
  it("resolves Super-Commit revision and clamps onion-skin opacity", () => {
    const vm = buildHistoryTimeScrubberVm(2, 65);
    expect(vm.selectedCheckpoint.revision).toBe(101n);
    expect(vm.onionSkinOpacityPct).toBe(65);
    expect(vm.scrubberBadge).toContain("@101");
  });
});
