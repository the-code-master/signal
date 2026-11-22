// Module: web/libs/ui-kit/src/panel/sig-panel.component.ts — Standardized <sig-panel> Container
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input } from "@angular/core";
import { buildSigPanelVm } from "./sig-panel.logic";

@Component({
  selector: "sig-panel",
  standalone: true,
  templateUrl: "./sig-panel.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SigPanelComponent {
  readonly title = input.required<string>();
  readonly subtitle = input<string>("");
  readonly badge = input<string>("");

  readonly vm = computed(() => buildSigPanelVm(this.title(), this.subtitle(), this.badge()));
}
