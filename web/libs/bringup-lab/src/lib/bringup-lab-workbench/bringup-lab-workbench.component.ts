// Module: web/libs/bringup-lab/src/lib/bringup-lab-workbench/bringup-lab-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { BootHilBoardmapComponent } from "../boot-hil-boardmap/boot-hil-boardmap.component";
import { ShmooPlotStudioComponent } from "../shmoo-plot-studio/shmoo-plot-studio.component";
import { buildBringupLabWorkbenchVm } from "./bringup-lab-workbench.logic";
import type { LabCaptureSource } from "./bringup-lab-workbench.model";

@Component({
  selector: "sig-bringup-lab-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    BootHilBoardmapComponent,
    ShmooPlotStudioComponent,
  ],
  templateUrl: "./bringup-lab-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BringupLabWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly captureSource = signal<LabCaptureSource>("SCPI_MSO64B_SCOPE");
  readonly bayesianCalibrated = signal<boolean>(true);

  readonly vm = computed(() =>
    buildBringupLabWorkbenchVm(
      this.workbench(),
      this.captureSource(),
      this.bayesianCalibrated(),
    ),
  );

  selectCapture(src: LabCaptureSource): void {
    this.captureSource.set(src);
  }

  toggleCalibration(): void {
    this.bayesianCalibrated.update((v) => !v);
  }
}
