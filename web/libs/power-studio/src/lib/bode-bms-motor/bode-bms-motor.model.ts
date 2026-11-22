// Module: web/libs/power-studio/src/lib/bode-bms-motor/bode-bms-motor.model.ts — FEAT-026/027/028 Bode, BMS & FOC Models
// Invariants: Crossover frequency (_hz), pack voltage (_uv), and q-axis current (_ma) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type LoopCompensatorTopology = "TYPE_II_OTA" | "TYPE_III_VOLTAGE_MODE" | "DIGITAL_2P2Z_PID";

export interface BodeBmsMotorVm {
  readonly compTopology: LoopCompensatorTopology;
  readonly compTopologies: readonly LoopCompensatorTopology[];
  readonly seriesCells: number;
  readonly parallelCells: number;
  readonly crossoverFreqHz: bigint;
  readonly phaseMarginDeg: number;
  readonly gainMarginDb: number;
  readonly packNominalVoltageUv: bigint;
  readonly focElectromagneticTorqueNm: number;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly motorBmsBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
