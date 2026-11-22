// Module: web/libs/admin/src/lib/policy-studio/policy-studio.component.ts — WASM Policy-as-Code Studio UI (FEAT-117)
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS or eval/exec.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildPolicyStudioVm } from "./policy-studio.logic";
import type { PolicyPackDomain } from "./policy-studio.model";

@Component({
  selector: "sig-policy-studio",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./policy-studio.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PolicyStudioComponent {
  readonly selectedDomain = signal<PolicyPackDomain>("CREEPAGE_800V");
  readonly resignedTag = signal<string | null>(null);

  readonly vm = computed(() => buildPolicyStudioVm(this.selectedDomain(), this.resignedTag()));

  selectDomain(domain: PolicyPackDomain): void {
    this.selectedDomain.set(domain);
  }

  runFixturesAndSign(): void {
    this.resignedTag.set(`orgadmin-signed-${this.selectedDomain().toLowerCase()}-v2026`);
  }
}
