// Module: web/libs/library-studio/src/lib/pin-symbol-splitter/pin-symbol-splitter.component.ts — FEAT-072 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildPinSymbolSplitterVm } from "./pin-symbol-splitter.logic";
import type { SchematicSymbolStandard } from "./pin-symbol-splitter.model";

@Component({
  selector: "sig-pin-symbol-splitter",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./pin-symbol-splitter.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PinSymbolSplitterComponent {
  readonly standards: readonly SchematicSymbolStandard[] = ["IEEE_315", "IEC_60617"];
  readonly bankCaps: readonly number[] = [1, 2, 4];

  readonly activeStandard = signal<SchematicSymbolStandard>("IEEE_315");
  readonly maxPinsPerBank = signal<number>(2);

  readonly vm = computed(() =>
    buildPinSymbolSplitterVm(this.activeStandard(), this.maxPinsPerBank()),
  );

  selectStandard(std: SchematicSymbolStandard): void {
    this.activeStandard.set(std);
  }

  selectBankCap(cap: number): void {
    this.maxPinsPerBank.set(cap);
  }
}
