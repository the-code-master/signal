// Module: web/libs/pcb/src/lib/pns-routing-meanders/pns-routing-meanders.model.ts — FEAT-012/015/017 PNS & Meander Models
// Invariants: All lengths (_nm), delays (_fs/_ps), and impedances (_mohm) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type PnsRouterMode = "SHOVE" | "WALKAROUND" | "BUS_TRUNK_MULTI_LANE";

export interface DiffPairFlightChannel {
  readonly pairId: string;
  readonly dieToBallDelayFs: bigint;
  readonly pcbRoutedDelayFs: bigint;
  readonly targetTotalDelayFs: bigint;
  readonly traceWidthNm: bigint;
  readonly zDiffMohm: bigint;
}

export interface PnsRoutingMeandersVm {
  readonly mode: PnsRouterMode;
  readonly modes: readonly PnsRouterMode[];
  readonly tromboneAmplitudeNm: bigint;
  readonly maxIntraPairSkewFs: bigint;
  readonly skewGateBadge: string;
  readonly skewGateTone: SigStatusTone;
  readonly tromboneSummary: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
