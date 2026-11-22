// Module: web/libs/schematic/src/lib/beautify-bus/beautify-bus.model.ts — FEAT-001 Auto-Beautify & FEAT-002 Typed Bus Models
// Invariants: Grid pitch and wirelength deltas stay in ES2020 bigint nanometers (_nm); bus widths bounded <=256 bits.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type BusProtocolKind = "RAW_VECTOR" | "AXI4_LITE" | "CAN_FD_DIFF" | "SPI_QUAD";

export interface BeautifyPreviewInput {
  readonly sheetId: string;
  readonly gridPitchNm: bigint;
  readonly crossingCountBefore: number;
  readonly crossingCountAfter: number;
  readonly wirelengthSavedNm: bigint;
  readonly applied: boolean;
}

export interface BusSliceExpansion {
  readonly valid: boolean;
  readonly bundleLabel: string;
  readonly bitWidth: number;
  readonly directionBadge: string;
  readonly expandedNets: readonly string[];
  readonly errorHint: string;
}

export interface BeautifyBusVm {
  readonly crossingBadge: string;
  readonly crossingTone: SigStatusTone;
  readonly wirelengthBadge: string;
  readonly gridPitchLabel: string;
  readonly applyButtonLabel: string;
  readonly busExpansion: BusSliceExpansion;
  readonly netColumns: readonly SigGridColumn[];
  readonly netRows: readonly SigGridRow[];
}
