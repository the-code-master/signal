// Module: web/libs/harness-3d/src/lib/harness-3d-workbench/harness-3d-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { MultiboardFlexNailboardComponent } from "../multiboard-flex-nailboard/multiboard-flex-nailboard.component";
import { buildHarness3dWorkbenchVm } from "./harness-3d-workbench.logic";

@Component({
  selector: "sig-harness-3d-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    MultiboardFlexNailboardComponent,
  ],
  templateUrl: "./harness-3d-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class Harness3dWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();

  readonly vm = computed(() => buildHarness3dWorkbenchVm(this.workbench()));
}
