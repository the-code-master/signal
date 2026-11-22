// Module: web/libs/ai-copilot/src/lib/ai-copilot-workbench/ai-copilot-workbench.logic.spec.ts — CU-P3-03 Copilot Tests
// Invariants: Verifies <=20:1 bomb-cap & <=64 depth rejection and F-133 post-rebase proof gate.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md
import { describe, expect, it } from "vitest";
import {
  AI_COPILOT_WORKBENCH_DESCRIPTOR,
  buildAiCopilotWorkbenchVm,
  evaluateProofMergeState,
  validateCleanRoomImport,
} from "./ai-copilot-workbench.logic";

describe("AiCopilotWorkbench ProofBundle & I-10 Clean-Room Import", () => {
  it("enforces <=20:1 archive bomb cap and F-133 merge-time delta proof gate", () => {
    expect(validateCleanRoomImport("ALTIUM_DOC", 1_000n, 15_000n, 16).accepted).toBe(true);
    expect(validateCleanRoomImport("ALTIUM_DOC", 1_000n, 25_000n, 16).accepted).toBe(false);
    expect(validateCleanRoomImport("GDSII_STREAM", 1_000n, 5_000n, 80).accepted).toBe(false);

    expect(evaluateProofMergeState("ADVANCED_CLEAN").deltaReverified).toBe(true);
    expect(evaluateProofMergeState("ADVANCED_CONFLICT").mergeAllowed).toBe(false);

    const vm = buildAiCopilotWorkbenchVm(AI_COPILOT_WORKBENCH_DESCRIPTOR, "KICAD_SEXPR", "UNCHANGED", false);
    expect(vm.proof.mergeAllowed).toBe(true);
    expect(vm.importCheck.accepted).toBe(true);
  });
});
