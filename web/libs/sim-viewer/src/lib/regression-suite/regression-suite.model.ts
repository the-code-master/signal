// Module: web/libs/sim-viewer/src/lib/regression-suite/regression-suite.model.ts — FEAT-086 Spec-Assertion Regression Console Models
// Invariants: Pure deterministic assertion evaluation without eval(); tracks margin percentage and gate status.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface EvaluatedSpecAssertion {
  readonly id: string;
  readonly expression: string;
  readonly measuredValue: number;
  readonly passed: boolean;
  readonly marginPct: number;
}

export interface RegressionSuiteVm {
  readonly allPassed: boolean;
  readonly gateBadge: string;
  readonly gateTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
