// Module: web/libs/sim-viewer/src/lib/pareto-frontier/pareto-frontier.logic.ts — FEAT-103 Non-Dominated Pareto Filter & Ranker
// Invariants: Prunes dominated candidates using exact bigint comparisons across Cost, Area, Peak Tj & Carbon.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type {
  ParetoCandidatePoint,
  ParetoFrontierVm,
  ParetoPriorityGoal,
} from "./pareto-frontier.model";

export const RAW_PARETO_POPULATION: readonly ParetoCandidatePoint[] = [
  {
    branchId: "cow/pareto-eff-max",
    bomCostMicroUsd: 162_000_000n,
    efficiencyPct: 99.24,
    pcbAreaSqNm: 9_800_000_000_000_000n,
    peakTjMkelvin: 382_150n,
    embodiedCarbonMgCo2e: 1_420_000n,
  },
  {
    branchId: "cow/pareto-balanced",
    bomCostMicroUsd: 118_500_000n,
    efficiencyPct: 98.75,
    pcbAreaSqNm: 7_200_000_000_000_000n,
    peakTjMkelvin: 396_150n,
    embodiedCarbonMgCo2e: 1_090_000n,
  },
  {
    branchId: "cow/pareto-compact-eco",
    bomCostMicroUsd: 94_000_000n,
    efficiencyPct: 98.12,
    pcbAreaSqNm: 5_400_000_000_000_000n,
    peakTjMkelvin: 412_150n,
    embodiedCarbonMgCo2e: 860_000n,
  },
  {
    branchId: "cow/dominated-suboptimal",
    bomCostMicroUsd: 170_000_000n,
    efficiencyPct: 97.8,
    pcbAreaSqNm: 11_000_000_000_000_000n,
    peakTjMkelvin: 430_150n,
    embodiedCarbonMgCo2e: 1_600_000n,
  },
];

const PARETO_COLUMNS: readonly SigGridColumn[] = [
  { key: "branchId", header: "CoW Candidate", mono: true },
  { key: "cost", header: "BOM Cost", mono: true },
  { key: "eff", header: "Efficiency", mono: true },
  { key: "areaTj", header: "Area / Peak Tj", mono: true },
  { key: "carbon", header: "Carbon", mono: true },
];

function dominates(a: ParetoCandidatePoint, b: ParetoCandidatePoint): boolean {
  const noWorse =
    a.bomCostMicroUsd <= b.bomCostMicroUsd &&
    a.efficiencyPct >= b.efficiencyPct &&
    a.pcbAreaSqNm <= b.pcbAreaSqNm &&
    a.peakTjMkelvin <= b.peakTjMkelvin &&
    a.embodiedCarbonMgCo2e <= b.embodiedCarbonMgCo2e;
  const strictlyBetter =
    a.bomCostMicroUsd < b.bomCostMicroUsd ||
    a.efficiencyPct > b.efficiencyPct ||
    a.pcbAreaSqNm < b.pcbAreaSqNm ||
    a.peakTjMkelvin < b.peakTjMkelvin ||
    a.embodiedCarbonMgCo2e < b.embodiedCarbonMgCo2e;
  return noWorse && strictlyBetter;
}

export function filterNonDominatedParetoCandidates(
  candidates: readonly ParetoCandidatePoint[],
): readonly ParetoCandidatePoint[] {
  return candidates.filter(
    (cand, i) => !candidates.some((other, j) => i !== j && dominates(other, cand)),
  );
}

export function buildParetoFrontierVm(
  goal: ParetoPriorityGoal,
  population: readonly ParetoCandidatePoint[] = RAW_PARETO_POPULATION,
): ParetoFrontierVm {
  const frontier = filterNonDominatedParetoCandidates(population);
  const sorted = frontier.slice().sort((a, b) => {
    if (goal === "MAX_EFFICIENCY") return b.efficiencyPct - a.efficiencyPct;
    if (goal === "MIN_COST") return a.bomCostMicroUsd < b.bomCostMicroUsd ? -1 : 1;
    if (goal === "MIN_AREA") return a.pcbAreaSqNm < b.pcbAreaSqNm ? -1 : 1;
    return a.embodiedCarbonMgCo2e < b.embodiedCarbonMgCo2e ? -1 : 1;
  });
  const recommended = sorted[0];

  const rows: SigGridRow[] = sorted.map((c) => {
    const usd = (c.bomCostMicroUsd / 1_000_000n).toString(10);
    const areaSqMm = (c.pcbAreaSqNm / 1_000_000_000_000n).toString(10);
    const tjC = ((c.peakTjMkelvin - 273_150n) / 1_000n).toString(10);
    const gCo2 = (c.embodiedCarbonMgCo2e / 1_000n).toString(10);
    return {
      id: c.branchId,
      branchId: c.branchId,
      cost: `$${usd}`,
      eff: `${c.efficiencyPct.toFixed(2)}%`,
      areaTj: `${areaSqMm} mm² • ${tjC} °C`,
      carbon: `${gCo2} gCO2e`,
    };
  });

  return {
    goal,
    goals: ["MAX_EFFICIENCY", "MIN_COST", "MIN_AREA", "MIN_CARBON"],
    frontier: sorted,
    recommended,
    recommendedBadge: `Recommended (${goal}): ${recommended.branchId} (${recommended.efficiencyPct}% Eff)`,
    columns: PARETO_COLUMNS,
    rows,
  };
}
