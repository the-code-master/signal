// Module: web/libs/collab-hub/src/lib/team-channels-huddle/team-channels-huddle.model.ts — 5-Team Channels & Huddle Models
// Invariants: Pure readonly TypeScript types; 64-bit _nm coordinates and _utc_ms timestamps use bigint ONLY.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type CollabTeamKind =
  | "DESIGN"
  | "DEVELOPMENT_BRINGUP"
  | "SOFTWARE_FIRMWARE"
  | "MANUFACTURING"
  | "OPERATIONS_SUPPLY_CHAIN"
  | "CROSS_TEAM_PROGRAM";

export type CollabViewerRole = "ORG_ADMIN" | "PCB_CONTRACTOR";

export interface CollabEntityAnchor {
  readonly anchorId: string;
  readonly domain: "SCHEMATIC" | "PCB" | "CAD_MCAD" | "PLM_OPS";
  readonly entityId: string;
  readonly commitSha: string;
  readonly xNm: bigint;
  readonly yNm: bigint;
  readonly zNm: bigint;
  readonly requiredProviderRepoId: string;
  readonly redacted: boolean;
}

export interface CollabChannelSpec {
  readonly channelId: string;
  readonly slug: string;
  readonly title: string;
  readonly team: CollabTeamKind;
  readonly requiredRepoScopes: readonly string[];
  readonly dekKeyId: string;
}

export interface CollabThreadMessage {
  readonly messageId: string;
  readonly channelId: string;
  readonly authorHmac: string;
  readonly body: string;
  readonly dekEnvelopePrefix: string;
  readonly anchors: readonly CollabEntityAnchor[];
  readonly redactedAnchorCount: number;
  readonly createdAtUtcMs: bigint;
}

export interface HuddleSessionState {
  readonly active: boolean;
  readonly huddleRoomId: string;
  readonly sfuTurnsEndpoint: string;
  readonly ticketPreview: string;
  readonly expiresAtUtcMs: bigint;
  readonly followPresenterViewport: boolean;
  readonly geometryRecordingDisabled: true;
}

export interface CanvasJumpEvent {
  readonly anchorId: string;
  readonly domain: string;
  readonly entityId: string;
  readonly commitSha: string;
  readonly xNm: bigint;
  readonly yNm: bigint;
  readonly zNm: bigint;
}

export interface TeamChannelsHuddleVm {
  readonly viewerRole: CollabViewerRole;
  readonly viewerRoles: readonly CollabViewerRole[];
  readonly visibleChannels: readonly CollabChannelSpec[];
  readonly activeChannel: CollabChannelSpec;
  readonly visibleMessages: readonly CollabThreadMessage[];
  readonly visibleClickableAnchors: readonly CollabEntityAnchor[];
  readonly messageColumns: readonly SigGridColumn[];
  readonly messageRows: readonly SigGridRow[];
  readonly huddle: HuddleSessionState;
  readonly huddleTone: SigStatusTone;
  readonly huddleBadge: string;
  readonly scopeSummaryBadge: string;
}
