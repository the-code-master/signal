// Module: web/libs/safety-wcca/src/lib/safety-wcca-workbench/safety-wcca-workbench.model.ts — FEAT-035/036 Safety Host Models
// Invariants: Models ISO 26262 ASIL-A..D FMEDA SPFM/LFM/PMHF metrics and ZBDD fault-tree minimal cut-sets.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type AsilTargetLevel = "ASIL_B" | "ASIL_C" | "ASIL_D";

export interface SafetyWccaWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly asilTarget: AsilTargetLevel;
  readonly asilTargets: readonly AsilTargetLevel[];
  readonly redundantCrowbarEnabled: boolean;
  readonly spfmPct: number;
  readonly lfmPct: number;
  readonly pmhfFit: number;
  readonly fmedaBadge: string;
  readonly fmedaTone: SigStatusTone;
  readonly ftaCutSetBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
