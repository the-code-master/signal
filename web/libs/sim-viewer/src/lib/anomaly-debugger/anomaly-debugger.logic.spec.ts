// Module: web/libs/sim-viewer/src/lib/anomaly-debugger/anomaly-debugger.logic.spec.ts — FEAT-106 Anomaly Debugger Tests
// Invariants: Verifies parasitic/derating root-cause attribution and ranked mitigation rows.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildAnomalyDebuggerVm } from "./anomaly-debugger.logic";

describe("AnomalyDebugger root-cause ranker (FEAT-106)", () => {
  it("attributes 180MHz switch ringing and DC-bias droop to exact culprit entities", () => {
    const ring = buildAnomalyDebuggerVm("SW_RINGING_180MHZ", null);
    expect(ring.selectedCase.culpritEntityId).toBe("LOOP_HB_DECAP_U1");
    expect(ring.rows.length).toBe(3);

    const droop = buildAnomalyDebuggerVm("LOAD_STEP_DROOP", "1. Upgrade 0805");
    expect(droop.selectedCase.dominantRootCause).toContain("DC-Bias Derating");
  });
});
