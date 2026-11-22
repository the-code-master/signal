// Module: web/libs/specs/src/lib/specs-bom-workbench/specs-bom-workbench.component.ts — CU-P2-04 Specs & BOM Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { BomConsolidationComponent } from "../bom-consolidation/bom-consolidation.component";
import { DatasheetDriftComponent } from "../datasheet-drift/datasheet-drift.component";
import { buildSpecsBomWorkbenchVm } from "./specs-bom-workbench.logic";

@Component({
  selector: "sig-specs-bom-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigDataGridComponent,
    DatasheetDriftComponent,
    BomConsolidationComponent,
  ],
  templateUrl: "./specs-bom-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SpecsBomWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly testBrokerSelection = signal<boolean>(false);

  readonly vm = computed(() =>
    buildSpecsBomWorkbenchVm(this.workbench(), this.testBrokerSelection()),
  );

  toggleSourcingMode(): void {
    this.testBrokerSelection.update((v) => !v);
  }
}
