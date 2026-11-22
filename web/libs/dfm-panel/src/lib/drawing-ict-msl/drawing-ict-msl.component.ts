// Module: web/libs/dfm-panel/src/lib/drawing-ict-msl/drawing-ict-msl.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildDrawingIctMslVm } from "./drawing-ict-msl.logic";
import type { Jstd020MslLevel } from "./drawing-ict-msl.model";

@Component({
  selector: "sig-drawing-ict-msl",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./drawing-ict-msl.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DrawingIctMslComponent {
  readonly mslLevel = signal<Jstd020MslLevel>("MSL_3_168H");
  readonly jtagEnabled = signal<boolean>(true);

  readonly vm = computed(() =>
    buildDrawingIctMslVm(this.mslLevel(), this.jtagEnabled()),
  );

  selectMsl(level: Jstd020MslLevel): void {
    this.mslLevel.set(level);
  }

  toggleJtag(): void {
    this.jtagEnabled.update((v) => !v);
  }
}
