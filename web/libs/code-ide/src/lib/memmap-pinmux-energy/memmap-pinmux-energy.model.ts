// Module: web/libs/code-ide/src/lib/memmap-pinmux-energy/memmap-pinmux-energy.model.ts — FEAT-054/055/056 Models
// Invariants: 64-bit physical addresses and per-symbol firmware energies (_nj) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface MemoryMapRegion {
  readonly regionId: string;
  readonly sourceKind: "CMSIS_SVD" | "LINKER_LD" | "DEVICETREE_DTS";
  readonly startAddr: bigint;
  readonly endAddr: bigint;
}

export interface FirmwareEnergySymbol {
  readonly symbolName: string;
  readonly invocationsPerSec: bigint;
  readonly energyPerCallNj: bigint;
}

export interface MemmapPinmuxEnergyVm {
  readonly satPinmuxSolved: boolean;
  readonly deepSleepEnabled: boolean;
  readonly memmapOverlapCount: number;
  readonly pinmuxConflictCount: number;
  readonly totalDutyEnergyPerSecNj: bigint;
  readonly estimatedBatteryYears: number;
  readonly halSnippetPreview: string;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly energyBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
