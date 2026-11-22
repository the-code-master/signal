// Module: web/libs/academy/src/lib/quick-calc-hud/quick-calc-hud.component.ts — FEAT-114 Quick-Calc HUD Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; emits 1-click Insert into Schematic.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigPanelComponent, SigSiInputComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildQuickCalcHudVm } from "./quick-calc-hud.logic";
import type { SchematicInsertPayload } from "./quick-calc-hud.model";

@Component({
  selector: "sig-quick-calc-hud",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigSiInputComponent],
  templateUrl: "./quick-calc-hud.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class QuickCalcHudComponent {
  readonly bandsCsv = signal<string>("brown, black, black, red, brown, red");
  readonly smdCode = signal<string>("01C");
  readonly dividerVinUv = signal<bigint>(12_000_000n);
  readonly ledSupplyUv = signal<bigint>(5_000_000n);
  readonly insertedIntoSchematic = output<SchematicInsertPayload>();

  readonly vm = computed(() =>
    buildQuickCalcHudVm(
      this.bandsCsv(),
      this.smdCode(),
      this.dividerVinUv(),
      this.ledSupplyUv(),
    ),
  );

  onBandsInput(event: Event): void {
    this.bandsCsv.set((event.target as HTMLInputElement).value);
  }

  onSmdInput(event: Event): void {
    this.smdCode.set((event.target as HTMLInputElement).value);
  }

  onDividerVinCommitted(nanoVolts: bigint): void {
    this.dividerVinUv.set(nanoVolts / 1_000n);
  }

  insertSmdIntoSchematic(): void {
    const res = this.vm().smdResult;
    this.insertedIntoSchematic.emit({
      calculatorKind: "EIA96_SMD",
      primaryValueMohm: res.nominalMohm,
      description: res.formattedSummary,
    });
  }

  insertLedLimiterIntoSchematic(): void {
    const led = this.vm().ledResult;
    this.insertedIntoSchematic.emit({
      calculatorKind: "LED_RESISTOR",
      primaryValueMohm: led.nearestE24Mohm,
      description: led.formattedSummary,
    });
  }
}
