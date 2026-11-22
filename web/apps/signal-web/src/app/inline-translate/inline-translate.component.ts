// Module: web/apps/signal-web/src/app/inline-translate/inline-translate.component.ts — FEAT-110 Translator Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildInlineTranslateVm } from "./inline-translate.logic";
import type { TechnicalLocaleCode } from "./inline-translate.model";

@Component({
  selector: "sig-inline-translate",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./inline-translate.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class InlineTranslateComponent {
  readonly sourceText = signal<string>(
    "Place R14 10kΩ near U2 on HV_DC_BUS_800V with 8000000nm clearance",
  );
  readonly activeLocale = signal<TechnicalLocaleCode>("ja");

  readonly vm = computed(() =>
    buildInlineTranslateVm(this.sourceText(), this.activeLocale()),
  );

  onSourceInput(event: Event): void {
    this.sourceText.set((event.target as HTMLInputElement).value);
  }

  selectLocale(locale: TechnicalLocaleCode): void {
    this.activeLocale.set(locale);
  }
}
