// Module: web/libs/visual-diff/src/lib/visual-diff-workbench/visual-diff-workbench.component.ts — Visual Diff Host
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; wires FEAT-097 & Onion-Skin Diff.
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent } from "@signal/ui-kit";
import { HistoryTimeScrubberComponent } from "../history-time-scrubber/history-time-scrubber.component";
import { buildVisualDiffWorkbenchVm } from "./visual-diff-workbench.logic";
import type { DiffDomainFilter } from "./visual-diff-workbench.model";

@Component({
  selector: "sig-visual-diff-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigDataGridComponent,
    DualPlaneViewportComponent,
    HistoryTimeScrubberComponent,
  ],
  templateUrl: "./visual-diff-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class VisualDiffWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly activeDomain = signal<DiffDomainFilter>("ALL");
  readonly activeRevision = signal<bigint>(101n);

  readonly vm = computed(() =>
    buildVisualDiffWorkbenchVm(this.workbench(), this.activeDomain(), this.activeRevision()),
  );

  selectDomain(domain: DiffDomainFilter): void {
    this.activeDomain.set(domain);
  }

  onRevisionSelected(rev: bigint): void {
    this.activeRevision.set(rev);
  }
}
