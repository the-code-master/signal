// Module: web/libs/pcb/src/lib/stackup-constraints-xray/stackup-constraints-xray.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildStackupConstraintsXrayVm } from "./stackup-constraints-xray.logic";
import type { LayerXrayPreset } from "./stackup-constraints-xray.model";

@Component({
  selector: "sig-stackup-constraints-xray",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./stackup-constraints-xray.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class StackupConstraintsXrayComponent {
  readonly activePreset = signal<LayerXrayPreset>("ALL_12_LAYERS");
  readonly teardropsApplied = signal<boolean>(true);

  readonly vm = computed(() =>
    buildStackupConstraintsXrayVm(this.activePreset(), this.teardropsApplied()),
  );

  selectPreset(preset: LayerXrayPreset): void {
    this.activePreset.set(preset);
  }

  toggleTeardrops(): void {
    this.teardropsApplied.update((v) => !v);
  }
}
