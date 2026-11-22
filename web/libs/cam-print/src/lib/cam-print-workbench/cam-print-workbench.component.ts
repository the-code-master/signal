// Module: web/libs/cam-print/src/lib/cam-print-workbench/cam-print-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { CncToolpathStudioComponent } from "../cnc-toolpath-studio/cnc-toolpath-studio.component";
import { DirectBrepSlicerComponent } from "../direct-brep-slicer/direct-brep-slicer.component";
import { buildCamPrintWorkbenchVm } from "./cam-print-workbench.logic";
import type { CamPrintTabId } from "./cam-print-workbench.model";

@Component({
  selector: "sig-cam-print-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    CncToolpathStudioComponent,
    DirectBrepSlicerComponent,
  ],
  templateUrl: "./cam-print-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CamPrintWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly activeTab = signal<CamPrintTabId>("SUBTRACTIVE_CNC_CAM");

  readonly vm = computed(() =>
    buildCamPrintWorkbenchVm(this.workbench(), this.activeTab()),
  );

  selectTab(tab: CamPrintTabId): void {
    this.activeTab.set(tab);
  }
}
