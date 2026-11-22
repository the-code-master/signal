// Module: web/libs/sim-viewer/src/lib/virtual-bench/virtual-bench.model.ts — FEAT-081 Virtual Bench Instrument Deck Models
// Invariants: RBW and RF sweep frequencies stored strictly as ES2020 bigint Hertz (_hz); trigger levels in bigint microvolts (_uv).
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type BenchInstrumentMode = "DSO_4CH" | "LOGIC_32CH" | "FFT_SPECTRUM" | "VNA_2PORT_SMITH";

export interface BenchModeOption {
  readonly id: BenchInstrumentMode;
  readonly label: string;
}

export interface BenchMetricPill {
  readonly id: string;
  readonly label: string;
  readonly tone: SigStatusTone;
}

export interface SmithImpedancePoint {
  readonly freqHz: bigint;
  readonly zRealOhm: number;
  readonly zImagOhm: number;
  readonly vswr: number;
  readonly returnLossDb: number;
}

export interface VirtualBenchVm {
  readonly activeMode: BenchInstrumentMode;
  readonly modes: readonly BenchModeOption[];
  readonly instrumentTitle: string;
  readonly metrics: readonly BenchMetricPill[];
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
