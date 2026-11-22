// Module: web/libs/canvas-core/src/dual-plane-client/dual-plane-client.model.ts — Dual-Plane Codec, QOI Tile & Transport Models
// Invariants: Enforces <=4096px tile bounds, 80ms lossless settling window, and bigint frame/timestamp counters (F-066..F-068, F-072, F-126).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
export type NegotiatedMotionCodec = "HEVC_444_HW" | "AV1_DUAL_LUMA_HW" | "ZSTD_QOI_TILES_ONLY";
export type NegotiatedTileCodec = "ZSTD_QOI_RGBA8" | "LOSSLESS_WEBP_RGBA8";
export type ViewportTransportKind = "H3_WEBTRANSPORT" | "TURNS_443_DATAGRAM" | "H2_WSS_BINARY";
export type ActiveRenderPlane = "PLANE_1_LOSSLESS_TILES" | "PLANE_2_HW_MOTION";

export interface MediaCapabilitiesProbeInput {
  readonly hevc444PowerEfficient: boolean;
  readonly av1HardwareAccelerated: boolean;
  readonly losslessWebpSupported: boolean;
  readonly devicePixelRatioX100: number;
  readonly secondaryMonitorCount: number;
}

export interface TransportCandidateInput {
  readonly webTransportAvailable: boolean;
  readonly udpBlockedByFirewall: boolean;
  readonly turns443Reachable: boolean;
}

export interface NegotiatedDualPlaneSession {
  readonly motionCodec: NegotiatedMotionCodec;
  readonly tileCodec: NegotiatedTileCodec;
  readonly transport: ViewportTransportKind;
  readonly settlingDelayMs: bigint;
  readonly singleGpuAtlasMonitors: number;
  readonly summaryBadge: string;
}

export interface QoiTileHeader {
  readonly widthPx: number;
  readonly heightPx: number;
  readonly channels: 3 | 4;
  readonly colorspace: 0 | 1;
}

export interface DecodedRgbaTile {
  readonly header: QoiTileHeader;
  readonly rgbaBytes: Uint8Array;
}

export interface PlaneSettlingState {
  readonly activePlane: ActiveRenderPlane;
  readonly lastMotionUtcMs: bigint;
  readonly settlingDelayMs: bigint;
  readonly settledFrameSeq: bigint;
  readonly awaitingLosslessSettle: boolean;
}
