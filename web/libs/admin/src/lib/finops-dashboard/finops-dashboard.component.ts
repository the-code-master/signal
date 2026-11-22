// Module: web/libs/admin/src/lib/finops-dashboard/finops-dashboard.component.ts — FinOps Telemetry UI (FEAT-116)
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS or browser storage.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { SEEDED_TEAM_FINOPS, buildFinOpsDashboardVm } from "./finops-dashboard.logic";

@Component({
  selector: "sig-finops-dashboard",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./finops-dashboard.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class FinOpsDashboardComponent {
  readonly burstGpuMicrosUsd = signal<bigint>(0n);

  readonly vm = computed(() =>
    buildFinOpsDashboardVm(SEEDED_TEAM_FINOPS, this.burstGpuMicrosUsd()),
  );

  simulate3dEmGpuBurst(): void {
    this.burstGpuMicrosUsd.update((cur) => (cur === 0n ? 1_100_000_000n : 0n));
  }
}
