// Module: web/libs/mcad-studio/src/lib/brep-sheetmetal-anchors/brep-sheetmetal-anchors.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildBrepSheetmetalAnchorsVm } from "./brep-sheetmetal-anchors.logic";

@Component({
  selector: "sig-brep-sheetmetal-anchors",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./brep-sheetmetal-anchors.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BrepSheetmetalAnchorsComponent {
  readonly kFactor = signal<number>(0.44);
  readonly sketchLocked = signal<boolean>(true);

  readonly vm = computed(() =>
    buildBrepSheetmetalAnchorsVm(this.kFactor(), this.sketchLocked()),
  );

  selectKFactor(k: number): void {
    this.kFactor.set(k);
  }

  toggleSketchLock(): void {
    this.sketchLocked.update((v) => !v);
  }
}
