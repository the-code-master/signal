// Module: web/libs/review/src/lib/stage-gate-checklist/stage-gate-checklist.logic.spec.ts — FEAT-098 Stage-Gate Tests
// Invariants: Verifies EVT/DVT/TAPEOUT ready state vs PVT blocked state until all criteria pass.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildStageGateChecklistVm } from "./stage-gate-checklist.logic";

describe("StageGateChecklist logic (FEAT-098)", () => {
  it("blocks PVT sign-off when criteria are pending and enables DVT/TAPEOUT sign-off", () => {
    const pvt = buildStageGateChecklistVm("PVT", new Set());
    expect(pvt.readyToSignOff).toBe(false);

    const dvtSigned = buildStageGateChecklistVm("DVT", new Set(["DVT"]));
    expect(dvtSigned.readyToSignOff).toBe(true);
    expect(dvtSigned.signedOff).toBe(true);
  });
});
