// Module: web/libs/sim-viewer/src/lib/waveform-canvas/waveform-canvas.logic.ts — <=4096-Bucket Arrow IPC Parser & Dual-Cursor Math
// Invariants: Rejects >4096-bucket frames (F-069); computes exact bigint Δt (_ps), 1/Δt (_hz), and dV/dt slew rate.
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type {
  DecimatedArrowBucket,
  ParsedArrowWaveformFrame,
  WaveformCanvasVm,
  WaveformCursorReadout,
} from "./waveform-canvas.model";

const ARROW_FRAME_HEADER_BYTES = 8;
const BYTES_PER_BUCKET = 32;
const MAX_DISPLAY_BUCKETS = 4096;
const SVG_WIDTH = 800;
const SVG_HEIGHT = 160;

export function encodeArrowWaveformFrame(buckets: readonly DecimatedArrowBucket[]): Uint8Array {
  const buf = new ArrayBuffer(ARROW_FRAME_HEADER_BYTES + buckets.length * BYTES_PER_BUCKET);
  const bytes = new Uint8Array(buf);
  bytes[0] = 0x53; // 'S'
  bytes[1] = 0x47; // 'G'
  bytes[2] = 0x41; // 'A'
  bytes[3] = 0x57; // 'W'
  const view = new DataView(buf);
  view.setUint32(4, buckets.length, true);
  buckets.forEach((b, idx) => {
    const off = ARROW_FRAME_HEADER_BYTES + idx * BYTES_PER_BUCKET;
    view.setBigInt64(off, b.timePs, true);
    view.setBigInt64(off + 8, b.minUv, true);
    view.setBigInt64(off + 16, b.maxUv, true);
    view.setBigInt64(off + 24, b.sampleUv, true);
  });
  return bytes;
}

export function parseDecimatedArrowFrame(bytes: Uint8Array): ParsedArrowWaveformFrame {
  if (bytes.byteLength < ARROW_FRAME_HEADER_BYTES) {
    return {
      valid: false,
      bucketCount: 0,
      globalMinUv: 0n,
      globalMaxUv: 0n,
      svgPolylinePoints: "",
      buckets: [],
      errorReason: "ARROW_FRAME_TRUNCATED",
    };
  }
  if (bytes[0] !== 0x53 || bytes[1] !== 0x47 || bytes[2] !== 0x41 || bytes[3] !== 0x57) {
    return {
      valid: false,
      bucketCount: 0,
      globalMinUv: 0n,
      globalMaxUv: 0n,
      svgPolylinePoints: "",
      buckets: [],
      errorReason: "ARROW_BAD_MAGIC",
    };
  }
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const bucketCount = view.getUint32(4, true);
  if (bucketCount === 0 || bucketCount > MAX_DISPLAY_BUCKETS) {
    return {
      valid: false,
      bucketCount,
      globalMinUv: 0n,
      globalMaxUv: 0n,
      svgPolylinePoints: "",
      buckets: [],
      errorReason: `ARROW_BUCKET_CAP_EXCEEDED: ${bucketCount} > ${MAX_DISPLAY_BUCKETS}`,
    };
  }
  if (bytes.byteLength < ARROW_FRAME_HEADER_BYTES + bucketCount * BYTES_PER_BUCKET) {
    return {
      valid: false,
      bucketCount,
      globalMinUv: 0n,
      globalMaxUv: 0n,
      svgPolylinePoints: "",
      buckets: [],
      errorReason: "ARROW_PAYLOAD_INCOMPLETE",
    };
  }

  const buckets: DecimatedArrowBucket[] = [];
  let globalMinUv = view.getBigInt64(ARROW_FRAME_HEADER_BYTES + 8, true);
  let globalMaxUv = view.getBigInt64(ARROW_FRAME_HEADER_BYTES + 16, true);

  for (let i = 0; i < bucketCount; i += 1) {
    const off = ARROW_FRAME_HEADER_BYTES + i * BYTES_PER_BUCKET;
    const timePs = view.getBigInt64(off, true);
    const minUv = view.getBigInt64(off + 8, true);
    const maxUv = view.getBigInt64(off + 16, true);
    const sampleUv = view.getBigInt64(off + 24, true);
    if (minUv < globalMinUv) globalMinUv = minUv;
    if (maxUv > globalMaxUv) globalMaxUv = maxUv;
    buckets.push({ timePs, minUv, maxUv, sampleUv });
  }

  const spanUv = globalMaxUv > globalMinUv ? globalMaxUv - globalMinUv : 1n;
  const denomIdx = Math.max(1, bucketCount - 1);
  const points = buckets.map((b, idx) => {
    const x = Math.round((idx / denomIdx) * (SVG_WIDTH - 24)) + 12;
    const normMilli = Number(((b.sampleUv - globalMinUv) * 1000n) / spanUv);
    const y = Math.round(SVG_HEIGHT - 16 - (normMilli / 1000) * (SVG_HEIGHT - 32));
    return `${x},${y}`;
  });

  return {
    valid: true,
    bucketCount,
    globalMinUv,
    globalMaxUv,
    svgPolylinePoints: points.join(" "),
    buckets,
    errorReason: "",
  };
}

