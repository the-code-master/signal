// Module: web/apps/signal-web/src/app/omni-palette/omni-palette.component.ts — FEAT-091 Cmd+K Omni-Palette Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, output, signal } from "@angular/core";
import { SigStatusPillComponent } from "@signal/ui-kit";
import type { WorkbenchNavLink } from "../shell/ide-shell.model";
import { buildOmniPaletteVm } from "./omni-palette.logic";

@Component({
  selector: "sig-omni-palette",
  standalone: true,
  imports: [SigStatusPillComponent],
  templateUrl: "./omni-palette.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class OmniPaletteComponent {
  readonly open = input.required<boolean>();
  readonly navLinks = input.required<readonly WorkbenchNavLink[]>();

  readonly commandExecuted = output<string>();
  readonly workbenchSelected = output<string>();
  readonly closed = output<void>();

  readonly rawInput = signal<string>("place R 10k at 1250000,2500000");

  readonly vm = computed(() => buildOmniPaletteVm(this.open(), this.rawInput(), this.navLinks()));

  onInput(event: Event): void {
    this.rawInput.set((event.target as HTMLInputElement).value);
  }

  runCurrentCommand(): void {
    const p = this.vm().parsed;
    if (p.valid) {
      this.commandExecuted.emit(p.normalizedAction);
      if (p.targetWorkbenchSlug) {
        this.workbenchSelected.emit(p.targetWorkbenchSlug);
      }
      this.closed.emit();
    } else if (this.vm().jumpCandidates.length > 0) {
      this.workbenchSelected.emit(this.vm().jumpCandidates[0].slug);
      this.closed.emit();
    }
  }

  selectExample(ex: string): void {
    this.rawInput.set(ex);
  }

  jumpTo(slug: string): void {
    this.workbenchSelected.emit(slug);
    this.closed.emit();
  }
}
