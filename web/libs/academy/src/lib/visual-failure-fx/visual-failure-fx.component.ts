// Module: web/libs/academy/src/lib/visual-failure-fx/visual-failure-fx.component.ts — FEAT-113 Visual Failure FX Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import type { SigUiProfile } from "@signal/design-system";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildVisualFailureFxVm, DEFAULT_SOA_SCENARIOS } from "./visual-failure-fx.logic";
import type { AutoFixUpgradeEvent } from "./visual-failure-fx.model";

@Component({
  selector: "sig-visual-failure-fx",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./visual-failure-fx.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class VisualFailureFxComponent {
  readonly scenarios = DEFAULT_SOA_SCENARIOS;
  readonly modes: readonly SigUiProfile[] = ["EXPLORER", "MAKER"];

  readonly selectedScenarioId = signal<string>(DEFAULT_SOA_SCENARIOS[0].id);
  readonly activeMode = signal<SigUiProfile>("EXPLORER");
  readonly autoFixed = signal<boolean>(false);
  readonly ratingAutoFixed = output<AutoFixUpgradeEvent>();

  readonly activeScenario = computed(
    () => this.scenarios.find((s) => s.id === this.selectedScenarioId()) ?? this.scenarios[0],
  );
  readonly vm = computed(() =>
    buildVisualFailureFxVm(this.activeScenario(), this.activeMode(), this.autoFixed()),
  );

  selectScenario(id: string): void {
    this.selectedScenarioId.set(id);
    this.autoFixed.set(false);
  }

  selectMode(mode: SigUiProfile): void {
    this.activeMode.set(mode);
  }

  toggleAutoFix(): void {
    const next = !this.autoFixed();
    this.autoFixed.set(next);
    if (next) {
      const s = this.activeScenario();
      this.ratingAutoFixed.emit({
        entityId: s.entityId,
        upgradeMpn: s.upgradeMpn,
        newRatedVoltageUv: s.upgradeVoltageUv,
        newRatedPowerUw: s.upgradePowerUw,
      });
    }
  }
}
