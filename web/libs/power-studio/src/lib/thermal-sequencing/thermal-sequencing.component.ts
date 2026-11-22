// Module: web/libs/power-studio/src/lib/thermal-sequencing/thermal-sequencing.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildThermalSequencingVm } from "./thermal-sequencing.logic";
import type { CoolingAssemblyKind } from "./thermal-sequencing.model";

@Component({
  selector: "sig-thermal-sequencing",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./thermal-sequencing.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ThermalSequencingComponent {
  readonly coolingKind = signal<CoolingAssemblyKind>("WEG_LIQUID_COLD_PLATE");

  readonly vm = computed(() => buildThermalSequencingVm(this.coolingKind()));

  selectCooling(kind: CoolingAssemblyKind): void {
    this.coolingKind.set(kind);
  }
}
