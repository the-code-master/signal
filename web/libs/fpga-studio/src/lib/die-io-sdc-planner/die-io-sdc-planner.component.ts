// Module: web/libs/fpga-studio/src/lib/die-io-sdc-planner/die-io-sdc-planner.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildDieIoSdcPlannerVm } from "./die-io-sdc-planner.logic";

@Component({
  selector: "sig-die-io-sdc-planner",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./die-io-sdc-planner.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DieIoSdcPlannerComponent {
  readonly autoPinSwapApplied = signal<boolean>(true);
  readonly grayCodeCdc = signal<boolean>(true);

  readonly vm = computed(() =>
    buildDieIoSdcPlannerVm(this.autoPinSwapApplied(), this.grayCodeCdc()),
  );

  togglePinSwap(): void {
    this.autoPinSwapApplied.update((v) => !v);
  }

  toggleCdcSync(): void {
    this.grayCodeCdc.update((v) => !v);
  }
}
