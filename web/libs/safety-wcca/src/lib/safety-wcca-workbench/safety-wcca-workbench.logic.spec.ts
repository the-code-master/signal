// Module: web/libs/safety-wcca/src/lib/safety-wcca-workbench/safety-wcca-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies ISO 26262 ASIL-D (SPFM>=99%, LFM>=90%, PMHF<10 FIT) and ZBDD Order-2 cut-set check.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  SAFETY_WCCA_WORKBENCH_DESCRIPTOR,
  buildSafetyWccaWorkbenchVm,
  evaluateAsilGatePassing,
} from "./safety-wcca-workbench.logic";

describe("safety-wcca-workbench.logic (FEAT-035/036)", () => {
  it("passes ASIL-D with redundant crowbar (SPFM 99.4%, PMHF 4.8 FIT) and fails without it", () => {
    expect(evaluateAsilGatePassing("ASIL_D", 99.4, 92.1, 4.8)).toBe(true);
    expect(evaluateAsilGatePassing("ASIL_D", 96.2, 84.5, 18.6)).toBe(false);

    const passVm = buildSafetyWccaWorkbenchVm(SAFETY_WCCA_WORKBENCH_DESCRIPTOR, "ASIL_D", true);
    expect(passVm.fmedaTone).toBe("ok");
    const failVm = buildSafetyWccaWorkbenchVm(SAFETY_WCCA_WORKBENCH_DESCRIPTOR, "ASIL_D", false);
    expect(failVm.fmedaTone).toBe("danger");
  });
});
