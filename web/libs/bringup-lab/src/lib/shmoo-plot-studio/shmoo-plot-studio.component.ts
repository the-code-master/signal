// Module: web/libs/bringup-lab/src/lib/shmoo-plot-studio/shmoo-plot-studio.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildShmooPlotStudioVm } from "./shmoo-plot-studio.logic";

@Component({
  selector: "sig-shmoo-plot-studio",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./shmoo-plot-studio.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ShmooPlotStudioComponent {
  readonly tempMkelvin = signal<bigint>(298_150n);
  readonly selectedCellId = signal<string>("V850_F400");

  readonly vm = computed(() =>
    buildShmooPlotStudioVm(this.tempMkelvin(), this.selectedCellId()),
  );

  selectTemp(tMk: bigint): void {
    this.tempMkelvin.set(tMk);
  }

  selectCorner(cellId: string): void {
    this.selectedCellId.set(cellId);
  }
}
