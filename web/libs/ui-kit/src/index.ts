// Module: web/libs/ui-kit/src/index.ts — Standardized @signal/ui-kit Public Component Barrel
// Invariants: Exports shared 4-file <sig-*> HTML components consumed across all 23 Signal workbenches.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
export { SigPanelComponent } from "./panel/sig-panel.component";
export type { SigPanelVm } from "./panel/sig-panel.model";
export { SigStatusPillComponent } from "./status-pill/sig-status-pill.component";
export type { SigStatusPillVm, SigStatusTone } from "./status-pill/sig-status-pill.model";
export { SigSiInputComponent } from "./si-input/sig-si-input.component";
export { parseSiInputToNano } from "./si-input/sig-si-input.logic";
export type { SigSiParseResult } from "./si-input/sig-si-input.model";
export { SigDataGridComponent } from "./data-grid/sig-data-grid.component";
export type { SigGridColumn, SigGridRow } from "./data-grid/sig-data-grid.model";
export { SigDrawerComponent } from "./drawer/sig-drawer.component";
export type { SigDrawerVm } from "./drawer/sig-drawer.model";