export function computeDualCursorReadout(
  buckets: readonly DecimatedArrowBucket[],
  idxA: number,
  idxB: number,
): WaveformCursorReadout {
  if (buckets.length === 0) {
    return {
      cursorATimePs: 0n,
      cursorBTimePs: 0n,
      deltaTimePs: 0n,
      frequencyHz: 0n,
      deltaVoltageUv: 0n,
      slewRateText: "0 mV/ns",
      readoutBadge: "No samples",
    };
  }
  const safeA = Math.max(0, Math.min(buckets.length - 1, idxA));
  const safeB = Math.max(0, Math.min(buckets.length - 1, idxB));
  const a = buckets[safeA];
  const b = buckets[safeB];
  const deltaTimePs = b.timePs - a.timePs;
  const absDtPs = deltaTimePs < 0n ? -deltaTimePs : deltaTimePs;
  const frequencyHz = absDtPs > 0n ? 1_000_000_000_000n / absDtPs : 0n;
  const deltaVoltageUv = b.sampleUv - a.sampleUv;
  // 1 uV / 1 ps = 10^-6 V / 10^-12 s = 10^6 V/s = 1 mV/ns:
  const slewMvPerNs = absDtPs > 0n ? (deltaVoltageUv * 1_000n) / absDtPs : 0n;
  const slewWhole = slewMvPerNs / 1_000n;
  const slewText = `${slewWhole.toString(10)} V/ns`;
  return {
    cursorATimePs: a.timePs,
    cursorBTimePs: b.timePs,
    deltaTimePs,
    frequencyHz,
    deltaVoltageUv,
    slewRateText: slewText,
    readoutBadge: `Δt=${deltaTimePs.toString(10)} ps | 1/Δt=${(frequencyHz / 1_000n).toString(10)} kHz | dV/dt=${slewText}`,
  };
}

export function createDefaultBuckWaveformBytes(): Uint8Array {
  const buckets: DecimatedArrowBucket[] = [];
  for (let i = 0; i < 64; i += 1) {
    const timePs = BigInt(i) * 10_000n;
    const phase = (i % 16) / 16;
    const wave = Math.sin(phase * Math.PI * 2);
    const sampleUv = 400_000_000n + BigInt(Math.round(wave * 380_000_000));
    buckets.push({
      timePs,
      minUv: sampleUv - 5_000_000n,
      maxUv: sampleUv + 5_000_000n,
      sampleUv,
    });
  }
  return encodeArrowWaveformFrame(buckets);
}

export function buildWaveformCanvasVm(
  frameBytes: Uint8Array,
  cursorAIdx: number,
  cursorBIdx: number,
  activeProbeNet: string,
): WaveformCanvasVm {
  const frame = parseDecimatedArrowFrame(frameBytes);
  const cursor = computeDualCursorReadout(frame.buckets, cursorAIdx, cursorBIdx);
  const denom = Math.max(1, frame.bucketCount - 1);
  return {
    frame,
    cursor,
    bucketCapBadge: frame.valid
      ? `Arrow IPC: ${frame.bucketCount} / 4096 Decimated Buckets`
      : frame.errorReason,
    bucketCapTone: frame.valid ? "ok" : "danger",
    probeNets: ["HV_SW_PHASE_U", "GATE_DRV_HS_U", "I_L_PHASE_U", "VOUT_DC_LINK"],
    activeProbeNet,
    cursorAPxX: Math.round((Math.min(cursorAIdx, denom) / denom) * (SVG_WIDTH - 24)) + 12,
    cursorBPxX: Math.round((Math.min(cursorBIdx, denom) / denom) * (SVG_WIDTH - 24)) + 12,
  };
}
