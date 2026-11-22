// Module: web/libs/power-studio/src/lib/pdn-creepage-ir-hotloop/pdn-creepage-ir-hotloop.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildPdnCreepageIrHotloopVm } from "./pdn-creepage-ir-hotloop.logic";

@Component({
  selector: "sig-pdn-creepage-ir-hotloop",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./pdn-creepage-ir-hotloop.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PdnCreepageIrHotloopComponent {
  readonly slotRibAdded = signal<boolean>(true);
  readonly negativeGateBias = signal<boolean>(true);

  readonly vm = computed(() =>
    buildPdnCreepageIrHotloopVm(this.slotRibAdded(), this.negativeGateBias()),
  );

  toggleMilledSlot(): void {
    this.slotRibAdded.update((v) => !v);
  }

  toggleNegativeVee(): void {
    this.negativeGateBias.update((v) => !v);
  }
}
