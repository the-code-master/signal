// Module: web/libs/emc-lab/src/lib/emi-esd-shield/emi-esd-shield.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildEmiEsdShieldVm } from "./emi-esd-shield.logic";
import type { EsdStressLevel } from "./emi-esd-shield.model";

@Component({
  selector: "sig-emi-esd-shield",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./emi-esd-shield.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class EmiEsdShieldComponent {
  readonly esdLevel = signal<EsdStressLevel>("IEC_15KV_AIR");
  readonly dampingEnabled = signal<boolean>(true);

  readonly vm = computed(() =>
    buildEmiEsdShieldVm(this.esdLevel(), this.dampingEnabled()),
  );

  selectEsd(level: EsdStressLevel): void {
    this.esdLevel.set(level);
  }

  toggleDamping(): void {
    this.dampingEnabled.update((v) => !v);
  }
}
