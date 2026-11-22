// Module: web/libs/pcb/src/lib/pns-routing-meanders/pns-routing-meanders.component.ts — FEAT-012/015/017 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import {
  SigDataGridComponent,
  SigPanelComponent,
  SigSiInputComponent,
  SigStatusPillComponent,
} from "@signal/ui-kit";
import { buildPnsRoutingMeandersVm } from "./pns-routing-meanders.logic";
import type { PnsRouterMode } from "./pns-routing-meanders.model";

@Component({
  selector: "sig-pns-routing-meanders",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigSiInputComponent, SigDataGridComponent],
  templateUrl: "./pns-routing-meanders.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PnsRoutingMeandersComponent {
  readonly activeMode = signal<PnsRouterMode>("BUS_TRUNK_MULTI_LANE");
  readonly tromboneAmpNm = signal<bigint>(250_000n);

  readonly vm = computed(() =>
    buildPnsRoutingMeandersVm(this.activeMode(), this.tromboneAmpNm()),
  );

  selectMode(mode: PnsRouterMode): void {
    this.activeMode.set(mode);
  }

  onAmplitudeCommitted(valNm: bigint): void {
    this.tromboneAmpNm.set(valNm);
  }
}
