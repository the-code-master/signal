// Module: web/libs/academy/src/lib/lab-runner/lab-runner.model.ts — Guided Circuit Lab & LTI 1.3 Models (FEAT-112)
// Invariants: Pure readonly TypeScript types; 64-bit rubric seeds and _ps/_uv telemetry use bigint ONLY.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type LtiLmsPlatform = "CANVAS_LTI13" | "MOODLE_LTI13" | "BLACKBOARD_LTI13";
export type LabFaultCondition = "NOMINAL" | "OPEN_FEEDBACK_R2" | "SHORTED_COUT" | "THERMAL_DERATE_85C";

export interface LabRubricCheck {
  readonly rubricId: string;
  readonly faultCondition: LabFaultCondition;
  readonly criterion: string;
  readonly measuredOutputUv: bigint;
  readonly targetMinUv: bigint;
  readonly targetMaxUv: bigint;
  readonly settlingTimePs: bigint;
  readonly passed: boolean;
}

export interface AutoGraderRunResult {
  readonly labAssignmentId: string;
  readonly randomizedRubricSeed: bigint;
  readonly lmsPlatform: LtiLmsPlatform;
  readonly passedFaultRubrics: number;
  readonly totalFaultRubrics: number;
  readonly scorePct: number;
  readonly lti13GradePassbackReceipt: string;
  readonly graderMlDsa65Signature: string;
  readonly checks: readonly LabRubricCheck[];
}

export interface LabRunnerVm {
  readonly lmsPlatforms: readonly LtiLmsPlatform[];
  readonly faultModes: readonly LabFaultCondition[];
  readonly activeFault: LabFaultCondition;
  readonly compensationFixed: boolean;
  readonly result: AutoGraderRunResult;
  readonly scoreTone: SigStatusTone;
  readonly scoreBadge: string;
  readonly rubricColumns: readonly SigGridColumn[];
  readonly rubricRows: readonly SigGridRow[];
}
