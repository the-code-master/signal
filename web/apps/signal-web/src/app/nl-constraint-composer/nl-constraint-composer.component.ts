// Module: web/apps/signal-web/src/app/nl-constraint-composer/nl-constraint-composer.component.ts — FEAT-105 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildNlConstraintComposerVm } from "./nl-constraint-composer.logic";
import type { ConstraintTargetLanguage } from "./nl-constraint-composer.model";

@Component({
  selector: "sig-nl-constraint-composer",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./nl-constraint-composer.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class NlConstraintComposerComponent {
  readonly promptText = signal<string>("Route PCIe6 differential lanes at 85 ohm with <= 500 fs skew");
  readonly targetLang = signal<ConstraintTargetLanguage>("DRC");
  readonly ruleCommitted = output<string>();

  readonly vm = computed(() =>
    buildNlConstraintComposerVm(this.promptText(), this.targetLang()),
  );

  onPromptInput(event: Event): void {
    this.promptText.set((event.target as HTMLInputElement).value);
  }

  selectLanguage(lang: ConstraintTargetLanguage): void {
    this.targetLang.set(lang);
  }

  commitRule(): void {
    if (this.vm().compiled.valid) {
      this.ruleCommitted.emit(this.vm().compiled.compiledRuleDsl);
    }
  }
}
