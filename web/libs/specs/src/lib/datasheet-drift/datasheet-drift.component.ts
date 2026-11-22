// Module: web/libs/specs/src/lib/datasheet-drift/datasheet-drift.component.ts — FEAT-078 Drift Drawer Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildDatasheetDriftVm } from "./datasheet-drift.logic";

@Component({
  selector: "sig-datasheet-drift",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./datasheet-drift.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DatasheetDriftComponent {
  readonly selectedNoticeId = signal<string>("DRIFT-UCC21750-REVD");
  readonly acknowledgedIds = signal<ReadonlySet<string>>(new Set());

  readonly vm = computed(() =>
    buildDatasheetDriftVm(this.selectedNoticeId(), this.acknowledgedIds()),
  );

  selectNotice(noticeId: string): void {
    this.selectedNoticeId.set(noticeId);
  }

  acknowledgeAndPinSelected(): void {
    const next = new Set(this.acknowledgedIds());
    next.add(this.selectedNoticeId());
    this.acknowledgedIds.set(next);
  }
}
