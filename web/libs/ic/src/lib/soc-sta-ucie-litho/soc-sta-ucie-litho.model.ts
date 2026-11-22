// Module: web/libs/ic/src/lib/soc-sta-ucie-litho/soc-sta-ucie-litho.model.ts — FEAT-045..049 SoC, STA/ECO, UCIe & Litho Models
// Invariants: STA slacks (_fs), CTS skews (_fs), and signed chiplet halo widths (_nm) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface SocStaUcieLithoVm {
  readonly metalEcoApplied: boolean;
  readonly cmpDummyFillApplied: boolean;
  readonly worstNegativeSlackFs: bigint;
  readonly ctsGlobalSkewFs: bigint;
  readonly spareCellsRewired: number;
  readonly haloRingWidthNm: bigint;
  readonly ucieBumpCount: number;
  readonly cmpDensityPct: number;
  readonly staGateBadge: string;
  readonly staGateTone: SigStatusTone;
  readonly haloLithoBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
