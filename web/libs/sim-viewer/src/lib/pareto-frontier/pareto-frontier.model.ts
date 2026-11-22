// Module: web/libs/sim-viewer/src/lib/pareto-frontier/pareto-frontier.model.ts — FEAT-103 Multi-Objective Pareto Models
// Invariants: Cost (_micro_usd), area (_sq_nm), Peak Tj (_mkelvin), and carbon (_mg_co2e) stay in ES2020 bigint.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";

export type ParetoPriorityGoal = "MAX_EFFICIENCY" | "MIN_COST" | "MIN_AREA" | "MIN_CARBON";

export interface ParetoCandidatePoint {
  readonly branchId: string;
  readonly bomCostMicroUsd: bigint;
  readonly efficiencyPct: number;
  readonly pcbAreaSqNm: bigint;
  readonly peakTjMkelvin: bigint;
  readonly embodiedCarbonMgCo2e: bigint;
}

export interface ParetoFrontierVm {
  readonly goal: ParetoPriorityGoal;
  readonly goals: readonly ParetoPriorityGoal[];
  readonly frontier: readonly ParetoCandidatePoint[];
  readonly recommended: ParetoCandidatePoint;
  readonly recommendedBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
