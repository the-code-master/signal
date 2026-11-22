// Module: web/apps/signal-web/src/app/spacemouse/spacemouse.component.ts — FEAT-093 Radial Pie-Menu & SpaceMouse Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildSpacemouseVm } from "./spacemouse.logic";

@Component({
  selector: "sig-spacemouse-control",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./spacemouse.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SpacemouseControlComponent {
  readonly selectedSliceIdx = signal<number | null>(0);
  readonly puckPitch = signal<number>(0.45);
  readonly pieActionTriggered = output<string>();

  readonly vm = computed(() =>
    buildSpacemouseVm(this.selectedSliceIdx(), {
      tx: this.puckPitch(),
      ty: 0,
      tz: 0.3,
      rx: this.puckPitch(),
      ry: 0,
      rz: 0,
    }),
  );

  selectSlice(idx: number, cmd: string): void {
    this.selectedSliceIdx.set(idx);
    this.pieActionTriggered.emit(cmd);
  }

  onPuckSlider(event: Event): void {
    this.puckPitch.set(Number((event.target as HTMLInputElement).value) / 100);
  }
}
