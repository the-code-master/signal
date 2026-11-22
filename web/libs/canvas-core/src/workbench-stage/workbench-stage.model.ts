// Module: web/libs/canvas-core/src/workbench-stage/workbench-stage.model.ts — Domain Workbench Stage Contract
// Invariants: Pure readonly configuration passed via Angular route data across all 25 lazy workbenches.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface WorkbenchKpiCard {
  readonly id: string;
  readonly label: string;
  readonly value: string;
  readonly tone: SigStatusTone;
}

export interface WorkbenchDescriptor {
  readonly slug: string;
  readonly title: string;
  readonly subtitle: string;
  readonly rustCrate: string;
  readonly subprojectRepo: string;
  readonly kpis: readonly WorkbenchKpiCard[];
  readonly hudSignals: readonly string[];
  readonly gridTitle: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
