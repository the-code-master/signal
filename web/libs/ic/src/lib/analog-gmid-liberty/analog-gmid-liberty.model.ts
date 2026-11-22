// Module: web/libs/ic/src/lib/analog-gmid-liberty/analog-gmid-liberty.model.ts — FEAT-041..044 Analog IC & BSPDN Models
// Invariants: Device W/L (_nm), transit frequencies (_hz), and BSPDN IR-drops (_uv) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type InversionRegionPreset = "WEAK_INVERSION_20" | "MODERATE_INVERSION_14" | "STRONG_INVERSION_8";

export interface AnalogGmidLibertyVm {
  readonly region: InversionRegionPreset;
  readonly regions: readonly InversionRegionPreset[];
  readonly backsidePdnEnabled: boolean;
  readonly deviceWidthNm: bigint;
  readonly deviceLengthNm: bigint;
  readonly gmOverIdPerV: number;
  readonly transitFreqHz: bigint;
  readonly pelgromSigmaVthMv: number;
  readonly bspdnIrDropUv: bigint;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly libertyBspdnBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
