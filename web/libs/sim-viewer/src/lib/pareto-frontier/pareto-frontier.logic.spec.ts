// Module: web/libs/sim-viewer/src/lib/pareto-frontier/pareto-frontier.logic.spec.ts — FEAT-103 Pareto Tests
// Invariants: Verifies dominated candidate pruning (4 -> 3 non-dominated points) and objective priority sorting.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildParetoFrontierVm,
  filterNonDominatedParetoCandidates,
  RAW_PARETO_POPULATION,
} from "./pareto-frontier.logic";

describe("ParetoFrontier NSGA-III non-dominance logic (FEAT-103)", () => {
  it("prunes dominated points and selects goal-optimal CoW branches", () => {
    const frontier = filterNonDominatedParetoCandidates(RAW_PARETO_POPULATION);
    expect(frontier.length).toBe(3);
    expect(frontier.some((c) => c.branchId === "cow/dominated-suboptimal")).toBe(false);

    expect(buildParetoFrontierVm("MAX_EFFICIENCY").recommended.branchId).toBe("cow/pareto-eff-max");
    expect(buildParetoFrontierVm("MIN_CARBON").recommended.branchId).toBe("cow/pareto-compact-eco");
  });
});
