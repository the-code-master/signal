// Module: web/libs/collab-hub/src/lib/project-status-tower/project-status-tower.component.ts — Status Control Tower UI
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS or browser storage.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import {
  SEEDED_DECISIONS,
  SEEDED_DEPENDENCIES,
  buildProjectStatusTowerVm,
} from "./project-status-tower.logic";
import type { CrossTeamDependencyItem, SignedDecisionEntry } from "./project-status-tower.model";

@Component({
  selector: "sig-project-status-tower",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./project-status-tower.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ProjectStatusTowerComponent {
  readonly nowUtcMs = signal<bigint>(1_795_324_800_000n);
  readonly dependencies = signal<readonly CrossTeamDependencyItem[]>(SEEDED_DEPENDENCIES);
  readonly decisions = signal<readonly SignedDecisionEntry[]>(SEEDED_DECISIONS);
  readonly fwOverrideRationale = signal<string | null>(null);

  readonly vm = computed(() =>
    buildProjectStatusTowerVm(
      this.dependencies(),
      this.decisions(),
      this.nowUtcMs(),
      this.fwOverrideRationale(),
    ),
  );

  simulateOverdueFwDependency(): void {
    this.dependencies.update((list) => [
      ...list,
      {
        dependencyId: `dep-late-${list.length + 1}`,
        producerTeam: "DESIGN",
        consumerTeam: "SOFTWARE_FIRMWARE",
        deliverableTitle: "Overdue ADC Calibration Pinout Table",
        dueAtUtcMs: 1_795_000_000_000n,
        handshakeStatus: "PROPOSED",
      },
    ]);
  }

  applySignedFwOverride(): void {
    this.fwOverrideRationale.set("Fallback mock calibration LUT active on HIL rig until rev E netlist lands.");
  }
}
