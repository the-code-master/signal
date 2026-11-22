// Module: web/libs/safety-wcca/src/lib/wcca-radhard-derating-aging/wcca-radhard-derating-aging.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/16-architectural-audit-flaws-and-futuristic-innovations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildWccaRadhardDeratingAgingVm } from "./wcca-radhard-derating-aging.logic";
import type { DeratingPolicyStandard } from "./wcca-radhard-derating-aging.model";

@Component({
  selector: "sig-wcca-radhard-derating-aging",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./wcca-radhard-derating-aging.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class WccaRadhardDeratingAgingComponent {
  readonly policy = signal<DeratingPolicyStandard>("ENTERPRISE_AUTOMOTIVE");
  readonly missionYears = signal<number>(15);

  readonly vm = computed(() =>
    buildWccaRadhardDeratingAgingVm(this.policy(), this.missionYears()),
  );

  selectPolicy(p: DeratingPolicyStandard): void {
    this.policy.set(p);
  }

  selectYears(yr: number): void {
    this.missionYears.set(yr);
  }
}
