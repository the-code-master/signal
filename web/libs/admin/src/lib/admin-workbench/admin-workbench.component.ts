// Module: web/libs/admin/src/lib/admin-workbench/admin-workbench.component.ts — Admin Posture, FinOps & Compliance Host
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; wires FEAT-075, FEAT-116..118 & CU-P8-04 Compliance.
// Spec: architecture/08-identity-enterprise-and-project-model.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { AvlWhereUsedConsoleComponent } from "../avl-where-used-console/avl-where-used-console.component";
import { ComplianceCenterComponent } from "../compliance-center/compliance-center.component";
import { FinOpsDashboardComponent } from "../finops-dashboard/finops-dashboard.component";
import { PolicyStudioComponent } from "../policy-studio/policy-studio.component";
import { PortfolioGraphComponent } from "../portfolio-graph/portfolio-graph.component";
import { buildAdminWorkbenchVm } from "./admin-workbench.logic";

@Component({
  selector: "sig-admin-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigDataGridComponent,
    FinOpsDashboardComponent,
    PolicyStudioComponent,
    PortfolioGraphComponent,
    ComplianceCenterComponent,
    AvlWhereUsedConsoleComponent,
  ],
  templateUrl: "./admin-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AdminWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly sweptExpired = signal<boolean>(false);

  readonly vm = computed(() =>
    buildAdminWorkbenchVm(this.workbench(), this.sweptExpired()),
  );

  sweepExpiredContractors(): void {
    this.sweptExpired.set(true);
  }
}
