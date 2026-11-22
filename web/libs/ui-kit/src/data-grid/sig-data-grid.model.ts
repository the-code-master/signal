// Module: web/libs/ui-kit/src/data-grid/sig-data-grid.model.ts — Windowed High-Density Data Grid Contract
// Invariants: Supports >50-row virtual windowing without DOM bloat (F-069).
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
export interface SigGridColumn {
  readonly key: string;
  readonly header: string;
  readonly mono?: boolean;
}

export type SigGridRow = Readonly<Record<string, string>>;
