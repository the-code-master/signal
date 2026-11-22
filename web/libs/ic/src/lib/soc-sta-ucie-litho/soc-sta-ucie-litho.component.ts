// Module: web/libs/ic/src/lib/soc-sta-ucie-litho/soc-sta-ucie-litho.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildSocStaUcieLithoVm } from "./soc-sta-ucie-litho.logic";

@Component({
  selector: "sig-soc-sta-ucie-litho",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./soc-sta-ucie-litho.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SocStaUcieLithoComponent {
  readonly metalEcoApplied = signal<boolean>(true);
  readonly cmpDummyFillApplied = signal<boolean>(true);

  readonly vm = computed(() =>
    buildSocStaUcieLithoVm(this.metalEcoApplied(), this.cmpDummyFillApplied()),
  );

  toggleMetalEco(): void {
    this.metalEcoApplied.update((v) => !v);
  }

  toggleCmpFill(): void {
    this.cmpDummyFillApplied.update((v) => !v);
  }
}
