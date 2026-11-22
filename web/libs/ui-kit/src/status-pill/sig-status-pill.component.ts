// Module: web/libs/ui-kit/src/status-pill/sig-status-pill.component.ts — Standardized <sig-status-pill>
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input } from "@angular/core";
import { buildSigStatusPillVm } from "./sig-status-pill.logic";
import type { SigStatusTone } from "./sig-status-pill.model";

@Component({
  selector: "sig-status-pill",
  standalone: true,
  templateUrl: "./sig-status-pill.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SigStatusPillComponent {
  readonly label = input.required<string>();
  readonly tone = input<SigStatusTone>("neutral");

  readonly vm = computed(() => buildSigStatusPillVm(this.label(), this.tone()));
}
