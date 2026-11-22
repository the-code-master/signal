// Module: web/libs/library-studio/src/lib/padstack-footprint-step/padstack-footprint-step.component.ts — FEAT-076 & FEAT-071 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildPadstackFootprintStepVm } from "./padstack-footprint-step.logic";
import type { Ipc7351Density } from "./padstack-footprint-step.model";

@Component({
  selector: "sig-padstack-footprint-step",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./padstack-footprint-step.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PadstackFootprintStepComponent {
  readonly densities: readonly Ipc7351Density[] = ["MOST", "NOMINAL", "LEAST"];

  readonly activeDensity = signal<Ipc7351Density>("NOMINAL");
  readonly backdrillEnabled = signal<boolean>(true);

  readonly vm = computed(() =>
    buildPadstackFootprintStepVm(this.activeDensity(), this.backdrillEnabled()),
  );

  selectDensity(density: Ipc7351Density): void {
    this.activeDensity.set(density);
  }

  toggleBackdrill(): void {
    this.backdrillEnabled.update((v) => !v);
  }
}
