// Module: web/libs/dfm-panel/src/lib/panel-dfa-fatigue-coating/panel-dfa-fatigue-coating.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildPanelDfaFatigueCoatingVm } from "./panel-dfa-fatigue-coating.logic";
import type { DepanelingMethod } from "./panel-dfa-fatigue-coating.model";

@Component({
  selector: "sig-panel-dfa-fatigue-coating",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./panel-dfa-fatigue-coating.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PanelDfaFatigueCoatingComponent {
  readonly depanelMethod = signal<DepanelingMethod>("ROUTER_MOUSE_BITE_TABS");

  readonly vm = computed(() => buildPanelDfaFatigueCoatingVm(this.depanelMethod()));

  selectMethod(method: DepanelingMethod): void {
    this.depanelMethod.set(method);
  }
}
