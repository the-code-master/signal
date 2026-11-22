// Module: web/libs/specs/src/lib/bom-consolidation/bom-consolidation.component.ts — FEAT-108 BOM Consolidator Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildBomConsolidationVm } from "./bom-consolidation.logic";

@Component({
  selector: "sig-bom-consolidation",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./bom-consolidation.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BomConsolidationComponent {
  readonly maxAllowedShiftPct = signal<number>(0.25);
  readonly presets: readonly number[] = [0.05, 0.25, 0.5];

  readonly vm = computed(() => buildBomConsolidationVm(this.maxAllowedShiftPct()));

  selectShiftCap(pct: number): void {
    this.maxAllowedShiftPct.set(pct);
  }
}
