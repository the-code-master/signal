// Module: web/libs/visual-diff/src/lib/history-time-scrubber/history-time-scrubber.component.ts — FEAT-097 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildHistoryTimeScrubberVm } from "./history-time-scrubber.logic";

@Component({
  selector: "sig-history-time-scrubber",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./history-time-scrubber.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class HistoryTimeScrubberComponent {
  readonly checkpointIdx = signal<number>(2);
  readonly onionOpacity = signal<number>(50);
  readonly revisionSelected = output<bigint>();

  readonly vm = computed(() =>
    buildHistoryTimeScrubberVm(this.checkpointIdx(), this.onionOpacity()),
  );

  onScrubIdx(event: Event): void {
    this.checkpointIdx.set(Number((event.target as HTMLInputElement).value));
    this.revisionSelected.emit(this.vm().selectedCheckpoint.revision);
  }

  onOpacityInput(event: Event): void {
    this.onionOpacity.set(Number((event.target as HTMLInputElement).value));
  }
}
