// Module: web/libs/canvas-core/src/dual-plane-client/dual-plane-client.logic.spec.ts — Dual-Plane & QOI Unit Tests
// Invariants: Verifies HEVC 4:4:4 -> AV1 Dual-Luma -> Zstd-QOI fallback, H3 -> turns:443 -> WSS, QOI opcodes, and 80ms settling.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import { describe, expect, it } from "vitest";
import {
  advanceSettlingStateMachine,
  createInitialSettlingState,
  decodeQoiRgbaTile,
  decodeQoiTileHeader,
  negotiateDualPlaneStream,
  onMotionInputSample,
  selectViewportTransport,
} from "./dual-plane-client.logic";

function buildTestQoi2x2(): Uint8Array {
  // 2x2 RGBA QOI image testing QOI_OP_RGBA, QOI_OP_DIFF, QOI_OP_RUN, and QOI_OP_INDEX:
  // Header: 'q','o','i','f', width=2, height=2, channels=4, colorspace=0
  // Pixel 0: QOI_OP_RGBA (10, 20, 30, 255) -> hash = (30 + 100 + 210 + 2805) & 63 = 3145 & 63 = 9
  // Pixel 1: QOI_OP_DIFF (+1, 0, -1) -> (11, 20, 29, 255) -> dr=1(+2=3), dg=0(+2=2), db=-1(+2=1) => 0x40 | (3<<4) | (2<<2) | 1 = 0x79
  // Pixel 2: QOI_OP_RUN (run=0 => repeats Pixel 1 once) => 0xc0
  // Pixel 3: QOI_OP_INDEX (slot 9 => recalls Pixel 0: 10, 20, 30, 255) => 0x09
  return new Uint8Array([
    0x71, 0x6f, 0x69, 0x66,
    0x00, 0x00, 0x00, 0x02,
    0x00, 0x00, 0x00, 0x02,
    0x04, 0x00,
    0xff, 10, 20, 30, 255,
    0x79,
    0xc0,
    0x09,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
  ]);
}

describe("Dual-Plane stream negotiation & QOI RGBA8 decoder", () => {
  it("prioritizes HEVC 4:4:4 HW -> AV1 Dual-Luma HW -> Zstd-QOI lossless tiles", () => {
    const net = { webTransportAvailable: true, udpBlockedByFirewall: false, turns443Reachable: true };
    const apple = negotiateDualPlaneStream(
      { hevc444PowerEfficient: true, av1HardwareAccelerated: true, losslessWebpSupported: true, devicePixelRatioX100: 200, secondaryMonitorCount: 1 },
      net,
    );
    expect(apple.motionCodec).toBe("HEVC_444_HW");
    expect(apple.transport).toBe("H3_WEBTRANSPORT");
    expect(apple.singleGpuAtlasMonitors).toBe(2);

    const nvdec = negotiateDualPlaneStream(
      { hevc444PowerEfficient: false, av1HardwareAccelerated: true, losslessWebpSupported: true, devicePixelRatioX100: 100, secondaryMonitorCount: 0 },
      { webTransportAvailable: true, udpBlockedByFirewall: true, turns443Reachable: true },
    );
    expect(nvdec.motionCodec).toBe("AV1_DUAL_LUMA_HW");
    expect(nvdec.transport).toBe("TURNS_443_DATAGRAM");

    const fallback = negotiateDualPlaneStream(
      { hevc444PowerEfficient: false, av1HardwareAccelerated: false, losslessWebpSupported: false, devicePixelRatioX100: 100, secondaryMonitorCount: 0 },
      { webTransportAvailable: false, udpBlockedByFirewall: true, turns443Reachable: false },
    );
    expect(fallback.motionCodec).toBe("ZSTD_QOI_TILES_ONLY");
    expect(fallback.tileCodec).toBe("ZSTD_QOI_RGBA8");
    expect(fallback.transport).toBe("H2_WSS_BINARY");
    expect(selectViewportTransport({ webTransportAvailable: false, udpBlockedByFirewall: false, turns443Reachable: true })).toBe("TURNS_443_DATAGRAM");
  });

  it("decodes QOI header and reconstructs exact lossless RGBA8 pixels", () => {
    const qoi = buildTestQoi2x2();
    const hdr = decodeQoiTileHeader(qoi);
    expect(hdr).toEqual({ widthPx: 2, heightPx: 2, channels: 4, colorspace: 0 });

    const decoded = decodeQoiRgbaTile(qoi);
    expect(Array.from(decoded.rgbaBytes)).toEqual([
      10, 20, 30, 255,
      11, 20, 29, 255,
      11, 20, 29, 255,
      10, 20, 30, 255,
    ]);
  });

  it("snaps back from Plane 2 HW motion to Plane 1 lossless tiles after 80ms idle", () => {
    const init = createInitialSettlingState(80n);
    const moving = onMotionInputSample(init, 1_000n, true);
    expect(moving.activePlane).toBe("PLANE_2_HW_MOTION");
    expect(moving.awaitingLosslessSettle).toBe(true);

    const mid = advanceSettlingStateMachine(moving, 1_050n, 41n);
    expect(mid.activePlane).toBe("PLANE_2_HW_MOTION");

    const settled = advanceSettlingStateMachine(mid, 1_080n, 42n);
    expect(settled.activePlane).toBe("PLANE_1_LOSSLESS_TILES");
    expect(settled.settledFrameSeq).toBe(42n);
    expect(settled.awaitingLosslessSettle).toBe(false);
  });
});
