// Module: web/libs/fpga-studio/src/lib/die-io-sdc-planner/die-io-sdc-planner.model.ts — FEAT-051/052 FPGA I/O & SDC Models
// Invariants: Bank Vcco voltages (_uv) and SDC clock periods (_ps) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface FpgaIoBankRow {
  readonly bankId: string;
  readonly vccoUv: bigint;
  readonly ioStandard: string;
  readonly ssnMarginPct: number;
  readonly pcbEscapeCrossings: number;
}

export interface DieIoSdcPlannerVm {
  readonly autoPinSwapApplied: boolean;
  readonly grayCodeCdcSynchronizer: boolean;
  readonly primaryClockPeriodPs: bigint;
  readonly totalEscapeCrossings: number;
  readonly unsafeCdcPaths: number;
  readonly ioSdcBadge: string;
  readonly ioSdcTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
