// Module: web/libs/sim-viewer/src/lib/stimulus-audition/stimulus-audition.logic.ts — Volatile PCM Synthesizer & Discrete RC A/B Filter
// Invariants: Synthesizes Float32Array buffers strictly in volatile JS RAM; zero Blob URLs or file download routes (FEAT-090).
// Spec: architecture/01-product-vision-and-features.md, architecture/07-security-encryption-dlp-and-sftp.md
import type {
  AuditionChannelAb,
  StimulusAuditionVm,
  StimulusPresetKind,
  VolatilePcmPair,
} from "./stimulus-audition.model";

function computeRms(buf: Float32Array): number {
  if (buf.length === 0) return 0;
  let sumSq = 0;
  for (let i = 0; i < buf.length; i += 1) {
    sumSq += buf[i] * buf[i];
  }
  return Number(Math.sqrt(sumSq / buf.length).toFixed(4));
}

export function synthesizeVolatilePcmPair(
  preset: StimulusPresetKind,
  sampleRateHz: bigint = 48_000n,
  durationPs: bigint = 50_000_000_000n,
): VolatilePcmPair {
  const rawSamples = Number((sampleRateHz * durationPs) / 1_000_000_000_000n);
  const sampleCount = Math.max(64, Math.min(8_192, rawSamples));
  const fs = Number(sampleRateHz);
  const channelAFloat32 = new Float32Array(sampleCount);
  const channelBFloat32 = new Float32Array(sampleCount);

  let lfsr = 0x7f;
  for (let i = 0; i < sampleCount; i += 1) {
    const t = i / fs;
    if (preset === "SINE_1KHZ_PURE") {
      channelAFloat32[i] = 0.8 * Math.sin(2 * Math.PI * 1000 * t);
    } else if (preset === "PRBS7_BITSTREAM") {
      if (i % 8 === 0) {
        const bit = ((lfsr >> 6) ^ (lfsr >> 5)) & 1;
        lfsr = ((lfsr << 1) | bit) & 0x7f;
      }
      channelAFloat32[i] = (lfsr & 1) === 1 ? 0.65 : -0.65;
    } else if (preset === "ISO_7637_LOAD_DUMP") {
      channelAFloat32[i] = 0.9 * Math.exp(-t * 85) * Math.sin(2 * Math.PI * 440 * t);
    } else {
      const fInst = 300 + 2_500 * (i / sampleCount);
      channelAFloat32[i] = 0.75 * Math.sin(2 * Math.PI * fInst * t);
    }
  }

  // Channel B: 1-pole discrete RC low-pass filter (fc = 1.2 kHz) emulating simulated analog filter output:
  const dt = 1 / fs;
  const rc = 1 / (2 * Math.PI * 1_200);
  const alpha = dt / (rc + dt);
  let yPrev = 0;
  for (let i = 0; i < sampleCount; i += 1) {
    yPrev = yPrev + alpha * (channelAFloat32[i] - yPrev);
    channelBFloat32[i] = yPrev;
  }

  return {
    sampleRateHz,
    durationPs,
    sampleCount,
    channelAFloat32,
    channelBFloat32,
    rmsA: computeRms(channelAFloat32),
    rmsB: computeRms(channelBFloat32),
  };
}

export function buildStimulusAuditionVm(
  preset: StimulusPresetKind,
  activeChannel: AuditionChannelAb,
  pair: VolatilePcmPair,
): StimulusAuditionVm {
  const activeBuf = activeChannel === "CHANNEL_A_STIMULUS" ? pair.channelAFloat32 : pair.channelBFloat32;
  const stride = Math.max(1, Math.floor(activeBuf.length / 80));
  const pts: string[] = [];
  for (let idx = 0; idx < activeBuf.length; idx += stride) {
    const x = Math.round(14 + (idx / Math.max(1, activeBuf.length - 1)) * 772);
    const y = Math.round(80 - activeBuf[idx] * 64);
    pts.push(`${x},${y}`);
  }
  const attenDb =
    pair.rmsA > 1e-9 && pair.rmsB > 1e-9
      ? Number((20 * Math.log10(pair.rmsB / pair.rmsA)).toFixed(2))
      : 0;

  return {
    preset,
    activeChannel,
    bufferSummaryBadge: `Volatile PCM: ${pair.sampleCount} samples @ ${pair.sampleRateHz.toString(10)} Hz`,
    abAttenuationBadge: `RMS A=${pair.rmsA} | RMS B=${pair.rmsB} (Δ=${attenDb} dB)`,
    dlpBadge: "Zero-Download RAM AudioContext Only",
    statusTone: "ok",
    svgPreviewPoints: pts.join(" "),
  };
}
