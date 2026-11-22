// Module: web/libs/power-studio/src/lib/thermal-sequencing/thermal-sequencing.model.ts — FEAT-029/030 Thermal & Sequencing Models
// Invariants: Temperatures (_mkelvin), rail voltages (_uv), and ramp delays (_ps) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type CoolingAssemblyKind = "NATURAL_CONVECTION" | "FORCED_25_CFM_HEATSINK" | "WEG_LIQUID_COLD_PLATE";

export interface PowerSeqRailSlot {
  readonly railName: string;
  readonly nominalUv: bigint;
  readonly enableAtPs: bigint;
  readonly pgValidAtPs: bigint;
}

export interface ThermalSequencingVm {
  readonly coolingKind: CoolingAssemblyKind;
  readonly coolingKinds: readonly CoolingAssemblyKind[];
  readonly peakJunctionMkelvin: bigint;
  readonly maxRatedTjMkelvin: bigint;
  readonly monotonicSequencingClean: boolean;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
