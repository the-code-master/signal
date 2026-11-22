// Module: web/libs/academy/src/lib/open-silicon-wizard/open-silicon-wizard.model.ts — Open-Silicon MPW Models (FEAT-115)
// Invariants: Pure readonly TypeScript types; 64-bit _nm shuttle tile dimensions use bigint ONLY.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type OpenSiliconPdkId = "SKYWATER_SKY130" | "GF180MCU" | "IHP_SG13G2";
export type ShuttleTileSlot = "TINYTAPEOUT_1X1" | "TINYTAPEOUT_2X2" | "CHIPIGNITE_USER_AREA";

export interface ShuttlePrecheckReport {
  readonly pdk: OpenSiliconPdkId;
  readonly tileSlot: ShuttleTileSlot;
  readonly dieWidthNm: bigint;
  readonly dieHeightNm: bigint;
  readonly maxWidthNm: bigint;
  readonly maxHeightNm: bigint;
  readonly metalDensityPct: number;
  readonly pinRingHarnessAligned: boolean;
  readonly antennaDiodesClean: boolean;
  readonly drcLvsZeroViolations: boolean;
  readonly tapeoutReady: boolean;
  readonly shuttleSubmissionDigest: string;
}

export interface OpenSiliconWizardVm {
  readonly pdks: readonly OpenSiliconPdkId[];
  readonly tileSlots: readonly ShuttleTileSlot[];
  readonly report: ShuttlePrecheckReport;
  readonly statusTone: SigStatusTone;
  readonly statusBadge: string;
  readonly checkColumns: readonly SigGridColumn[];
  readonly checkRows: readonly SigGridRow[];
}
