// Module: web/libs/sim-viewer/src/lib/waveform-canvas/waveform-canvas.model.ts — Arrow IPC Decimated Waveform & Dual-Cursor Models
// Invariants: Enforces <=4096 display pixel buckets (F-069); timestamps in bigint picoseconds (_ps) and voltages in bigint microvolts (_uv).
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigStatusTone } from "@signal/ui-kit";

export interface DecimatedArrowBucket {
  readonly timePs: bigint;
  readonly minUv: bigint;
  readonly maxUv: bigint;
  readonly sampleUv: bigint;
}

export interface ParsedArrowWaveformFrame {
  readonly valid: boolean;
  readonly bucketCount: number;
  readonly globalMinUv: bigint;
  readonly globalMaxUv: bigint;
  readonly svgPolylinePoints: string;
  readonly buckets: readonly DecimatedArrowBucket[];
  readonly errorReason: string;
}

export interface WaveformCursorReadout {
  readonly cursorATimePs: bigint;
  readonly cursorBTimePs: bigint;
  readonly deltaTimePs: bigint;
  readonly frequencyHz: bigint;
  readonly deltaVoltageUv: bigint;
  readonly slewRateText: string;
  readonly readoutBadge: string;
}

export interface WaveformCanvasVm {
  readonly frame: ParsedArrowWaveformFrame;
  readonly cursor: WaveformCursorReadout;
  readonly bucketCapBadge: string;
  readonly bucketCapTone: SigStatusTone;
  readonly probeNets: readonly string[];
  readonly activeProbeNet: string;
  readonly cursorAPxX: number;
  readonly cursorBPxX: number;
}
