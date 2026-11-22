// Module: web/libs/sim-viewer/src/lib/monte-carlo/monte-carlo.component.ts — FEAT-083 Monte Carlo Analytics Studio Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildMonteCarloStudioVm } from "./monte-carlo.logic";

@Component({
  selector: "sig-monte-carlo-studio",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./monte-carlo.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MonteCarloStudioComponent {
  readonly runCounts: readonly number[] = [100, 250, 1000];
  readonly selectedRuns = signal<number>(250);

  readonly vm = computed(() => buildMonteCarloStudioVm(this.selectedRuns()));

  selectRunCount(runs: number): void {
    this.selectedRuns.set(runs);
  }
}
