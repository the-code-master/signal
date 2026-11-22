// Module: web/libs/library-studio/src/lib/pin-symbol-splitter/pin-symbol-splitter.model.ts — Master Pin Table & FEAT-072 Splitter Models
// Invariants: 3D STEP lead centroids stay in ES2020 bigint nanometers (_nm); enforces 4-way Pin/Pad/SPICE/STEP bijection.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type SchematicSymbolStandard = "IEEE_315" | "IEC_60617";
export type PinFunctionalGroup = "POWER_GND" | "SERDES_HS" | "PWM_CTRL" | "SENSE_ANALOG";

export interface MasterPinRow {
  readonly pinNumber: string;
  readonly signalName: string;
  readonly group: PinFunctionalGroup;
  readonly padDesignator: string;
  readonly spicePortId: number;
  readonly stepLeadXNm: bigint;
  readonly stepLeadYNm: bigint;
}

export interface PartitionedSymbolBank {
  readonly unitIndex: number;
  readonly bankTitle: string;
  readonly pinNumbers: readonly string[];
}

export interface PinSymbolSplitterVm {
  readonly standard: SchematicSymbolStandard;
  readonly standardBadge: string;
  readonly bijectionValid: boolean;
  readonly bijectionBadge: string;
  readonly bijectionTone: SigStatusTone;
  readonly banks: readonly PartitionedSymbolBank[];
  readonly pinColumns: readonly SigGridColumn[];
  readonly pinRows: readonly SigGridRow[];
}
