// Module: web/libs/ai-copilot/src/lib/ai-copilot-workbench/ai-copilot-workbench.component.ts — CU-P3-03 Copilot Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildAiCopilotWorkbenchVm } from "./ai-copilot-workbench.logic";
import type { LegacyImportFormat, MainBranchDriftState } from "./ai-copilot-workbench.model";

@Component({
  selector: "sig-ai-copilot-workbench",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./ai-copilot-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AiCopilotWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly activeFormat = signal<LegacyImportFormat>("ALTIUM_DOC");
  readonly activeDrift = signal<MainBranchDriftState>("ADVANCED_CLEAN");
  readonly simulateBomb = signal<boolean>(false);

  readonly vm = computed(() =>
    buildAiCopilotWorkbenchVm(
      this.workbench(),
      this.activeFormat(),
      this.activeDrift(),
      this.simulateBomb(),
    ),
  );

  selectFormat(format: LegacyImportFormat): void {
    this.activeFormat.set(format);
  }

  selectDrift(drift: MainBranchDriftState): void {
    this.activeDrift.set(drift);
  }

  toggleBombSimulation(): void {
    this.simulateBomb.update((v) => !v);
  }
}
