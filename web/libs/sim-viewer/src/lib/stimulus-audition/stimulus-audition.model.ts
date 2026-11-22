// Module: web/libs/sim-viewer/src/lib/stimulus-audition/stimulus-audition.model.ts — FEAT-090 Volatile WebAudio PCM Audition Models
// Invariants: Sample rate (_hz) and duration (_ps) use ES2020 bigint; PCM stays strictly in volatile JS RAM (zero Blob/file download).
// Spec: architecture/01-product-vision-and-features.md, architecture/07-security-encryption-dlp-and-sftp.md
import type { SigStatusTone } from "@signal/ui-kit";

export type StimulusPresetKind = "SINE_1KHZ_PURE" | "PRBS7_BITSTREAM" | "ISO_7637_LOAD_DUMP" | "CHIRP_SWEEP";
export type AuditionChannelAb = "CHANNEL_A_STIMULUS" | "CHANNEL_B_FILTERED";

export interface VolatilePcmPair {
  readonly sampleRateHz: bigint;
  readonly durationPs: bigint;
  readonly sampleCount: number;
  readonly channelAFloat32: Float32Array;
  readonly channelBFloat32: Float32Array;
  readonly rmsA: number;
  readonly rmsB: number;
}

export interface StimulusAuditionVm {
  readonly preset: StimulusPresetKind;
  readonly activeChannel: AuditionChannelAb;
  readonly bufferSummaryBadge: string;
  readonly abAttenuationBadge: string;
  readonly dlpBadge: string;
  readonly statusTone: SigStatusTone;
  readonly svgPreviewPoints: string;
}
