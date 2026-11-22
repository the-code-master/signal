// Module: web/libs/safety-wcca/src/lib/safety-wcca-workbench/safety-wcca-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { WccaRadhardDeratingAgingComponent } from "../wcca-radhard-derating-aging/wcca-radhard-derating-aging.component";
import { buildSafetyWccaWorkbenchVm } from "./safety-wcca-workbench.logic";
import type { AsilTargetLevel } from "./safety-wcca-workbench.model";

@Component({
  selector: "sig-safety-wcca-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigDataGridComponent,
    DualPlaneViewportComponent,
    WccaRadhardDeratingAgingComponent,
  ],
  templateUrl: "./safety-wcca-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SafetyWccaWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly asilTarget = signal<AsilTargetLevel>("ASIL_D");
  readonly crowbarEnabled = signal<boolean>(true);

  readonly vm = computed(() =>
    buildSafetyWccaWorkbenchVm(
      this.workbench(),
      this.asilTarget(),
      this.crowbarEnabled(),
    ),
  );

  selectAsil(target: AsilTargetLevel): void {
    this.asilTarget.set(target);
  }

  toggleRedundantCrowbar(): void {
    this.crowbarEnabled.update((v) => !v);
  }
}
