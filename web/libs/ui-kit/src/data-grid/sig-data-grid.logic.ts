// Module: web/libs/ui-kit/src/data-grid/sig-data-grid.logic.ts — Pure Windowed Grid Slice Builder
// Invariants: Caps rendered DOM rows to maxVisibleRows (default 50, F-069).
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigGridRow } from "./sig-data-grid.model";

export function sliceVisibleGridRows(
  rows: readonly SigGridRow[],
  startIndex: number,
  maxVisibleRows = 50,
): readonly SigGridRow[] {
  const safeStart = Math.max(0, Math.min(startIndex, Math.max(0, rows.length - maxVisibleRows)));
  return rows.slice(safeStart, safeStart + maxVisibleRows);
}
