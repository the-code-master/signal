// Module: web/libs/visual-diff/src/lib/visual-diff-workbench/visual-diff-workbench.logic.spec.ts — Visual Diff Tests
// Invariants: Verifies domain filtering across SCHEMATIC, PCB, and CAD_MCAD semantic diffs.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import {
  buildVisualDiffWorkbenchVm,
  VISUAL_DIFF_WORKBENCH_DESCRIPTOR,
} from "./visual-diff-workbench.logic";

describe("buildVisualDiffWorkbenchVm", () => {
  it("filters Onion-Skin semantic diffs across ALL, PCB, and CAD_MCAD", () => {
    expect(buildVisualDiffWorkbenchVm(VISUAL_DIFF_WORKBENCH_DESCRIPTOR, "ALL", 101n).diffRows.length).toBe(4);
    expect(buildVisualDiffWorkbenchVm(VISUAL_DIFF_WORKBENCH_DESCRIPTOR, "PCB", 101n).diffRows.length).toBe(2);
    expect(buildVisualDiffWorkbenchVm(VISUAL_DIFF_WORKBENCH_DESCRIPTOR, "CAD_MCAD", 101n).diffRows.length).toBe(1);
  });
});
