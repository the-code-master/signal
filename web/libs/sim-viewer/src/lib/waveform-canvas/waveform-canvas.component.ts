// Module: web/libs/sim-viewer/src/lib/waveform-canvas/waveform-canvas.component.ts — Arrow-Streamed Waveform Canvas
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; emits schematic net cross-probe.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildWaveformCanvasVm, createDefaultBuckWaveformBytes } from "./waveform-canvas.logic";

@Component({
  selector: "sig-waveform-canvas",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./waveform-canvas.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class WaveformCanvasComponent {
  private readonly defaultFrame = createDefaultBuckWaveformBytes();

  readonly cursorAIdx = signal<number>(8);
  readonly cursorBIdx = signal<number>(24);
  readonly activeProbeNet = signal<string>("HV_SW_PHASE_U");
  readonly netCrossProbed = output<string>();

  readonly vm = computed(() =>
    buildWaveformCanvasVm(
      this.defaultFrame,
      this.cursorAIdx(),
      this.cursorBIdx(),
      this.activeProbeNet(),
    ),
  );

  onCursorAInput(event: Event): void {
    this.cursorAIdx.set(Number((event.target as HTMLInputElement).value));
  }

  onCursorBInput(event: Event): void {
    this.cursorBIdx.set(Number((event.target as HTMLInputElement).value));
  }

  crossProbeNet(netName: string): void {
    this.activeProbeNet.set(netName);
    this.netCrossProbed.emit(netName);
  }
}
