// Module: web/libs/schematic/src/lib/beautify-bus/beautify-bus.component.ts — FEAT-001 & FEAT-002 Panel Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildBeautifyBusVm } from "./beautify-bus.logic";
import type { BusProtocolKind } from "./beautify-bus.model";

@Component({
  selector: "sig-beautify-bus",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./beautify-bus.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BeautifyBusComponent {
  readonly protocols: readonly BusProtocolKind[] = ["RAW_VECTOR", "AXI4_LITE", "CAN_FD_DIFF", "SPI_QUAD"];

  readonly applied = signal<boolean>(false);
  readonly sliceExpression = signal<string>("DATA[31:0]");
  readonly activeProtocol = signal<BusProtocolKind>("RAW_VECTOR");
  readonly beautifyApplied = output<boolean>();

  readonly vm = computed(() =>
    buildBeautifyBusVm(
      {
        sheetId: "sheet-01-inverter",
        gridPitchNm: 254_000n,
        crossingCountBefore: 18,
        crossingCountAfter: 2,
        wirelengthSavedNm: 19_200_000n,
        applied: this.applied(),
      },
      this.sliceExpression(),
      this.activeProtocol(),
    ),
  );

  toggleBeautifyApply(): void {
    const next = !this.applied();
    this.applied.set(next);
    this.beautifyApplied.emit(next);
  }

  onSliceInput(event: Event): void {
    const target = event.target as HTMLInputElement;
    this.sliceExpression.set(target.value);
  }

  selectProtocol(protocol: BusProtocolKind): void {
    this.activeProtocol.set(protocol);
  }
}
