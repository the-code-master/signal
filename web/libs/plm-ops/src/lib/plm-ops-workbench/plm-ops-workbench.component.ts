// Module: web/libs/plm-ops/src/lib/plm-ops-workbench/plm-ops-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { ChangeDeviationCtbTowerComponent } from "../change-deviation-ctb-tower/change-deviation-ctb-tower.component";
import { MultiBomPassportComponent } from "../multi-bom-passport/multi-bom-passport.component";
import { buildPlmOpsWorkbenchVm } from "./plm-ops-workbench.logic";
import type { ErpConnectorPreset } from "./plm-ops-workbench.model";

@Component({
  selector: "sig-plm-ops-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    MultiBomPassportComponent,
    ChangeDeviationCtbTowerComponent,
  ],
  templateUrl: "./plm-ops-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PlmOpsWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly erpTarget = signal<ErpConnectorPreset>("SAP_S4HANA");

  readonly vm = computed(() =>
    buildPlmOpsWorkbenchVm(this.workbench(), this.erpTarget()),
  );

  selectErp(target: ErpConnectorPreset): void {
    this.erpTarget.set(target);
  }
}
