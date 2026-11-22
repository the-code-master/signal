// Module: web/libs/ic/src/lib/ic-workbench/ic-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { AnalogGmidLibertyComponent } from "../analog-gmid-liberty/analog-gmid-liberty.component";
import { SocStaUcieLithoComponent } from "../soc-sta-ucie-litho/soc-sta-ucie-litho.component";
import { UpfCpoCryoSecComponent } from "../upf-cpo-cryo-sec/upf-cpo-cryo-sec.component";
import { buildIcWorkbenchVm } from "./ic-workbench.logic";
import type { IcStudioTabId } from "./ic-workbench.model";

@Component({
  selector: "sig-ic-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    AnalogGmidLibertyComponent,
    SocStaUcieLithoComponent,
    UpfCpoCryoSecComponent,
  ],
  templateUrl: "./ic-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class IcWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly activeTab = signal<IcStudioTabId>("ANALOG_GMID_BSPDN");

  readonly vm = computed(() => buildIcWorkbenchVm(this.workbench(), this.activeTab()));

  selectTab(tab: IcStudioTabId): void {
    this.activeTab.set(tab);
  }
}
