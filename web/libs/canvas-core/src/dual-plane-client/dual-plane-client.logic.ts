// Module: web/libs/canvas-core/src/dual-plane-client/dual-plane-client.logic.ts — Capability-Probed Dual-Plane & Pure-TS QOI RGBA8 Decoder
// Invariants: Selects HEVC 4:4:4 HW -> AV1 Dual-Luma HW -> Zstd-QOI RGBA8; enforces 80ms settling and bomb-capped <=4096px QOI decode.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type {
  DecodedRgbaTile,
  MediaCapabilitiesProbeInput,
  NegotiatedDualPlaneSession,
  NegotiatedMotionCodec,
  PlaneSettlingState,
  QoiTileHeader,
  TransportCandidateInput,
  ViewportTransportKind,
} from "./dual-plane-client.model";

const QOI_HEADER_LEN = 14;
const QOI_END_MARKER_LEN = 8;
const MAX_TILE_DIM_PX = 4096;

export function selectViewportTransport(candidate: TransportCandidateInput): ViewportTransportKind {
  if (candidate.webTransportAvailable && !candidate.udpBlockedByFirewall) {
    return "H3_WEBTRANSPORT";
  }
  if (candidate.turns443Reachable) {
    return "TURNS_443_DATAGRAM";
  }
  return "H2_WSS_BINARY";
}

export function negotiateDualPlaneStream(
  probe: MediaCapabilitiesProbeInput,
  transportInput: TransportCandidateInput,
): NegotiatedDualPlaneSession {
  let motionCodec: NegotiatedMotionCodec = "ZSTD_QOI_TILES_ONLY";
  if (probe.hevc444PowerEfficient) {
    motionCodec = "HEVC_444_HW";
  } else if (probe.av1HardwareAccelerated) {
    motionCodec = "AV1_DUAL_LUMA_HW";
  }
  const tileCodec = "ZSTD_QOI_RGBA8";
  const transport = selectViewportTransport(transportInput);
  const singleGpuAtlasMonitors = Math.max(1, 1 + probe.secondaryMonitorCount);
  return {
    motionCodec,
    tileCodec,
    transport,
    settlingDelayMs: 80n,
    singleGpuAtlasMonitors,
    summaryBadge: `${motionCodec} + ${tileCodec} via ${transport} (${singleGpuAtlasMonitors}x Atlas)`,
  };
}

export function decodeQoiTileHeader(bytes: Uint8Array): QoiTileHeader {
  if (bytes.byteLength < QOI_HEADER_LEN + QOI_END_MARKER_LEN) {
    throw new Error("QOI_TRUNCATED: buffer shorter than 22-byte minimum");
  }
  if (bytes[0] !== 0x71 || bytes[1] !== 0x6f || bytes[2] !== 0x69 || bytes[3] !== 0x66) {
    throw new Error("QOI_BAD_MAGIC: expected 'qoif' header");
  }
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const widthPx = view.getUint32(4, false);
  const heightPx = view.getUint32(8, false);
  const channels = bytes[12];
  const colorspace = bytes[13];
  if (widthPx === 0 || heightPx === 0 || widthPx > MAX_TILE_DIM_PX || heightPx > MAX_TILE_DIM_PX) {
    throw new Error(`QOI_DIM_BOUNDS: ${widthPx}x${heightPx} exceeds 1..${MAX_TILE_DIM_PX}px cap`);
  }
  if (channels !== 3 && channels !== 4) {
    throw new Error(`QOI_BAD_CHANNELS: unsupported channel count ${channels}`);
  }
  if (colorspace !== 0 && colorspace !== 1) {
    throw new Error(`QOI_BAD_COLORSPACE: unsupported colorspace ${colorspace}`);
  }
  return { widthPx, heightPx, channels, colorspace };
}

