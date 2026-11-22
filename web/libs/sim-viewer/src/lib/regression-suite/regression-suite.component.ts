// Module: web/libs/sim-viewer/src/lib/regression-suite/regression-suite.component.ts — FEAT-086 Regression Suite Console Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import {
  buildRegressionSuiteVm,
  DEFAULT_ASSERTION_EXPRESSIONS,
  DEFAULT_SIM_METRICS,
} from "./regression-suite.logic";

@Component({
  selector: "sig-regression-suite",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./regression-suite.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class RegressionSuiteComponent {
  readonly customRule = signal<string>("phase_margin_deg > 45");
  readonly extraRules = signal<readonly string[]>(DEFAULT_ASSERTION_EXPRESSIONS);

  readonly vm = computed(() => buildRegressionSuiteVm(this.extraRules(), DEFAULT_SIM_METRICS));

  onRuleInput(event: Event): void {
    this.customRule.set((event.target as HTMLInputElement).value);
  }

  appendRule(): void {
    const trimmed = this.customRule().trim();
    if (trimmed) {
      this.extraRules.update((prev) => [...prev, trimmed]);
    }
  }
}
