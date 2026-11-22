// Module: web/libs/schematic/src/lib/live-sim-hud/live-sim-hud.component.ts — FEAT-003 & FEAT-004 Interactive HUD Controls
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildLiveSimHudVm } from "./live-sim-hud.logic";
import type { SchematicFaultMode } from "./live-sim-hud.model";

@Component({
  selector: "sig-live-sim-hud",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./live-sim-hud.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class LiveSimHudComponent {
  readonly faultModes: readonly SchematicFaultMode[] = [
    "NONE",
    "OPEN_CIRCUIT",
    "SHORT_CIRCUIT",
    "PARAMETRIC_DRIFT",
    "STUCK_AT_LOGIC",
  ];

  readonly wiperPercent = signal<number>(50);
  readonly switchClosed = signal<boolean>(true);
  readonly faultMode = signal<SchematicFaultMode>("NONE");
  readonly currentRiverEnabled = signal<boolean>(true);
  readonly soaThermalEnabled = signal<boolean>(true);
  readonly wiperChanged = output<bigint>();

  readonly vm = computed(() =>
    buildLiveSimHudVm({
      potTotalMohm: 10_000_000n,
      wiperPercent: this.wiperPercent(),
      switchClosed: this.switchClosed(),
      faultMode: this.faultMode(),
      currentRiverEnabled: this.currentRiverEnabled(),
      soaThermalEnabled: this.soaThermalEnabled(),
      rmsCurrentNa: this.switchClosed() ? 14_200_000_000n : 250_000_000n,
      junctionTempMkelvin: this.faultMode() === "NONE" ? 385_150n : 455_150n,
      maxJunctionTempMkelvin: 448_150n,
    }),
  );

  onWiperInput(event: Event): void {
    const target = event.target as HTMLInputElement;
    this.wiperPercent.set(Number(target.value));
    this.wiperChanged.emit(this.vm().lowerLegMohm);
  }

  toggleSwitch(): void {
    this.switchClosed.update((v) => !v);
  }

  selectFault(mode: SchematicFaultMode): void {
    this.faultMode.set(mode);
  }

  toggleCurrentRiver(): void {
    this.currentRiverEnabled.update((v) => !v);
  }

  toggleSoaBadge(): void {
    this.soaThermalEnabled.update((v) => !v);
  }
}
