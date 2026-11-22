// Module: web/libs/ui-kit/src/si-input/sig-si-input.component.ts — Standardized <sig-si-input> Control
// Invariants: <=120 LOC; Zoneless OnPush; emits exact bigint nanoUnits; zero local CSS.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, output, signal } from "@angular/core";
import { parseSiInputToNano } from "./sig-si-input.logic";

@Component({
  selector: "sig-si-input",
  standalone: true,
  templateUrl: "./sig-si-input.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SigSiInputComponent {
  readonly label = input.required<string>();
  readonly unitSymbol = input<string>("m");
  readonly initialText = input<string>("85u");
  readonly valueCommitted = output<bigint>();

  readonly rawValue = signal<string>("");
  readonly effectiveText = computed(() => this.rawValue() || this.initialText());
  readonly parsed = computed(() => parseSiInputToNano(this.effectiveText(), this.unitSymbol()));

  onInput(event: Event): void {
    const target = event.target as HTMLInputElement;
    this.rawValue.set(target.value);
    const res = parseSiInputToNano(target.value, this.unitSymbol());
    if (res.valid) {
      this.valueCommitted.emit(res.nanoUnits);
    }
  }
}
