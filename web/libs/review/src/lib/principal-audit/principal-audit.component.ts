// Module: web/libs/review/src/lib/principal-audit/principal-audit.component.ts — FEAT-104 Red-Team Audit Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildPrincipalAuditVm } from "./principal-audit.logic";
import type { AuditSeverityFilter } from "./principal-audit.model";

@Component({
  selector: "sig-principal-audit",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./principal-audit.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PrincipalAuditComponent {
  readonly activeFilter = signal<AuditSeverityFilter>("ALL");
  readonly fixedCodes = signal<ReadonlySet<number>>(new Set());

  readonly vm = computed(() =>
    buildPrincipalAuditVm(this.activeFilter(), this.fixedCodes()),
  );

  selectFilter(filter: AuditSeverityFilter): void {
    this.activeFilter.set(filter);
  }

  autoFixAllOpen(): void {
    this.fixedCodes.set(new Set([104, 142, 189]));
  }
}
