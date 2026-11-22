// Module: web/libs/harness-3d/src/lib/multiboard-flex-nailboard/multiboard-flex-nailboard.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import {
  SigDataGridComponent,
  SigPanelComponent,
  SigSiInputComponent,
  SigStatusPillComponent,
} from "@signal/ui-kit";
import { buildMultiboardFlexNailboardVm } from "./multiboard-flex-nailboard.logic";

@Component({
  selector: "sig-multiboard-flex-nailboard",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigSiInputComponent, SigDataGridComponent],
  templateUrl: "./multiboard-flex-nailboard.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MultiboardFlexNailboardComponent {
  readonly foldProgressPct = signal<number>(100);
  readonly bendRadiusNm = signal<bigint>(2_500_000n);

  readonly vm = computed(() =>
    buildMultiboardFlexNailboardVm(this.foldProgressPct(), this.bendRadiusNm()),
  );

  selectFold(pct: number): void {
    this.foldProgressPct.set(pct);
  }

  onRadiusCommitted(radiusNm: bigint): void {
    this.bendRadiusNm.set(radiusNm);
  }
}
