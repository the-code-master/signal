// Module: web/libs/sim-viewer/src/lib/what-if-scrubber/what-if-scrubber.model.ts — FEAT-085 Saltation-Adjoint Scrubber Models
// Invariants: Parameter values and compute latency use ES2020 bigint (_nh / _ps); enforces <15ms interactive budget.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";

export interface GhostFamilyCurve {
  readonly id: string;
  readonly deltaLabel: string;
  readonly paramNh: bigint;
  readonly peakRippleMv: number;
  readonly svgPoints: string;
  readonly isActiveCurve: boolean;
}

export interface WhatIfScrubberVm {
  readonly activeParamNh: bigint;
  readonly scrubPercent: number;
  readonly computeLatencyPs: bigint;
  readonly latencyBadge: string;
  readonly latencyTone: SigStatusTone;
  readonly adjointGradientBadge: string;
  readonly ghostCurves: readonly GhostFamilyCurve[];
}
