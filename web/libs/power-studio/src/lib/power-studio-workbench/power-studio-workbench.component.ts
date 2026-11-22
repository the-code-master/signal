// Module: web/libs/power-studio/src/lib/power-studio-workbench/power-studio-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { BodeBmsMotorComponent } from "../bode-bms-motor/bode-bms-motor.component";
import { PdnCreepageIrHotloopComponent } from "../pdn-creepage-ir-hotloop/pdn-creepage-ir-hotloop.component";
import { ThermalSequencingComponent } from "../thermal-sequencing/thermal-sequencing.component";
import { buildPowerStudioWorkbenchVm } from "./power-studio-workbench.logic";
import type { PowerStudioTabId } from "./power-studio-workbench.model";

@Component({
  selector: "sig-power-studio-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    PdnCreepageIrHotloopComponent,
    BodeBmsMotorComponent,
    ThermalSequencingComponent,
  ],
  templateUrl: "./power-studio-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PowerStudioWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly activeTab = signal<PowerStudioTabId>("PDN_CREEPAGE_HOTLOOP");

  readonly vm = computed(() =>
    buildPowerStudioWorkbenchVm(this.workbench(), this.activeTab()),
  );

  selectTab(tab: PowerStudioTabId): void {
    this.activeTab.set(tab);
  }
}
