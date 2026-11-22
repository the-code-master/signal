// Module: web/libs/sim-viewer/src/lib/breaking-point/breaking-point.component.ts — FEAT-102 Breaking-Point Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildBreakingPointVm } from "./breaking-point.logic";

@Component({
  selector: "sig-breaking-point",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./breaking-point.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BreakingPointComponent {
  readonly surgeVolts = signal<number>(800);
  readonly ambientCelsius = signal<number>(85);

  readonly vm = computed(() =>
    buildBreakingPointVm(
      273_150n + BigInt(this.ambientCelsius()) * 1_000n,
      BigInt(this.surgeVolts()) * 1_000_000n,
    ),
  );

  onSurgeInput(event: Event): void {
    this.surgeVolts.set(Number((event.target as HTMLInputElement).value));
  }

  onAmbientInput(event: Event): void {
    this.ambientCelsius.set(Number((event.target as HTMLInputElement).value));
  }
}
