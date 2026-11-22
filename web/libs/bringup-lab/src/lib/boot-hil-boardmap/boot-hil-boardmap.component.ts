// Module: web/libs/bringup-lab/src/lib/boot-hil-boardmap/boot-hil-boardmap.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildBootHilBoardmapVm } from "./boot-hil-boardmap.logic";
import type { HilFaultScenario } from "./boot-hil-boardmap.model";

@Component({
  selector: "sig-boot-hil-boardmap",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./boot-hil-boardmap.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BootHilBoardmapComponent {
  readonly hilScenario = signal<HilFaultScenario>("VDD_BROWNOUT_DIP");
  readonly selectedTpId = signal<string>("TP101");

  readonly vm = computed(() =>
    buildBootHilBoardmapVm(this.hilScenario(), this.selectedTpId()),
  );

  selectHil(scenario: HilFaultScenario): void {
    this.hilScenario.set(scenario);
  }

  selectTestpoint(tpId: string): void {
    this.selectedTpId.set(tpId);
  }
}
