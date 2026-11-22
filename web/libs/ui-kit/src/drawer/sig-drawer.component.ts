// Module: web/libs/ui-kit/src/drawer/sig-drawer.component.ts — Standardized <sig-drawer> Slide-Over Sheet
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, output } from "@angular/core";
import { buildSigDrawerVm } from "./sig-drawer.logic";

@Component({
  selector: "sig-drawer",
  standalone: true,
  templateUrl: "./sig-drawer.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SigDrawerComponent {
  readonly open = input.required<boolean>();
  readonly title = input.required<string>();
  readonly closed = output<void>();

  readonly vm = computed(() => buildSigDrawerVm(this.open(), this.title()));
}
