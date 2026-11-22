// Module: web/libs/sim-viewer/src/lib/anomaly-debugger/anomaly-debugger.component.ts — FEAT-106 Anomaly Debugger Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildAnomalyDebuggerVm } from "./anomaly-debugger.logic";
import type { AnomalySymptomId } from "./anomaly-debugger.model";

@Component({
  selector: "sig-anomaly-debugger",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./anomaly-debugger.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AnomalyDebuggerComponent {
  readonly activeSymptom = signal<AnomalySymptomId>("SW_RINGING_180MHZ");
  readonly appliedFix = signal<string | null>(null);

  readonly vm = computed(() =>
    buildAnomalyDebuggerVm(this.activeSymptom(), this.appliedFix()),
  );

  selectSymptom(id: AnomalySymptomId): void {
    this.activeSymptom.set(id);
    this.appliedFix.set(null);
  }

  applyTopMitigation(): void {
    const top = this.vm().selectedCase.rankedMitigations[0] ?? null;
    this.appliedFix.set(top);
  }
}
