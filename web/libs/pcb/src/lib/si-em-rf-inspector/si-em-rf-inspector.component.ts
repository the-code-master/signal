// Module: web/libs/pcb/src/lib/si-em-rf-inspector/si-em-rf-inspector.component.ts — SI/EM/RF Inspector Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildSiEmRfInspectorVm } from "./si-em-rf-inspector.logic";
import type { RfTopologyPreset } from "./si-em-rf-inspector.model";

@Component({
  selector: "sig-si-em-rf-inspector",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./si-em-rf-inspector.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SiEmRfInspectorComponent {
  readonly weaveRotated = signal<boolean>(true);
  readonly backdrilled = signal<boolean>(true);
  readonly rfPreset = signal<RfTopologyPreset>("WILKINSON_2WAY_28GHZ");
  readonly emPhaseDeg = signal<number>(90);

  readonly vm = computed(() =>
    buildSiEmRfInspectorVm(
      this.weaveRotated(),
      this.backdrilled(),
      this.rfPreset(),
      this.emPhaseDeg(),
    ),
  );

  toggleWeaveRotation(): void {
    this.weaveRotated.update((v) => !v);
  }

  toggleBackdrill(): void {
    this.backdrilled.update((v) => !v);
  }

  selectRfPreset(preset: RfTopologyPreset): void {
    this.rfPreset.set(preset);
  }

  stepEmPhase(): void {
    this.emPhaseDeg.update((p) => (p + 45) % 360);
  }
}
