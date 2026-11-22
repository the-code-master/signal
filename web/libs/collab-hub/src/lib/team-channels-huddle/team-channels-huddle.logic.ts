// Module: web/libs/collab-hub/src/lib/team-channels-huddle/team-channels-huddle.logic.ts — Scope Masking & Huddle Logic
// Invariants: Pure deterministic functions <=250 LOC; masks out-of-scope @entity anchors & enforces <=15m SFU TTL.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type {
  CanvasJumpEvent,
  CollabChannelSpec,
  CollabEntityAnchor,
  CollabThreadMessage,
  CollabViewerRole,
  HuddleSessionState,
  TeamChannelsHuddleVm,
} from "./team-channels-huddle.model";

export const MAX_HUDDLE_TTL_MS = 900_000n;

export const SEEDED_COLLAB_CHANNELS: readonly CollabChannelSpec[] = [
  {
    channelId: "ch-all-hands",
    slug: "#all-hands-status",
    title: "Cross-Team Program Status",
    team: "CROSS_TEAM_PROGRAM",
    requiredRepoScopes: ["local_gh_repo_1001"],
    dekKeyId: "dek-pqc-aurora-collab-v1",
  },
  {
    channelId: "ch-design-ee",
    slug: "#design-ee-layout",
    title: "Design — Schematic & PCB",
    team: "DESIGN",
    requiredRepoScopes: ["local_gh_repo_1002"],
    dekKeyId: "dek-pqc-aurora-collab-v1",
  },
  {
    channelId: "ch-dev-hil",
    slug: "#dev-hil-bringup",
    title: "Development — HIL & PCB Bring-Up",
    team: "DEVELOPMENT_BRINGUP",
    requiredRepoScopes: ["local_gh_repo_1003"],
    dekKeyId: "dek-pqc-aurora-collab-v1",
  },
  {
    channelId: "ch-sw-fw",
    slug: "#sw-firmware-hal",
    title: "Software — PinMux & SVD Firmware",
    team: "SOFTWARE_FIRMWARE",
    requiredRepoScopes: ["local_gh_repo_1002"],
    dekKeyId: "dek-pqc-aurora-collab-v1",
  },
  {
    channelId: "ch-mfg-cam",
    slug: "#mfg-cam-fab",
    title: "Manufacturing — DFM, CAM & Fab",
    team: "MANUFACTURING",
    requiredRepoScopes: ["local_gh_repo_1005"],
    dekKeyId: "dek-pqc-aurora-collab-v1",
  },
  {
    channelId: "ch-ops-plm",
    slug: "#ops-plm-supply",
    title: "Operations — BOM, CTB & Supply Chain",
    team: "OPERATIONS_SUPPLY_CHAIN",
    requiredRepoScopes: ["local_gh_repo_1005"],
    dekKeyId: "dek-pqc-aurora-collab-v1",
  },
];

export const SEEDED_RAW_MESSAGES: readonly CollabThreadMessage[] = [
  {
    messageId: "msg-01",
    channelId: "ch-all-hands",
    authorHmac: "actor_hmac_0001lead",
    body: "DVT thermal pad check linking @U1_SIC_HS layout to @ECO-2026-104 second-source die.",
    dekEnvelopePrefix: "sigv1.collab-dek.9f12a4",
    anchors: [
      {
        anchorId: "anc-pcb-u1-sic",
        domain: "PCB",
        entityId: "@U1_SIC_HS",
        commitSha: "a81f90c",
        xNm: 42_500_000n,
        yNm: 18_250_000n,
        zNm: 0n,
        requiredProviderRepoId: "local_gh_repo_1003",
        redacted: false,
      },
      {
        anchorId: "anc-ops-eco-104",
        domain: "PLM_OPS",
        entityId: "@ECO-2026-104",
        commitSha: "7f22d19",
        xNm: 0n,
        yNm: 0n,
        zNm: 0n,
        requiredProviderRepoId: "local_gh_repo_1005",
        redacted: false,
      },
    ],
    redactedAnchorCount: 0,
    createdAtUtcMs: 1_795_200_000_000n,
  },
  {
    messageId: "msg-02",
    channelId: "ch-dev-hil",
    authorHmac: "actor_hmac_0002pcb",
    body: "Measured 38 V/ns dV/dt edge at @TP_VGS_HS during 800V double-pulse bench test.",
    dekEnvelopePrefix: "sigv1.collab-dek.44b01e",
    anchors: [
      {
        anchorId: "anc-pcb-tp-vgs",
        domain: "PCB",
        entityId: "@TP_VGS_HS",
        commitSha: "b192e04",
        xNm: 12_800_000n,
        yNm: 9_400_000n,
        zNm: 0n,
        requiredProviderRepoId: "local_gh_repo_1003",
        redacted: false,
      },
    ],
    redactedAnchorCount: 0,
    createdAtUtcMs: 1_795_260_000_000n,
  },
];

