// Module: web/apps/signal-web/src/app/selection-query/selection-query.component.ts — FEAT-099 Selection Query Toolbar Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigStatusPillComponent } from "@signal/ui-kit";
import { evaluateSelectionQuery } from "./selection-query.logic";

@Component({
  selector: "sig-selection-query-toolbar",
  standalone: true,
  imports: [SigStatusPillComponent],
  templateUrl: "./selection-query.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SelectionQueryToolbarComponent {
  readonly queryText = signal<string>("type:cap AND voltage_rating < 2*net_max_v");
  readonly selectionMatched = output<readonly string[]>();

  readonly vm = computed(() => evaluateSelectionQuery(this.queryText()));

  onQueryInput(event: Event): void {
    this.queryText.set((event.target as HTMLInputElement).value);
    this.selectionMatched.emit(this.vm().matchedEntityIds);
  }

  applyPreset(preset: string): void {
    this.queryText.set(preset);
    this.selectionMatched.emit(this.vm().matchedEntityIds);
  }
}
