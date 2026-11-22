// Module: web/libs/power-studio/src/lib/bode-bms-motor/bode-bms-motor.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildBodeBmsMotorVm } from "./bode-bms-motor.logic";
import type { LoopCompensatorTopology } from "./bode-bms-motor.model";

@Component({
  selector: "sig-bode-bms-motor",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./bode-bms-motor.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BodeBmsMotorComponent {
  readonly compTopology = signal<LoopCompensatorTopology>("TYPE_III_VOLTAGE_MODE");

  readonly vm = computed(() => buildBodeBmsMotorVm(this.compTopology()));

  selectTopology(topo: LoopCompensatorTopology): void {
    this.compTopology.set(topo);
  }
}
