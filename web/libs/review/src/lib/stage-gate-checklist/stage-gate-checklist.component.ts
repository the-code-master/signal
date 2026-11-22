// Module: web/libs/review/src/lib/stage-gate-checklist/stage-gate-checklist.component.ts — FEAT-098 Stage-Gate Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildStageGateChecklistVm } from "./stage-gate-checklist.logic";
import type { HardwareStageGate } from "./stage-gate-checklist.model";

@Component({
  selector: "sig-stage-gate-checklist",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./stage-gate-checklist.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class StageGateChecklistComponent {
  readonly activeGate = signal<HardwareStageGate>("DVT");
  readonly signedGates = signal<ReadonlySet<HardwareStageGate>>(new Set());

  readonly vm = computed(() =>
    buildStageGateChecklistVm(this.activeGate(), this.signedGates()),
  );

  selectGate(gate: HardwareStageGate): void {
    this.activeGate.set(gate);
  }

  signActiveGate(): void {
    if (!this.vm().readyToSignOff) return;
    const next = new Set(this.signedGates());
    next.add(this.activeGate());
    this.signedGates.set(next);
  }
}
