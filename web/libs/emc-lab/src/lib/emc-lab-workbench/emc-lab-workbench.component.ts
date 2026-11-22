// Module: web/libs/emc-lab/src/lib/emc-lab-workbench/emc-lab-workbench.component.ts — EMC Lab Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { EmiEsdShieldComponent } from "../emi-esd-shield/emi-esd-shield.component";
import { buildEmcLabWorkbenchVm } from "./emc-lab-workbench.logic";
import type { EmcStandardMask } from "./emc-lab-workbench.model";

@Component({
  selector: "sig-emc-lab-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigDataGridComponent,
    DualPlaneViewportComponent,
    EmiEsdShieldComponent,
  ],
  templateUrl: "./emc-lab-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class EmcLabWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly activeStandard = signal<EmcStandardMask>("CISPR_25_CLASS_5");
  readonly ultrasonicClampEnabled = signal<boolean>(true);

  readonly vm = computed(() =>
    buildEmcLabWorkbenchVm(
      this.workbench(),
      this.activeStandard(),
      this.ultrasonicClampEnabled() ? 32_000n : 8_000n,
    ),
  );

  selectStandard(std: EmcStandardMask): void {
    this.activeStandard.set(std);
  }

  toggleUltrasonicBurstClamp(): void {
    this.ultrasonicClampEnabled.update((v) => !v);
  }
}
