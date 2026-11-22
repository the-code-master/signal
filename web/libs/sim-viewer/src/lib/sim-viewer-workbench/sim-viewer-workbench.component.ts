// Module: web/libs/sim-viewer/src/lib/sim-viewer-workbench/sim-viewer-workbench.component.ts — Sim-Viewer Workbench Host
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; wires CU-P1-03 + CU-P1-04.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import type { WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { WaveformFormulaBarComponent } from "../formula-bar/formula-bar.component";
import { JitterStudioComponent } from "../jitter-studio/jitter-studio.component";
import { MonteCarloStudioComponent } from "../monte-carlo/monte-carlo.component";
import { RegressionSuiteComponent } from "../regression-suite/regression-suite.component";
import { StimulusAuditionComponent } from "../stimulus-audition/stimulus-audition.component";
import { VirtualBenchDeckComponent } from "../virtual-bench/virtual-bench.component";
import { WaveformCanvasComponent } from "../waveform-canvas/waveform-canvas.component";
import { WhatIfScrubberComponent } from "../what-if-scrubber/what-if-scrubber.component";
import { buildSimViewerWorkbenchVm } from "./sim-viewer-workbench.logic";
import type { SimStudioTabId } from "./sim-viewer-workbench.model";

@Component({
  selector: "sig-sim-viewer-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    WaveformCanvasComponent,
    WaveformFormulaBarComponent,
    VirtualBenchDeckComponent,
    MonteCarloStudioComponent,
    JitterStudioComponent,
    WhatIfScrubberComponent,
    RegressionSuiteComponent,
    StimulusAuditionComponent,
  ],
  templateUrl: "./sim-viewer-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SimViewerWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();

  readonly activeTab = signal<SimStudioTabId>("VIRTUAL_BENCH");
  readonly probedNet = signal<string>("HV_SW_PHASE_U");
  readonly activeFormula = signal<string>("integrate(v(out)*i(L1))");

  readonly vm = computed(() =>
    buildSimViewerWorkbenchVm(
      this.workbench(),
      this.activeTab(),
      this.probedNet(),
      this.activeFormula(),
    ),
  );

  selectTab(tab: SimStudioTabId): void {
    this.activeTab.set(tab);
  }

  onNetCrossProbed(net: string): void {
    this.probedNet.set(net);
  }

  onFormulaCommitted(expr: string): void {
    this.activeFormula.set(expr);
  }
}
