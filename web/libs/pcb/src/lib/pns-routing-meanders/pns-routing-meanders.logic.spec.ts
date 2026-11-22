// Module: web/libs/pcb/src/lib/pns-routing-meanders/pns-routing-meanders.logic.spec.ts — FEAT-012/015/017 Unit Tests
// Invariants: Verifies Die-to-Ball + PCB total flight-time summation and trombone meander loop synthesis.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  DEFAULT_PCIE6_CHANNELS,
  buildPnsRoutingMeandersVm,
  computeRequiredTromboneLoops,
  computeTotalFlightDelayFs,
} from "./pns-routing-meanders.logic";

describe("pns-routing-meanders.logic (FEAT-012/015/017)", () => {
  it("combines Die-to-Ball package delay with PCB flight delay in bigint fs and sizes trombone loops", () => {
    expect(computeTotalFlightDelayFs(DEFAULT_PCIE6_CHANNELS[0])).toBe(357_500n);
    expect(computeRequiredTromboneLoops(700n, 250_000n)).toBe(1);

    const vm = buildPnsRoutingMeandersVm("BUS_TRUNK_MULTI_LANE", 250_000n);
    expect(vm.skewGateTone).toBe("ok");
    expect(vm.maxIntraPairSkewFs).toBe(700n);
  });
});
