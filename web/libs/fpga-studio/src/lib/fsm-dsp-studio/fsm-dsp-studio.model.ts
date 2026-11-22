// Module: web/libs/fpga-studio/src/lib/fsm-dsp-studio/fsm-dsp-studio.model.ts — FEAT-057/060 FSM & Fixed-Point DSP Models
// Invariants: DSP sample rates (_hz) stay in ES2020 bigint; Q(m.n) SQNR and FSM deadlock checks stay deterministic.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type FsmEncodingStyle = "ONE_HOT_SAFE_DEFAULT" | "HAMMING3_SEU_TOLERANT";

export interface FsmDspStudioVm {
  readonly fsmEncoding: FsmEncodingStyle;
  readonly fsmEncodings: readonly FsmEncodingStyle[];
  readonly fracBitsN: number;
  readonly fracBitPresets: readonly number[];
  readonly sampleRateHz: bigint;
  readonly quantizedSqnrDb: number;
  readonly fsmDeadlockFree: boolean;
  readonly rtlPreviewLine: string;
  readonly dspFsmBadge: string;
  readonly dspFsmTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
