// Module: web/libs/admin/src/lib/compliance-center/compliance-center.component.ts — Compliance Center UI (CU-P8-04)
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS or raw PII display.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildComplianceCenterVm } from "./compliance-center.logic";

@Component({
  selector: "sig-compliance-center",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./compliance-center.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ComplianceCenterComponent {
  readonly saltShredded = signal<boolean>(false);

  readonly vm = computed(() => buildComplianceCenterVm(this.saltShredded()));

  executeGdprSaltShred(): void {
    this.saltShredded.set(true);
  }
}
