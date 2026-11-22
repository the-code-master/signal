// Module: web/libs/collab-hub/src/lib/team-channels-huddle/team-channels-huddle.logic.spec.ts — Unit Tests
// Invariants: Verifies contractor channel hiding, [Restricted Entity] masking, bigint Jump-to-Canvas & <=15m SFU TTL.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import { describe, expect, it } from "vitest";
import {
  MAX_HUDDLE_TTL_MS,
  SEEDED_RAW_MESSAGES,
  buildTeamChannelsHuddleVm,
  createHuddleTicketState,
  toCanvasJumpEvent,
} from "./team-channels-huddle.logic";

describe("team-channels-huddle.logic", () => {
  it("exposes all 6 team/program channels to ORG_ADMIN with unmasked @entity anchors", () => {
    const huddle = createHuddleTicketState("ch-all-hands", false, true, 1_795_324_800_000n);
    const vm = buildTeamChannelsHuddleVm("ORG_ADMIN", "ch-all-hands", SEEDED_RAW_MESSAGES, huddle);
    expect(vm.visibleChannels.length).toBe(6);
    expect(vm.visibleMessages[0]?.redactedAnchorCount).toBe(0);
    expect(vm.visibleClickableAnchors.length).toBe(2);
    expect(vm.messageRows[0]?.["anchors"]).toContain("@ECO-2026-104@7f22d19");
  });

  it("hides unauthorized channels from PCB_CONTRACTOR and masks @ECO-2026-104 in #all-hands-status", () => {
    const huddle = createHuddleTicketState("ch-all-hands", true, true, 1_795_324_800_000n, 1_900_000n);
    const vm = buildTeamChannelsHuddleVm("PCB_CONTRACTOR", "ch-ops-plm", SEEDED_RAW_MESSAGES, huddle);
    expect(vm.visibleChannels.map((c) => c.channelId)).toEqual(["ch-all-hands", "ch-dev-hil"]);
    expect(vm.activeChannel.channelId).toBe("ch-all-hands");

    const msg = vm.visibleMessages[0]!;
    expect(msg.redactedAnchorCount).toBe(1);
    expect(msg.anchors[0]?.entityId).toBe("@U1_SIC_HS");
    expect(msg.anchors[1]?.entityId).toBe("[Restricted Entity — Access Required]");
    expect(toCanvasJumpEvent(msg.anchors[1]!)).toBeNull();

    const jump = toCanvasJumpEvent(msg.anchors[0]!);
    expect(jump?.xNm).toBe(42_500_000n);
    expect(huddle.expiresAtUtcMs - 1_795_324_800_000n).toBe(MAX_HUDDLE_TTL_MS);
    expect(huddle.geometryRecordingDisabled).toBe(true);
  });
});
