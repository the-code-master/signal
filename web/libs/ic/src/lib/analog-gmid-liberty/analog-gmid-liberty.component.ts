// Module: web/libs/ic/src/lib/analog-gmid-liberty/analog-gmid-liberty.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildAnalogGmidLibertyVm } from "./analog-gmid-liberty.logic";
import type { InversionRegionPreset } from "./analog-gmid-liberty.model";

@Component({
  selector: "sig-analog-gmid-liberty",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./analog-gmid-liberty.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AnalogGmidLibertyComponent {
  readonly region = signal<InversionRegionPreset>("MODERATE_INVERSION_14");
  readonly bspdnEnabled = signal<boolean>(true);

  readonly vm = computed(() =>
    buildAnalogGmidLibertyVm(this.region(), this.bspdnEnabled()),
  );

  selectRegion(reg: InversionRegionPreset): void {
    this.region.set(reg);
  }

  toggleBspdn(): void {
    this.bspdnEnabled.update((v) => !v);
  }
}
