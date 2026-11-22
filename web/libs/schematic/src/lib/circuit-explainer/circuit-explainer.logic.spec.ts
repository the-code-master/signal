// Module: web/libs/schematic/src/lib/circuit-explainer/circuit-explainer.logic.spec.ts — FEAT-101 Explainer Tests
// Invariants: Verifies VF2 functional stage flow chain, symbolic H(s), and bigint nm bounding boxes.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildCircuitExplainerVm } from "./circuit-explainer.logic";

describe("CircuitExplainer VF2 stage & symbolic H(s) logic (FEAT-101)", () => {
  it("builds functional stage flow chain and resolves symbolic transfer card", () => {
    const vm = buildCircuitExplainerVm("STG-COMP");
    expect(vm.signalFlowSummary).toBe("STG-EMI -> STG-SIC-HB -> STG-COMP");
    expect(vm.selectedStage.symbolicTransferFn).toContain("G_c(s)");
    expect(vm.bboxBadge).toContain("42000000");
  });
});
