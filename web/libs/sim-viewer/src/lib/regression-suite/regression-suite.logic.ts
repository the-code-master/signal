// Module: web/libs/sim-viewer/src/lib/regression-suite/regression-suite.logic.ts — Spec-Assertion Parser & Margin Evaluator
// Invariants: Parses `<metric> <op> <limit>` (>, >=, <, <=) deterministically; computes signed specification margin %.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { EvaluatedSpecAssertion, RegressionSuiteVm } from "./regression-suite.model";

export const DEFAULT_ASSERTION_EXPRESSIONS: readonly string[] = [
  "phase_margin_deg > 45",
  "vout_ripple_mv < 15",
  "efficiency_pct >= 97.5",
  "tj_max_c <= 150",
];

export const DEFAULT_SIM_METRICS: Readonly<Record<string, number>> = {
  phase_margin_deg: 61.4,
  vout_ripple_mv: 9.8,
  efficiency_pct: 98.4,
  tj_max_c: 132.0,
};

const ASSERTION_COLUMNS: readonly SigGridColumn[] = [
  { key: "expression", header: "Executable Spec Assertion", mono: true },
  { key: "measured", header: "Measured", mono: true },
  { key: "margin", header: "Spec Margin", mono: true },
  { key: "verdict", header: "Verdict", mono: true },
];

export function evaluateSpecAssertion(
  id: string,
  expression: string,
  telemetry: Readonly<Record<string, number>>,
): EvaluatedSpecAssertion {
  const match = /^([A-Za-z_][A-Za-z0-9_]*)\s*(>=|<=|>|<)\s*(-?\d+(?:\.\d+)?)$/.exec(expression.trim());
  if (!match) {
    return { id, expression, measuredValue: 0, passed: false, marginPct: -100 };
  }
  const metric = match[1];
  const op = match[2];
  const limit = Number(match[3]);
  const measuredValue = telemetry[metric];
  if (measuredValue === undefined) {
    return { id, expression, measuredValue: 0, passed: false, marginPct: -100 };
  }

  let passed = false;
  let signedSlack = 0;
  if (op === ">") {
    passed = measuredValue > limit;
    signedSlack = measuredValue - limit;
  } else if (op === ">=") {
    passed = measuredValue >= limit;
    signedSlack = measuredValue - limit;
  } else if (op === "<") {
    passed = measuredValue < limit;
    signedSlack = limit - measuredValue;
  } else {
    passed = measuredValue <= limit;
    signedSlack = limit - measuredValue;
  }

  const denom = Math.abs(limit) > 1e-9 ? Math.abs(limit) : 1;
  const marginPct = Number(((signedSlack / denom) * 100).toFixed(2));
  return { id, expression, measuredValue, passed, marginPct };
}

export function buildRegressionSuiteVm(
  expressions: readonly string[],
  telemetry: Readonly<Record<string, number>>,
): RegressionSuiteVm {
  const results = expressions.map((expr, i) => evaluateSpecAssertion(`asrt-${i + 1}`, expr, telemetry));
  const passCount = results.filter((r) => r.passed).length;
  const allPassed = results.length > 0 && passCount === results.length;
  const gateTone: SigStatusTone = allPassed ? "ok" : "danger";
  const rows: SigGridRow[] = results.map((r) => ({
    id: r.id,
    expression: r.expression,
    measured: r.measuredValue.toFixed(2),
    margin: `${r.marginPct >= 0 ? "+" : ""}${r.marginPct.toFixed(1)}%`,
    verdict: r.passed ? "PASS" : "FAIL",
  }));

  return {
    allPassed,
    gateBadge: `Regression Gate: ${passCount}/${results.length} Passed`,
    gateTone,
    columns: ASSERTION_COLUMNS,
    rows,
  };
}
