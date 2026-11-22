// Module: web/libs/schematic/src/lib/schematic-workbench/schematic-workbench.model.ts — Schematic Workbench Container Models
// Invariants: Preserves WorkbenchDescriptor contract while organizing FEAT-001..FEAT-010 panels cleanly.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "@signal/canvas-core";

export type SchematicTabId =
  | "BEAUTIFY_BUS"
  | "LIVE_HUD"
  | "HIERARCHY_VARIANTS"
  | "PINSWAP_EQUATIONS"
  | "CIRCUIT_EXPLAINER";

export interface SchematicTabOption {
  readonly id: SchematicTabId;
  readonly label: string;
}

export interface SchematicWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activeTab: SchematicTabId;
  readonly tabs: readonly SchematicTabOption[];
  readonly statusBanner: string;
}
