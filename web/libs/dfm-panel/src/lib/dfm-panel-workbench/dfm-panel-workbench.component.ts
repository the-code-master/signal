// Module: web/libs/dfm-panel/src/lib/dfm-panel-workbench/dfm-panel-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { DrawingIctMslComponent } from "../drawing-ict-msl/drawing-ict-msl.component";
import { PanelDfaFatigueCoatingComponent } from "../panel-dfa-fatigue-coating/panel-dfa-fatigue-coating.component";
import { buildDfmPanelWorkbenchVm } from "./dfm-panel-workbench.logic";
import type { ProductionVolumeTier } from "./dfm-panel-workbench.model";

@Component({
  selector: "sig-dfm-panel-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    PanelDfaFatigueCoatingComponent,
    DrawingIctMslComponent,
  ],
  templateUrl: "./dfm-panel-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DfmPanelWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly volumeTier = signal<ProductionVolumeTier>("MASS_50000");

  readonly vm = computed(() =>
    buildDfmPanelWorkbenchVm(this.workbench(), this.volumeTier()),
  );

  selectVolume(tier: ProductionVolumeTier): void {
    this.volumeTier.set(tier);
  }
}
