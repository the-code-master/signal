// Module: web/libs/ui-kit/src/data-grid/sig-data-grid.component.ts — Standardized <sig-data-grid>
// Invariants: <=120 LOC; Zoneless OnPush; caps rendered rows via sliceVisibleGridRows (F-069).
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input } from "@angular/core";
import { sliceVisibleGridRows } from "./sig-data-grid.logic";
import type { SigGridColumn, SigGridRow } from "./sig-data-grid.model";

@Component({
  selector: "sig-data-grid",
  standalone: true,
  templateUrl: "./sig-data-grid.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SigDataGridComponent {
  readonly columns = input.required<readonly SigGridColumn[]>();
  readonly rows = input.required<readonly SigGridRow[]>();
  readonly trackKey = input<string>("id");

  readonly visibleRows = computed(() => sliceVisibleGridRows(this.rows(), 0, 50));
}
