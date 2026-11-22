// Module: web/libs/admin/src/lib/admin-workbench/admin-workbench.component.ts — Admin Posture & AVL Workbench Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/08-identity-enterprise-and-project-model.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { AvlWhereUsedConsoleComponent } from "../avl-where-used-console/avl-where-used-console.component";
import { buildAdminWorkbenchVm } from "./admin-workbench.logic";

@Component({
  selector: "sig-admin-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigDataGridComponent,
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
