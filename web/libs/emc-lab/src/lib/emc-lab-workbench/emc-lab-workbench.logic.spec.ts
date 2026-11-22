// Module: web/libs/emc-lab/src/lib/emc-lab-workbench/emc-lab-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies CISPR-25 Class 5 +6.4 dB margin and 20 Hz..20 kHz MLCC piezo acoustic whine detection.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  EMC_LAB_WORKBENCH_DESCRIPTOR,
  buildEmcLabWorkbenchVm,
  isPiezoAcousticAudibleRisk,
} from "./emc-lab-workbench.logic";

describe("emc-lab-workbench.logic (FEAT-031/039)", () => {
  it("computes +6.4 dB CISPR-25 Class 5 margin and flags 8 kHz X7R MLCC ripple as audible piezo whine", () => {
    expect(isPiezoAcousticAudibleRisk(8_000n, "X7R")).toBe(true);
    expect(isPiezoAcousticAudibleRisk(8_000n, "C0G")).toBe(false);
    expect(isPiezoAcousticAudibleRisk(32_000n, "X7R")).toBe(false);

    const safeVm = buildEmcLabWorkbenchVm(EMC_LAB_WORKBENCH_DESCRIPTOR, "CISPR_25_CLASS_5", 32_000n);
    expect(safeVm.worstMarginDb).toBe(6.4);
    expect(safeVm.spectrumTone).toBe("ok");
  });
});
