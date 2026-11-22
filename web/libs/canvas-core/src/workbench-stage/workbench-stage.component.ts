// Module: web/libs/canvas-core/src/workbench-stage/workbench-stage.component.ts — Standardized Lazy Workbench Host
// Invariants: <=120 LOC; Zoneless OnPush; binds route-data descriptor via input(); zero local CSS.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { DualPlaneViewportComponent } from "../viewport/dual-plane-viewport.component";
import { buildWorkbenchStageVm } from "./workbench-stage.logic";
import type { WorkbenchDescriptor } from "./workbench-stage.model";

@Component({
  selector: "sig-workbench-stage",
  standalone: true,
  imports: [SigPanelComponent, SigDataGridComponent, SigStatusPillComponent, DualPlaneViewportComponent],
  templateUrl: "./workbench-stage.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class WorkbenchStageComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly vm = computed(() => buildWorkbenchStageVm(this.workbench()));
}
