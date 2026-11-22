// Module: web/libs/admin/src/lib/portfolio-graph/portfolio-graph.logic.spec.ts — Unit Tests (FEAT-118)
// Invariants: Verifies multi-hop shared-IP / MPN transitive blast radius and bigint _micros_usd aggregation.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import { describe, expect, it } from "vitest";
import { buildPortfolioGraphVm } from "./portfolio-graph.logic";

describe("portfolio-graph.logic", () => {
  it("computes 2-hop transitive blast radius and $29.7M bigint revenue exposure for C3M0016120K EOL", () => {
    const vm = buildPortfolioGraphVm("C3M0016120K_SIC_EOL");
    expect(vm.summary.impactedProjectSlugs).toEqual([
      "aurora-ev-inverter",
      "hyperion-350kw-charger",
    ]);
    expect(vm.summary.transitiveDepth).toBe(2);
    expect(vm.summary.estimatedRevenueAtRiskMicrosUsd).toBe(29_700_000_000_000n);
  });
});
