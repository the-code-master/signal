// Module: web/libs/academy/src/lib/open-silicon-wizard/open-silicon-wizard.component.ts — Open-Silicon Wizard UI (FEAT-115)
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; emits shuttle digest when GDSII/OASIS precheck passes.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildOpenSiliconWizardVm, slotMaxBoundsNm } from "./open-silicon-wizard.logic";
import type { OpenSiliconPdkId, ShuttleTileSlot } from "./open-silicon-wizard.model";

@Component({
  selector: "sig-open-silicon-wizard",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./open-silicon-wizard.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class OpenSiliconWizardComponent {
  readonly shuttleValidated = output<string>();

  readonly pdk = signal<OpenSiliconPdkId>("SKYWATER_SKY130");
  readonly tileSlot = signal<ShuttleTileSlot>("TINYTAPEOUT_1X1");
  readonly dieWidthNm = signal<bigint>(155_000_000n);
  readonly dieHeightNm = signal<bigint>(96_000_000n);
  readonly metalFillInserted = signal<boolean>(true);

  readonly vm = computed(() =>
    buildOpenSiliconWizardVm(
      this.pdk(),
      this.tileSlot(),
      this.dieWidthNm(),
      this.dieHeightNm(),
      this.metalFillInserted(),
    ),
  );

  selectPdk(nextPdk: OpenSiliconPdkId): void {
    this.pdk.set(nextPdk);
  }

  selectSlot(slot: ShuttleTileSlot): void {
    const [maxW, maxH] = slotMaxBoundsNm(slot);
    this.tileSlot.set(slot);
    this.dieWidthNm.set(maxW - 5_000_000n);
    this.dieHeightNm.set(maxH - 4_000_000n);
  }

  toggleMetalFill(): void {
    this.metalFillInserted.update((v) => !v);
  }

  stageMpwShuttleBundle(): void {
    this.shuttleValidated.emit(this.vm().report.shuttleSubmissionDigest);
  }
}
