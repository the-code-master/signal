// Module: web/libs/admin/src/lib/avl-where-used-console/avl-where-used-console.component.ts — FEAT-075 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildAvlWhereUsedVm } from "./avl-where-used-console.logic";

@Component({
  selector: "sig-avl-where-used-console",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./avl-where-used-console.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AvlWhereUsedConsoleComponent {
  readonly selectedMpn = signal<string>("UCC21750QDWR");
  readonly draftedMpn = signal<string | null>(null);
  readonly ecoDrafted = output<string>();

  readonly vm = computed(() =>
    buildAvlWhereUsedVm(this.selectedMpn(), this.draftedMpn()),
  );

  selectMpn(mpn: string): void {
    this.selectedMpn.set(mpn);
  }

  draftCrossProjectEco(): void {
    const mpn = this.selectedMpn();
    this.draftedMpn.set(mpn);
    this.ecoDrafted.emit(mpn);
  }
}
