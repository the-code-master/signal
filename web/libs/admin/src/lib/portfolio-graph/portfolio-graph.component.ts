// Module: web/libs/admin/src/lib/portfolio-graph/portfolio-graph.component.ts — Portfolio Blast-Radius UI (FEAT-118)
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS or browser storage.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildPortfolioGraphVm } from "./portfolio-graph.logic";
import type { BlastTriggerArtifact } from "./portfolio-graph.model";

@Component({
  selector: "sig-portfolio-graph",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./portfolio-graph.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PortfolioGraphComponent {
  readonly selectedArtifact = signal<BlastTriggerArtifact>("C3M0016120K_SIC_EOL");

  readonly vm = computed(() => buildPortfolioGraphVm(this.selectedArtifact()));

  selectArtifact(artifact: BlastTriggerArtifact): void {
    this.selectedArtifact.set(artifact);
  }
}
