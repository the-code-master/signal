// Module: web/libs/collab-hub/src/lib/collab-hub-workbench/collab-hub-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies bigint _nm Jump-to-Canvas HUD telemetry propagation into DualPlaneViewport signals.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import { describe, expect, it } from "vitest";
import {
  COLLAB_HUB_WORKBENCH_DESCRIPTOR,
  buildCollabHubWorkbenchVm,
} from "./collab-hub-workbench.logic";

describe("collab-hub-workbench.logic", () => {
  it("propagates authorized @entity bigint _nm Jump-to-Canvas focus into viewport HUD signals", () => {
    const idleVm = buildCollabHubWorkbenchVm(COLLAB_HUB_WORKBENCH_DESCRIPTOR, null);
    expect(idleVm.lastJump).toBeNull();
    expect(idleVm.hudSignals.length).toBe(4);

    const jumpedVm = buildCollabHubWorkbenchVm(COLLAB_HUB_WORKBENCH_DESCRIPTOR, {
      anchorId: "anc-pcb-u1-sic",
      domain: "PCB",
      entityId: "@U1_SIC_HS",
      commitSha: "a81f90c",
      xNm: 42_500_000n,
      yNm: 18_250_000n,
      zNm: 0n,
    });
    expect(jumpedVm.jumpHudLabel).toContain("(42500000 nm, 18250000 nm)");
    expect(jumpedVm.hudSignals[3]).toContain("@U1_SIC_HS");
  });
});