export const MESSAGE_GRID_COLUMNS: readonly SigGridColumn[] = [
  { key: "author", header: "Pseudonym", mono: true },
  { key: "dek", header: "Hybrid PQC DEK", mono: true },
  { key: "body", header: "Decrypted Thread Message" },
  { key: "anchors", header: "Scoped @entity Anchors", mono: true },
];

export function authorizedReposForViewer(role: CollabViewerRole): readonly string[] {
  return role === "ORG_ADMIN"
    ? ["local_gh_repo_1001", "local_gh_repo_1002", "local_gh_repo_1003", "local_gh_repo_1004", "local_gh_repo_1005"]
    : ["local_gh_repo_1001", "local_gh_repo_1003"];
}

export function maskMessageForRepos(
  msg: CollabThreadMessage,
  authorizedRepos: readonly string[],
): CollabThreadMessage {
  let redactedCount = 0;
  const maskedAnchors: CollabEntityAnchor[] = msg.anchors.map((anc) => {
    if (authorizedRepos.includes(anc.requiredProviderRepoId)) {
      return anc;
    }
    redactedCount += 1;
    return {
      ...anc,
      entityId: "[Restricted Entity — Access Required]",
      commitSha: "REDACTED",
      xNm: 0n,
      yNm: 0n,
      zNm: 0n,
      redacted: true,
    };
  });
  return { ...msg, anchors: maskedAnchors, redactedAnchorCount: redactedCount };
}

export function createHuddleTicketState(
  channelId: string,
  active: boolean,
  followPresenterViewport: boolean,
  nowUtcMs: bigint,
  requestedTtlMs: bigint = MAX_HUDDLE_TTL_MS,
): HuddleSessionState {
  const clampedTtl = requestedTtlMs > MAX_HUDDLE_TTL_MS ? MAX_HUDDLE_TTL_MS : requestedTtlMs;
  const expiresAtUtcMs = nowUtcMs + clampedTtl;
  return {
    active,
    huddleRoomId: `hud-aurora-${channelId}`,
    sfuTurnsEndpoint: "turns:huddle.signal.local:443",
    ticketPreview: active ? `ed25519.sfu.${expiresAtUtcMs.toString()}.norecord` : "idle",
    expiresAtUtcMs,
    followPresenterViewport,
    geometryRecordingDisabled: true,
  };
}

export function toCanvasJumpEvent(anchor: CollabEntityAnchor): CanvasJumpEvent | null {
  if (anchor.redacted) {
    return null;
  }
  return {
    anchorId: anchor.anchorId,
    domain: anchor.domain,
    entityId: anchor.entityId,
    commitSha: anchor.commitSha,
    xNm: anchor.xNm,
    yNm: anchor.yNm,
    zNm: anchor.zNm,
  };
}

export function buildTeamChannelsHuddleVm(
  viewerRole: CollabViewerRole,
  selectedChannelId: string,
  messages: readonly CollabThreadMessage[],
  huddle: HuddleSessionState,
): TeamChannelsHuddleVm {
  const repos = authorizedReposForViewer(viewerRole);
  const visibleChannels = SEEDED_COLLAB_CHANNELS.filter((ch) =>
    ch.requiredRepoScopes.every((scope) => repos.includes(scope)),
  );
  const activeChannel =
    visibleChannels.find((ch) => ch.channelId === selectedChannelId) ?? visibleChannels[0]!;

  const visibleMessages = messages
    .filter((m) => m.channelId === activeChannel.channelId)
    .map((m) => maskMessageForRepos(m, repos));

  const visibleClickableAnchors = visibleMessages
    .flatMap((m) => m.anchors)
    .filter((a) => !a.redacted);

  const messageRows: SigGridRow[] = visibleMessages.map((m) => ({
    id: m.messageId,
    author: m.authorHmac,
    dek: m.dekEnvelopePrefix,
    body: m.body,
    anchors: m.anchors.map((a) => (a.redacted ? a.entityId : `${a.entityId}@${a.commitSha}`)).join(", "),
  }));

  return {
    viewerRole,
    viewerRoles: ["ORG_ADMIN", "PCB_CONTRACTOR"],
    visibleChannels,
    activeChannel,
    visibleMessages,
    visibleClickableAnchors,
    messageColumns: MESSAGE_GRID_COLUMNS,
    messageRows,
    huddle,
    huddleTone: huddle.active ? "ok" : "neutral",
    huddleBadge: huddle.active
      ? `LIVE SFU (${huddle.sfuTurnsEndpoint} • TTL <=15m • No-Record)`
      : "Huddle Idle (turns:443 Ready)",
    scopeSummaryBadge: `${viewerRole}: ${visibleChannels.length}/${SEEDED_COLLAB_CHANNELS.length} Scoped Channels`,
  };
}
