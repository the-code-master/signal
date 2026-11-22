// Module: web/libs/sim-viewer/src/lib/pareto-frontier/pareto-frontier.component.ts — FEAT-103 Pareto Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildParetoFrontierVm } from "./pareto-frontier.logic";
import type { ParetoPriorityGoal } from "./pareto-frontier.model";

@Component({
  selector: "sig-pareto-frontier",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./pareto-frontier.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ParetoFrontierComponent {
  readonly activeGoal = signal<ParetoPriorityGoal>("MAX_EFFICIENCY");
  readonly cowPreviewRequested = output<string>();

  readonly vm = computed(() => buildParetoFrontierVm(this.activeGoal()));

  selectGoal(goal: ParetoPriorityGoal): void {
    this.activeGoal.set(goal);
  }

  previewRecommendedBranch(): void {
    this.cowPreviewRequested.emit(this.vm().recommended.branchId);
  }
}
