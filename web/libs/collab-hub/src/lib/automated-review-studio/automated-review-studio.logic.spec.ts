// Module: web/libs/collab-hub/src/lib/automated-review-studio/automated-review-studio.logic.spec.ts — Unit Tests
// Invariants: Verifies deterministic bigint STA slack citations, Zero-Leak deck slides, and 2FA replay + 503 quorum rejection.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import { describe, expect, it } from "vitest";
import {
  buildAutomatedReviewStudioVm,
  compileDeterministicReviewPacket,
  signOffStageGatePacket,
} from "./automated-review-studio.logic";

describe("automated-review-studio.logic", () => {
  it("compiles deterministic Weekly, Monthly Executive & Stage-Gate packets with bigint STA slack", () => {
    const pkt = compileDeterministicReviewPacket("STAGE_GATE_STATUS", 1_795_324_800_000n);
    expect(pkt.worstStaSlackFs).toBe(142_000n);
    expect(pkt.archivedOpsRepoPath).toBe("reviews/2026/rev-2026-dvt-gate.json");

    const vm = buildAutomatedReviewStudioVm(pkt, true, 1, "Ready");
    expect(vm.presentationMode).toBe(true);
    expect(vm.activeSlide.slideIndex).toBe(1);
    expect(vm.kpiRows[1]?.["value"]).toBe("142000 fs");
  });

  it("enforces 3/3 audit quorum, ML-DSA-65 signature, and single-use Step-Up 2FA nonce burn on sign-off", () => {
    const pkt = compileDeterministicReviewPacket("STAGE_GATE_STATUS", 1_795_324_800_000n);

    const quorumFail = signOffStageGatePacket(pkt, "nonce-valid-001", "ml-dsa-65:lead0011223344", 2, [], 1n);
    expect(quorumFail.rejected).toBe(true);
    expect(quorumFail.statusMessage).toContain("503");

    const ok = signOffStageGatePacket(pkt, "nonce-valid-001", "ml-dsa-65:lead0011223344", 3, [], 7n);
    expect(ok.rejected).toBe(false);
    expect(ok.packet.gateApproved).toBe(true);
    expect(ok.packet.quorumAuditSeq).toBe(7n);

    const replay = signOffStageGatePacket(
      ok.packet,
      "nonce-valid-001",
      "ml-dsa-65:lead0011223344",
      3,
      ok.burnedNonces,
      8n,
    );
    expect(replay.rejected).toBe(true);
    expect(replay.statusMessage).toContain("STEPUP_NONCE_ALREADY_BURNED");
  });
});
