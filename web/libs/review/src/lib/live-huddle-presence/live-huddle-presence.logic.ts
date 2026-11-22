// Module: web/libs/review/src/lib/live-huddle-presence/live-huddle-presence.logic.ts — FEAT-094 Follow-Me & Laser Logic
// Invariants: Resolves leader viewport lock in bigint nanometers; formats ephemeral 60Hz laser reticle state.
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { CollaboratorPresencePeer, LiveHuddlePresenceVm } from "./live-huddle-presence.model";

export const DEFAULT_HUDDLE_PEERS: readonly CollaboratorPresencePeer[] = [
  { sessionId: "peer-lead-ee", displayHandle: "Lead Power EE", cameraCenterXNm: 14_200_000n, cameraCenterYNm: 8_500_000n, activeDomain: "SCHEMATIC" },
  { sessionId: "peer-si-layout", displayHandle: "224G SI Layout", cameraCenterXNm: 42_800_000n, cameraCenterYNm: 19_100_000n, activeDomain: "PCB" },
  { sessionId: "peer-mech-lead", displayHandle: "Thermal MCAD", cameraCenterXNm: 88_000_000n, cameraCenterYNm: 31_000_000n, activeDomain: "CAD_MCAD" },
];

export function buildLiveHuddlePresenceVm(
  followedSessionId: string | null,
  laserPointerActive: boolean,
  peers: readonly CollaboratorPresencePeer[] = DEFAULT_HUDDLE_PEERS,
): LiveHuddlePresenceVm {
  const followedLeader =
    followedSessionId !== null ? (peers.find((p) => p.sessionId === followedSessionId) ?? null) : null;
  const followBadge = followedLeader
    ? `Following ${followedLeader.displayHandle} @ (${followedLeader.cameraCenterXNm.toString(10)}, ${followedLeader.cameraCenterYNm.toString(10)}) nm`
    : "Camera: Independent Free-Pan";
  return {
    peers,
    followedLeader,
    laserPointerActive,
    followBadge,
    laserBadge: laserPointerActive ? "60 Hz Ephemeral Laser: ACTIVE" : "Laser Pointer: OFF",
  };
}
