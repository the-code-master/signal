// Module: web/libs/dfm-panel/src/lib/dfm-panel-workbench/dfm-panel-workbench.model.ts — FEAT-068 DFM Host Models
// Invariants: Unit should-cost (_micro_usd) stays in ES2020 bigint across volume tiers.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type ProductionVolumeTier = "PROTO_25" | "PILOT_1000" | "MASS_50000";

export interface DfmPanelWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly volumeTier: ProductionVolumeTier;
  readonly volumeTiers: readonly ProductionVolumeTier[];
  readonly unitShouldCostMicroUsd: bigint;
  readonly firstPassYieldPct: number;
  readonly costYieldBadge: string;
  readonly costYieldTone: SigStatusTone;
}
