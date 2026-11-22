// Module: web/libs/academy/src/lib/lab-runner/lab-runner.component.ts — Guided Lab & Auto-Grader UI (FEAT-112)
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; emits LTI 1.3 passback receipt on auto-grade.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildLabRunnerVm } from "./lab-runner.logic";
import type { LabFaultCondition, LtiLmsPlatform } from "./lab-runner.model";

@Component({
  selector: "sig-lab-runner",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./lab-runner.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class LabRunnerComponent {
  readonly gradePassbackSubmitted = output<string>();

  readonly lmsPlatform = signal<LtiLmsPlatform>("CANVAS_LTI13");
  readonly activeFault = signal<LabFaultCondition>("NOMINAL");
  readonly compensationFixed = signal<boolean>(false);
  readonly rubricSeed = signal<bigint>(9_007_199_254_740_993n);

  readonly vm = computed(() =>
    buildLabRunnerVm(
      this.lmsPlatform(),
      this.activeFault(),
      this.compensationFixed(),
      this.rubricSeed(),
    ),
  );

  selectLms(platform: LtiLmsPlatform): void {
    this.lmsPlatform.set(platform);
  }

  injectFault(fault: LabFaultCondition): void {
    this.activeFault.set(fault);
  }

  tuneType3Compensation(): void {
    this.compensationFixed.set(true);
  }

  runHeadlessAutoGrader(): void {
    this.rubricSeed.update((s) => s + 2n);
    this.gradePassbackSubmitted.emit(this.vm().result.lti13GradePassbackReceipt);
  }
}
