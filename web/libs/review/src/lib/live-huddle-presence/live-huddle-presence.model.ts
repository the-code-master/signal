// Module: web/libs/review/src/lib/live-huddle-presence/live-huddle-presence.model.ts — FEAT-094 Follow-Me & Laser Pointer Models
// Invariants: Camera/laser world coordinates stay in ES2020 bigint nanometers (_nm).
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/17-extended-feature-specifications-ui-and-rust.md
export interface CollaboratorPresencePeer {
  readonly sessionId: string;
  readonly displayHandle: string;
  readonly cameraCenterXNm: bigint;
  readonly cameraCenterYNm: bigint;
  readonly activeDomain: string;
}

export interface LiveHuddlePresenceVm {
  readonly peers: readonly CollaboratorPresencePeer[];
  readonly followedLeader: CollaboratorPresencePeer | null;
  readonly laserPointerActive: boolean;
  readonly followBadge: string;
  readonly laserBadge: string;
}
