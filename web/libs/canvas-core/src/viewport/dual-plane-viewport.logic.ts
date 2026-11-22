// Module: web/libs/canvas-core/src/viewport/dual-plane-viewport.logic.ts — Pure Dual-Plane Codec & BigInt HUD Formatter
// Invariants: Formats >2^53-1 nm coordinates with bigint math; integrates Dual-Plane negotiator, 120Hz HUD projection & volatile closure state.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import { negotiateDualPlaneStream } from "../dual-plane-client/dual-plane-client.logic";
import type { PlaneSettlingState } from "../dual-plane-client/dual-plane-client.model";
import { projectHudPrimitives } from "../hud-overlay/hud-overlay.logic";
import type { HudWorldPrimitive, ViewportTransformNm } from "../hud-overlay/hud-overlay.model";
import type { CloudClipboardEnvelope } from "../volatile-reconnect/volatile-reconnect.model";
import type { ViewportReticleVm } from "./dual-plane-viewport.model";

const DEFAULT_WORLD_PRIMITIVES: readonly HudWorldPrimitive[] = [
  {
    id: "hud-snap",
    kind: "SNAP_CROSSHAIR",
    originXNm: 9_007_199_254_760_000n,
    originYNm: -9_007_199_254_710_000n,
    targetXNm: 9_007_199_254_760_000n,
    targetYNm: -9_007_199_254_710_000n,
    label: "Snap: 25_000 nm Grid",
  },
  {
    id: "hud-river",
    kind: "CURRENT_RIVER_PARTICLE",
    originXNm: 9_007_199_254_750_000n,
    originYNm: -9_007_199_254_730_000n,
    targetXNm: 9_007_199_254_950_000n,
    targetYNm: -9_007_199_254_730_000n,
    label: "I(HV_BUS)",
    currentNa: 12_400_000_000n,
  },
  {
    id: "hud-tune",
    kind: "LENGTH_TUNE_GAUGE",
    originXNm: 9_007_199_254_800_000n,
    originYNm: -9_007_199_254_680_000n,
    targetXNm: 9_007_199_254_920_000n,
    targetYNm: -9_007_199_254_680_000n,
    label: "GATE_DIFF_PAIR",
    targetLengthNm: 18_500_000n,
    actualLengthNm: 18_514_000n,
  },
];

export interface ViewportStateSnapshot {
  readonly domainTitle: string;
  readonly cursorXNm: bigint;
  readonly cursorYNm: bigint;
  readonly traceWidthNm: bigint;
  readonly orbiting3d: boolean;
  readonly hudSignals: readonly string[];
  readonly settlingState: PlaneSettlingState;
  readonly pendingVolatileOps: number;
  readonly clipboardEnvelope: CloudClipboardEnvelope | null;
  readonly frameTick120Hz: number;
}

export function buildViewportReticleVm(snap: ViewportStateSnapshot): ViewportReticleVm {
  const session = negotiateDualPlaneStream(
    {
      hevc444PowerEfficient: true,
      av1HardwareAccelerated: true,
      losslessWebpSupported: true,
      devicePixelRatioX100: 200,
      secondaryMonitorCount: 0,
    },
    { webTransportAvailable: true, udpBlockedByFirewall: false, turns443Reachable: true },
  );
  const planeModeLabel =
    snap.orbiting3d || snap.settlingState.activePlane === "PLANE_2_HW_MOTION"
      ? "Plane 2: HW HEVC 4:4:4 / Dual-Luma 120 FPS"
      : "Plane 1: Lossless Zstd-QOI RGBA8 Dirty Tiles";
  const settlingBadge = snap.settlingState.awaitingLosslessSettle
    ? "Settling: 80 ms Pending"
    : `Settled Lossless #${snap.settlingState.settledFrameSeq.toString(10)}`;
  const transform: ViewportTransformNm = {
    panOriginXNm: 9_007_199_254_740_000n,
    panOriginYNm: -9_007_199_254_760_000n,
    nmPerPixel: 1_000n,
    viewportWidthPx: 960,
    viewportHeightPx: 420,
  };
  const clipboardBadge = snap.clipboardEnvelope
    ? `Cloud Clip: ${snap.clipboardEnvelope.envelopeId} (${snap.clipboardEnvelope.aeadCiphertextLength} B)`
    : "Cloud Clip: Idle (*.sigclip AEAD)";
  return {
    domainTitle: snap.domainTitle,
    planeModeLabel,
    streamSessionBadge: session.summaryBadge,
    settlingBadge,
    cursorXText: `X: ${snap.cursorXNm.toString(10)} nm`,
    cursorYText: `Y: ${snap.cursorYNm.toString(10)} nm`,
    traceWidthText: `Constraint: ${snap.traceWidthNm.toString(10)} nm`,
    watermarkBadge: "Zero-Leak DLP • Tardos + Constellation Active",
    volatileBufferBadge: `Volatile Closure Ring: ${snap.pendingVolatileOps}/256 ops (<=60s RAM)`,
    clipboardBadge,
    hudSignals: snap.hudSignals,
    projectedHudPrimitives: projectHudPrimitives(DEFAULT_WORLD_PRIMITIVES, transform, snap.frameTick120Hz),
  };
}
