// Module: web/libs/plm-ops/src/lib/multi-bom-passport/multi-bom-passport.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildMultiBomPassportVm } from "./multi-bom-passport.logic";
import type { PlmBomViewMode } from "./multi-bom-passport.model";

@Component({
  selector: "sig-multi-bom-passport",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./multi-bom-passport.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MultiBomPassportComponent {
  readonly viewMode = signal<PlmBomViewMode>("MBOM");

  readonly vm = computed(() => buildMultiBomPassportVm(this.viewMode()));

  selectView(mode: PlmBomViewMode): void {
    this.viewMode.set(mode);
  }
}
