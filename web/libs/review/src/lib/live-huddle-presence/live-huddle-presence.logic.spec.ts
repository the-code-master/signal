// Module: web/libs/review/src/lib/live-huddle-presence/live-huddle-presence.logic.spec.ts — FEAT-094 Presence Tests
// Invariants: Verifies Follow-Me bigint camera lock and laser reticle toggle.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildLiveHuddlePresenceVm } from "./live-huddle-presence.logic";

describe("LiveHuddlePresence Follow-Me & Laser (FEAT-094)", () => {
  it("locks camera to selected collaborator leader and tracks laser state", () => {
    const free = buildLiveHuddlePresenceVm(null, false);
    expect(free.followedLeader).toBeNull();

    const locked = buildLiveHuddlePresenceVm("peer-si-layout", true);
    expect(locked.followedLeader?.activeDomain).toBe("PCB");
    expect(locked.followBadge).toContain("42800000");
    expect(locked.laserPointerActive).toBe(true);
  });
});
