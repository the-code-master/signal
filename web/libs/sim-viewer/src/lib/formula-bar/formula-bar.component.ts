// Module: web/libs/sim-viewer/src/lib/formula-bar/formula-bar.component.ts — FEAT-082 Waveform Formula Bar Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildWaveformFormulaBarVm } from "./formula-bar.logic";

@Component({
  selector: "sig-waveform-formula-bar",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./formula-bar.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class WaveformFormulaBarComponent {
  readonly expression = signal<string>("integrate(v(out)*i(L1))");
  readonly formulaCommitted = output<string>();

  readonly vm = computed(() => buildWaveformFormulaBarVm(this.expression()));

  onFormulaInput(event: Event): void {
    this.expression.set((event.target as HTMLInputElement).value);
  }

  applySuggestion(preset: string): void {
    this.expression.set(preset);
    this.formulaCommitted.emit(preset);
  }
}
