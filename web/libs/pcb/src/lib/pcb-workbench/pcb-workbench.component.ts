// Module: web/libs/pcb/src/lib/pcb-workbench/pcb-workbench.component.ts — CU-P4-01 PCB Workbench Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { PnsRoutingMeandersComponent } from "../pns-routing-meanders/pns-routing-meanders.component";
import { SiEmRfInspectorComponent } from "../si-em-rf-inspector/si-em-rf-inspector.component";
import { StackupConstraintsXrayComponent } from "../stackup-constraints-xray/stackup-constraints-xray.component";
import { buildPcbWorkbenchVm } from "./pcb-workbench.logic";
import type { PcbStudioTabId } from "./pcb-workbench.model";

@Component({
  selector: "sig-pcb-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    PnsRoutingMeandersComponent,
    SiEmRfInspectorComponent,
    StackupConstraintsXrayComponent,
  ],
  templateUrl: "./pcb-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PcbWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly activeTab = signal<PcbStudioTabId>("PNS_MEANDERS");

  readonly vm = computed(() => buildPcbWorkbenchVm(this.workbench(), this.activeTab()));

  selectTab(tab: PcbStudioTabId): void {
    this.activeTab.set(tab);
  }
}
