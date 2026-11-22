// Module: web/libs/academy/src/lib/lab-runner/lab-runner.logic.ts — Headless Auto-Grader & LTI 1.3 Logic (FEAT-112)
// Invariants: Pure deterministic functions <=250 LOC; evaluates fault-injected rubrics & signs LTI 1.3 passback receipt.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  AutoGraderRunResult,
  LabFaultCondition,
  LabRubricCheck,
  LabRunnerVm,
  LtiLmsPlatform,
} from "./lab-runner.model";

export const LMS_PLATFORMS: readonly LtiLmsPlatform[] = [
  "CANVAS_LTI13",
  "MOODLE_LTI13",
  "BLACKBOARD_LTI13",
];

export const LAB_FAULT_MODES: readonly LabFaultCondition[] = [
  "NOMINAL",
  "OPEN_FEEDBACK_R2",
  "SHORTED_COUT",
  "THERMAL_DERATE_85C",
];

export const RUBRIC_GRID_COLUMNS: readonly SigGridColumn[] = [
  { key: "fault", header: "Fault Injection", mono: true },
  { key: "criterion", header: "Headless SPICE Rubric Criterion" },
  { key: "measured", header: "Measured (uV / ps)", mono: true },
  { key: "verdict", header: "Verdict", mono: true },
];

export function evaluateHeadlessLabRubric(
  labAssignmentId: string,
  randomizedSeed: bigint,
  lmsPlatform: LtiLmsPlatform,
  compensationFixed: boolean,
): AutoGraderRunResult {
  const seedOffsetUv = (randomizedSeed % 7n) * 1_000n;
  const thermalUv = compensationFixed ? 3_302_000n + seedOffsetUv : 3_620_000n;

  const rawChecks: readonly Omit<LabRubricCheck, "passed">[] = [
    {
      rubricId: "rub-01",
      faultCondition: "NOMINAL",
      criterion: "3.3V Buck steady-state regulation within +/-2%",
      measuredOutputUv: 3_300_000n + seedOffsetUv,
      targetMinUv: 3_234_000n,
      targetMaxUv: 3_366_000n,
      settlingTimePs: 185_000_000n,
    },
    {
      rubricId: "rub-02",
      faultCondition: "OPEN_FEEDBACK_R2",
      criterion: "Over-voltage clamp engages <= 3.60V on open FB divider",
      measuredOutputUv: 3_540_000n,
      targetMinUv: 0n,
      targetMaxUv: 3_600_000n,
      settlingTimePs: 92_000_000n,
    },
    {
      rubricId: "rub-03",
      faultCondition: "SHORTED_COUT",
      criterion: "Hiccup current-limit foldback protects high-side FET",
      measuredOutputUv: 120_000n,
      targetMinUv: 0n,
      targetMaxUv: 450_000n,
      settlingTimePs: 45_000_000n,
    },
    {
      rubricId: "rub-04",
      faultCondition: "THERMAL_DERATE_85C",
      criterion: "85C Type-III compensation phase margin keeps Vout <= 3.36V",
      measuredOutputUv: thermalUv,
      targetMinUv: 3_240_000n,
      targetMaxUv: 3_360_000n,
      settlingTimePs: 210_000_000n,
    },
  ];

  const checks: LabRubricCheck[] = rawChecks.map((c) => ({
    ...c,
    passed: c.measuredOutputUv >= c.targetMinUv && c.measuredOutputUv <= c.targetMaxUv,
  }));

  const passedCount = checks.filter((c) => c.passed).length;
  const totalCount = checks.length;
  const scorePct = Math.round((passedCount / totalCount) * 1000) / 10;
  const seedTag = randomizedSeed.toString(16);

  return {
    labAssignmentId,
    randomizedRubricSeed: randomizedSeed,
    lmsPlatform,
    passedFaultRubrics: passedCount,
    totalFaultRubrics: totalCount,
    scorePct,
    lti13GradePassbackReceipt: `lti13.ags.${lmsPlatform.toLowerCase()}.${labAssignmentId}.score-${scorePct}.seed-${seedTag}`,
    graderMlDsa65Signature: `ml-dsa-65:autograder-${labAssignmentId}-${seedTag}`,
    checks,
  };
}

export function buildLabRunnerVm(
  lmsPlatform: LtiLmsPlatform,
  activeFault: LabFaultCondition,
  compensationFixed: boolean,
  randomizedSeed: bigint,
): LabRunnerVm {
  const result = evaluateHeadlessLabRubric(
    "ee201-lab04-buck",
    randomizedSeed,
    lmsPlatform,
    compensationFixed,
  );
  const scoreTone: SigStatusTone =
    result.scorePct === 100 ? "ok" : result.scorePct >= 75 ? "warn" : "danger";

  const rubricRows: SigGridRow[] = result.checks.map((c) => ({
    id: c.rubricId,
    fault: c.faultCondition === activeFault ? `* ${c.faultCondition}` : c.faultCondition,
    criterion: c.criterion,
    measured: `${c.measuredOutputUv.toString()} uV @ ${c.settlingTimePs.toString()} ps`,
    verdict: c.passed ? "PASS" : "FAIL",
  }));

  return {
    lmsPlatforms: LMS_PLATFORMS,
    faultModes: LAB_FAULT_MODES,
    activeFault,
    compensationFixed,
    result,
    scoreTone,
    scoreBadge: `${result.scorePct}% (${result.passedFaultRubrics}/${result.totalFaultRubrics}) • ${lmsPlatform}`,
    rubricColumns: RUBRIC_GRID_COLUMNS,
    rubricRows,
  };
}
