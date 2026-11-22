// Module: web/libs/code-ide/src/lib/memmap-pinmux-energy/memmap-pinmux-energy.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildMemmapPinmuxEnergyVm } from "./memmap-pinmux-energy.logic";

@Component({
  selector: "sig-memmap-pinmux-energy",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./memmap-pinmux-energy.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MemmapPinmuxEnergyComponent {
  readonly satPinmuxSolved = signal<boolean>(true);
  readonly deepSleepEnabled = signal<boolean>(true);

  readonly vm = computed(() =>
    buildMemmapPinmuxEnergyVm(this.satPinmuxSolved(), this.deepSleepEnabled()),
  );

  toggleSatPinmux(): void {
    this.satPinmuxSolved.update((v) => !v);
  }

  toggleDeepSleep(): void {
    this.deepSleepEnabled.update((v) => !v);
  }
}
