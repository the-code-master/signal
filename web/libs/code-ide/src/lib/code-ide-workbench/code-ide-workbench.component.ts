// Module: web/libs/code-ide/src/lib/code-ide-workbench/code-ide-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { MemmapPinmuxEnergyComponent } from "../memmap-pinmux-energy/memmap-pinmux-energy.component";
import { buildCodeIdeWorkbenchVm } from "./code-ide-workbench.logic";
import type { ActiveIdeLanguage } from "./code-ide-workbench.model";

@Component({
  selector: "sig-code-ide-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigDataGridComponent,
    MemmapPinmuxEnergyComponent,
  ],
  templateUrl: "./code-ide-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CodeIdeWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly language = signal<ActiveIdeLanguage>("RUST_NO_STD");
  readonly gliftZeroizerApplied = signal<boolean>(true);

  readonly vm = computed(() =>
    buildCodeIdeWorkbenchVm(
      this.workbench(),
      this.language(),
      this.gliftZeroizerApplied(),
    ),
  );

  selectLanguage(lang: ActiveIdeLanguage): void {
    this.language.set(lang);
  }

  toggleGliftZeroizer(): void {
    this.gliftZeroizerApplied.update((v) => !v);
  }
}
