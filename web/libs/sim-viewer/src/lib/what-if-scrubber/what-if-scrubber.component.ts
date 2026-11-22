// Module: web/libs/sim-viewer/src/lib/what-if-scrubber/what-if-scrubber.component.ts — FEAT-085 What-If Scrubber Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildWhatIfScrubberVm } from "./what-if-scrubber.logic";

@Component({
  selector: "sig-what-if-scrubber",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./what-if-scrubber.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class WhatIfScrubberComponent {
  readonly scrubPercent = signal<number>(0);
  readonly vm = computed(() => buildWhatIfScrubberVm(4_700n, this.scrubPercent()));

  onScrubInput(event: Event): void {
    this.scrubPercent.set(Number((event.target as HTMLInputElement).value));
  }
}
