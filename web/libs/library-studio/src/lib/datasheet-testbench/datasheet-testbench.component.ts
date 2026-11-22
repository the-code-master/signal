// Module: web/libs/library-studio/src/lib/datasheet-testbench/datasheet-testbench.component.ts — FEAT-107 Wizard Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildDatasheetTestbenchVm } from "./datasheet-testbench.logic";

@Component({
  selector: "sig-datasheet-testbench",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./datasheet-testbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DatasheetTestbenchComponent {
  readonly selectedMpn = signal<string>("TPS546D24ARVFR");
  readonly testbenchGenerated = output<string>();

  readonly vm = computed(() => buildDatasheetTestbenchVm(this.selectedMpn()));

  selectPart(mpn: string): void {
    this.selectedMpn.set(mpn);
  }

  openGeneratedTestbench(): void {
    this.testbenchGenerated.emit(this.vm().selected.generatedBranchId);
  }
}
