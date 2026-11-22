// Module: web/libs/sim-viewer/src/lib/virtual-bench/virtual-bench.component.ts — FEAT-081 Virtual Bench Deck Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildVirtualBenchVm } from "./virtual-bench.logic";
import type { BenchInstrumentMode } from "./virtual-bench.model";

@Component({
  selector: "sig-virtual-bench-deck",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./virtual-bench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class VirtualBenchDeckComponent {
  readonly activeMode = signal<BenchInstrumentMode>("FFT_SPECTRUM");
  readonly rbwHz = signal<bigint>(10_000n);
  readonly triggerUv = signal<bigint>(400_000_000n);

  readonly vm = computed(() =>
    buildVirtualBenchVm(this.activeMode(), this.rbwHz(), this.triggerUv()),
  );

  selectMode(mode: BenchInstrumentMode): void {
    this.activeMode.set(mode);
  }
}
