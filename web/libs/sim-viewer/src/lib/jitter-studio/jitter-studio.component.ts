// Module: web/libs/sim-viewer/src/lib/jitter-studio/jitter-studio.component.ts — FEAT-084 SerDes Jitter Studio Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildJitterStudioVm, presetJitterInput } from "./jitter-studio.logic";
import type { SerDesRatePreset } from "./jitter-studio.model";

@Component({
  selector: "sig-jitter-studio",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./jitter-studio.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class JitterStudioComponent {
  readonly presets: readonly SerDesRatePreset[] = ["112G_PAM4", "224G_PAM4"];
  readonly activePreset = signal<SerDesRatePreset>("224G_PAM4");

  readonly vm = computed(() => buildJitterStudioVm(presetJitterInput(this.activePreset())));

  selectPreset(preset: SerDesRatePreset): void {
    this.activePreset.set(preset);
  }
}
