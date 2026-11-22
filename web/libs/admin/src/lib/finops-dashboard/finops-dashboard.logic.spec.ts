// Module: web/libs/admin/src/lib/finops-dashboard/finops-dashboard.logic.spec.ts — Unit Tests (FEAT-116)
// Invariants: Verifies 80% SOFT_WARN_80_PCT and 100% HARD_CAP_100_PCT_TRIPPED quota gates in bigint _micros_usd.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import { describe, expect, it } from "vitest";
import { SEEDED_TEAM_FINOPS, buildFinOpsDashboardVm } from "./finops-dashboard.logic";

describe("finops-dashboard.logic", () => {
  it("flags 80% soft budget warning and trips hard 100% quota cap on GPU burst", () => {
    const softWarn = buildFinOpsDashboardVm(SEEDED_TEAM_FINOPS, 0n);
    expect(softWarn.rollup.alertState).toBe("SOFT_WARN_80_PCT");
    expect(softWarn.rollup.hardBudgetCapTripped).toBe(false);

    const tripped = buildFinOpsDashboardVm(SEEDED_TEAM_FINOPS, 1_100_000_000n);
    expect(tripped.rollup.alertState).toBe("HARD_CAP_100_PCT_TRIPPED");
    expect(tripped.rollup.hardBudgetCapTripped).toBe(true);
    expect(tripped.alertTone).toBe("danger");
  });
});
