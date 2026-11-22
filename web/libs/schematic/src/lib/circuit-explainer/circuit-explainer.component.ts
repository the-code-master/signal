// Module: web/libs/schematic/src/lib/circuit-explainer/circuit-explainer.component.ts — FEAT-101 Explainer Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildCircuitExplainerVm } from "./circuit-explainer.logic";

@Component({
  selector: "sig-circuit-explainer",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./circuit-explainer.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CircuitExplainerComponent {
  readonly selectedStageId = signal<string>("STG-EMI");
  readonly vm = computed(() => buildCircuitExplainerVm(this.selectedStageId()));

  selectStage(stageId: string): void {
    this.selectedStageId.set(stageId);
  }
}