export function decodeQoiRgbaTile(bytes: Uint8Array): DecodedRgbaTile {
  const header = decodeQoiTileHeader(bytes);
  const pixelCount = header.widthPx * header.heightPx;
  const rgbaBytes = new Uint8Array(pixelCount * 4);
  const index = new Uint8Array(64 * 4);
  const chunksLimit = bytes.byteLength - QOI_END_MARKER_LEN;

  for (let i = 0; i < 7; i += 1) {
    if (bytes[chunksLimit + i] !== 0x00) {
      throw new Error("QOI_BAD_END_MARKER: missing 7 zero trailer bytes");
    }
  }
  if (bytes[chunksLimit + 7] !== 0x01) {
    throw new Error("QOI_BAD_END_MARKER: missing trailing 0x01 marker");
  }

  let r = 0;
  let g = 0;
  let b = 0;
  let a = 255;
  let run = 0;
  let pos = QOI_HEADER_LEN;

  for (let px = 0; px < pixelCount; px += 1) {
    if (run > 0) {
      run -= 1;
    } else if (pos < chunksLimit) {
      const b1 = bytes[pos++];
      if (b1 === 0xfe) {
        r = bytes[pos++];
        g = bytes[pos++];
        b = bytes[pos++];
      } else if (b1 === 0xff) {
        r = bytes[pos++];
        g = bytes[pos++];
        b = bytes[pos++];
        a = bytes[pos++];
      } else if ((b1 & 0xc0) === 0x00) {
        const idx = (b1 & 0x3f) * 4;
        r = index[idx];
        g = index[idx + 1];
        b = index[idx + 2];
        a = index[idx + 3];
      } else if ((b1 & 0xc0) === 0x40) {
        r = (r + ((b1 >> 4) & 0x03) - 2) & 0xff;
        g = (g + ((b1 >> 2) & 0x03) - 2) & 0xff;
        b = (b + (b1 & 0x03) - 2) & 0xff;
      } else if ((b1 & 0xc0) === 0x80) {
        const b2 = bytes[pos++];
        const vg = (b1 & 0x3f) - 32;
        r = (r + vg - 8 + ((b2 >> 4) & 0x0f)) & 0xff;
        g = (g + vg) & 0xff;
        b = (b + vg - 8 + (b2 & 0x0f)) & 0xff;
      } else if ((b1 & 0xc0) === 0xc0) {
        run = b1 & 0x3f;
      }
      const hashSlot = ((r * 3 + g * 5 + b * 7 + a * 11) & 0x3f) * 4;
      index[hashSlot] = r;
      index[hashSlot + 1] = g;
      index[hashSlot + 2] = b;
      index[hashSlot + 3] = a;
    }
    const outIdx = px * 4;
    rgbaBytes[outIdx] = r;
    rgbaBytes[outIdx + 1] = g;
    rgbaBytes[outIdx + 2] = b;
    rgbaBytes[outIdx + 3] = a;
  }

  return { header, rgbaBytes };
}

export function createInitialSettlingState(settlingDelayMs = 80n): PlaneSettlingState {
  return {
    activePlane: "PLANE_1_LOSSLESS_TILES",
    lastMotionUtcMs: 0n,
    settlingDelayMs,
    settledFrameSeq: 0n,
    awaitingLosslessSettle: false,
  };
}

export function onMotionInputSample(
  state: PlaneSettlingState,
  nowUtcMs: bigint,
  motionSupported: boolean,
): PlaneSettlingState {
  if (!motionSupported) {
    return { ...state, lastMotionUtcMs: nowUtcMs, awaitingLosslessSettle: false };
  }
  return {
    ...state,
    activePlane: "PLANE_2_HW_MOTION",
    lastMotionUtcMs: nowUtcMs,
    awaitingLosslessSettle: true,
  };
}

export function advanceSettlingStateMachine(
  state: PlaneSettlingState,
  nowUtcMs: bigint,
  frameSeq: bigint,
): PlaneSettlingState {
  if (!state.awaitingLosslessSettle) {
    return state;
  }
  if (nowUtcMs - state.lastMotionUtcMs >= state.settlingDelayMs) {
    return {
      ...state,
      activePlane: "PLANE_1_LOSSLESS_TILES",
      settledFrameSeq: frameSeq,
      awaitingLosslessSettle: false,
    };
  }
  return state;
}
