// Module: web/libs/ic/src/lib/upf-cpo-cryo-sec/upf-cpo-cryo-sec.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildUpfCpoCryoSecVm } from "./upf-cpo-cryo-sec.logic";

@Component({
  selector: "sig-upf-cpo-cryo-sec",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./upf-cpo-cryo-sec.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class UpfCpoCryoSecComponent {
  readonly domMaskingEnabled = signal<boolean>(true);
  readonly cryo4kActive = signal<boolean>(true);

  readonly vm = computed(() =>
    buildUpfCpoCryoSecVm(this.domMaskingEnabled(), this.cryo4kActive()),
  );

  toggleDomMasking(): void {
    this.domMaskingEnabled.update((v) => !v);
  }

  toggleCryo4k(): void {
    this.cryo4kActive.update((v) => !v);
  }
}
