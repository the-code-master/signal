// Module: web/libs/fpga-studio/src/lib/fsm-dsp-studio/fsm-dsp-studio.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildFsmDspStudioVm } from "./fsm-dsp-studio.logic";
import type { FsmEncodingStyle } from "./fsm-dsp-studio.model";

@Component({
  selector: "sig-fsm-dsp-studio",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./fsm-dsp-studio.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class FsmDspStudioComponent {
  readonly fsmEncoding = signal<FsmEncodingStyle>("HAMMING3_SEU_TOLERANT");
  readonly fracBitsN = signal<number>(14);

  readonly vm = computed(() =>
    buildFsmDspStudioVm(this.fsmEncoding(), this.fracBitsN()),
  );

  selectEncoding(enc: FsmEncodingStyle): void {
    this.fsmEncoding.set(enc);
  }

  selectBits(n: number): void {
    this.fracBitsN.set(n);
  }
}
