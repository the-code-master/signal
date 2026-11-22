// Module: web/libs/specs/src/lib/bom-consolidation/bom-consolidation.logic.spec.ts — FEAT-108 Unit Tests
// Invariants: Ensures precision feedback loop resistors exceeding Saltation-Adjoint shift tolerance are never merged.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md
import { describe, expect, it } from "vitest";
import { buildBomConsolidationVm, evaluateSafeFeederMerges } from "./bom-consolidation.logic";

describe("bom-consolidation.logic (FEAT-108)", () => {
  it("saves 2 SMT feeder slots at 0.25% max shift while blocking precision feedback divider merges", () => {
    const safe = evaluateSafeFeederMerges(0.25);
    expect(safe.map((p) => p.proposalId)).toEqual(["MERGE-0402-10K", "MERGE-0603-4K7"]);

    const vm = buildBomConsolidationVm(0.25, 42);
    expect(vm.smtFeederSlotsSaved).toBe(2);
    expect(vm.uniqueLineItemsAfter).toBe(40);
    expect(vm.adjointZeroShiftVerified).toBe(true);
    expect(vm.rows[2]?.["verdict"]).toBe("BLOCKED_SENSITIVE_LOOP");
  });

  it("restricts merges to strict zero-shift candidates under ultra-tight 0.05% cap", () => {
    const vm = buildBomConsolidationVm(0.05, 42);
    expect(vm.smtFeederSlotsSaved).toBe(1);
    expect(vm.uniqueLineItemsAfter).toBe(41);
  });
});
