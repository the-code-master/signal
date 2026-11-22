// Module: web/apps/signal-web/src/app/accessibility/accessibility.component.ts — FEAT-100 WCAG 2.2 AA CAD Stepper Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; exposes aria-live="polite" spatial netlist reader.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import {
  buildAccessibilityCadVm,
  createInitialAccessibleCursor,
  stepAccessibleCadCursor,
} from "./accessibility.logic";
import type { AccessibleCursorState, CadKeymapPreset } from "./accessibility.model";

@Component({
  selector: "sig-accessibility-cad",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./accessibility.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AccessibilityCadComponent {
  readonly cursorState = signal<AccessibleCursorState>(createInitialAccessibleCursor("KiCad"));
  readonly presetSelected = output<CadKeymapPreset>();

  readonly vm = computed(() => buildAccessibilityCadVm(this.cursorState()));

  selectPreset(preset: CadKeymapPreset): void {
    this.cursorState.update((cur) => ({ ...cur, preset }));
    this.presetSelected.emit(preset);
  }

  triggerKey(key: string): void {
    this.cursorState.update((cur) => stepAccessibleCadCursor(cur, key));
  }

  onSurfaceKeydown(event: KeyboardEvent): void {
    const next = stepAccessibleCadCursor(this.cursorState(), event.key);
    if (next !== this.cursorState()) {
      event.preventDefault();
      this.cursorState.set(next);
    }
  }
}
