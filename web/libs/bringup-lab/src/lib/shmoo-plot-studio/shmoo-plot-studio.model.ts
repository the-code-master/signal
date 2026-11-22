// Module: web/libs/bringup-lab/src/lib/shmoo-plot-studio/shmoo-plot-studio.model.ts — FEAT-088 PVT Shmoo Models
// Invariants: Supply voltages (_uv), clock frequencies (_hz), and temperatures (_mkelvin) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface ShmooCellPoint {
  readonly cellId: string;
  readonly supplyVoltageUv: bigint;
  readonly clockFreqHz: bigint;
  readonly tempMkelvin: bigint;
  readonly simPassed: boolean;
  readonly ateSiliconPassed: boolean;
}

export interface ShmooPlotStudioVm {
  readonly tempMkelvin: bigint;
  readonly tempPresetsMkelvin: readonly bigint[];
  readonly cells: readonly ShmooCellPoint[];
  readonly selectedCell: ShmooCellPoint;
  readonly passRegionPct: number;
  readonly ateAgreementPct: number;
  readonly shmooBadge: string;
  readonly shmooTone: SigStatusTone;
  readonly selectedPointBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
