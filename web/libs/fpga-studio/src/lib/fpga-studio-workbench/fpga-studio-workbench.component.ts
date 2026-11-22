// Module: web/libs/fpga-studio/src/lib/fpga-studio-workbench/fpga-studio-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { DieIoSdcPlannerComponent } from "../die-io-sdc-planner/die-io-sdc-planner.component";
import { FsmDspStudioComponent } from "../fsm-dsp-studio/fsm-dsp-studio.component";
import { buildFpgaStudioWorkbenchVm } from "./fpga-studio-workbench.logic";
import type { FpgaPartPreset } from "./fpga-studio-workbench.model";

@Component({
  selector: "sig-fpga-studio-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    DieIoSdcPlannerComponent,
    FsmDspStudioComponent,
  ],
  templateUrl: "./fpga-studio-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class FpgaStudioWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly partPreset = signal<FpgaPartPreset>("XCKU5P_ULTRASCALE_PLUS");

  readonly vm = computed(() =>
    buildFpgaStudioWorkbenchVm(this.workbench(), this.partPreset()),
  );

  selectPart(part: FpgaPartPreset): void {
    this.partPreset.set(part);
  }
}
