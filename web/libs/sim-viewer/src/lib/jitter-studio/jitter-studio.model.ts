// Module: web/libs/sim-viewer/src/lib/jitter-studio/jitter-studio.model.ts — FEAT-084 SerDes RJ/DJ/PJ/DDJ & PAM4 Bathtub Models
// Invariants: All jitter components stored strictly as ES2020 bigint femtoseconds (_fs); PAM4 BER down to 1e-15.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type SerDesRatePreset = "112G_PAM4" | "224G_PAM4";

export interface JitterDecompositionInput {
  readonly ratePreset: SerDesRatePreset;
  readonly unitIntervalFs: bigint;
  readonly rjRmsFs: bigint;
  readonly djPpFs: bigint;
  readonly pjPpFs: bigint;
  readonly ddjPpFs: bigint;
}

export interface JitterStudioVm {
  readonly ratePreset: SerDesRatePreset;
  readonly tjAt1e15Fs: bigint;
  readonly eyeOpeningUi: number;
  readonly eyeBadge: string;
  readonly eyeTone: SigStatusTone;
  readonly breakdownBadges: readonly string[];
  readonly eyeColumns: readonly SigGridColumn[];
  readonly eyeRows: readonly SigGridRow[];
}
