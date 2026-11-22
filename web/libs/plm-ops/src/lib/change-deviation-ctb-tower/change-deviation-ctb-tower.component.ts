// Module: web/libs/plm-ops/src/lib/change-deviation-ctb-tower/change-deviation-ctb-tower.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildChangeDeviationCtbTowerVm } from "./change-deviation-ctb-tower.logic";

@Component({
  selector: "sig-change-deviation-ctb-tower",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./change-deviation-ctb-tower.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ChangeDeviationCtbTowerComponent {
  readonly part11Signed = signal<boolean>(true);
  readonly deviationConsumed = signal<bigint>(450n);
  readonly activeStepNum = signal<number>(2);

  readonly vm = computed(() =>
    buildChangeDeviationCtbTowerVm(
      this.part11Signed(),
      this.deviationConsumed(),
      this.activeStepNum(),
    ),
  );

  togglePart11Signoff(): void {
    this.part11Signed.update((v) => !v);
  }

  simulateConsumeLot(): void {
    this.deviationConsumed.update((q) => (q >= 480n ? 450n : 480n));
  }

  selectWorkStep(stepNum: number): void {
    this.activeStepNum.set(stepNum);
  }
}
