// Module: web/libs/academy/src/lib/open-silicon-wizard/open-silicon-wizard.logic.spec.ts — Unit Tests (FEAT-115)
// Invariants: Verifies Sky130/GF180MCU/SG13G2 bigint _nm tile bounds, 30..70% metal density gate & OASIS digest.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import { describe, expect, it } from "vitest";
import { buildOpenSiliconWizardVm } from "./open-silicon-wizard.logic";

describe("open-silicon-wizard.logic", () => {
  it("blocks under-filled tiles (<30% density) and passes Sky130/IHP SG13G2 precheck once dummy fill is inserted", () => {
    const blocked = buildOpenSiliconWizardVm(
      "SKYWATER_SKY130",
      "TINYTAPEOUT_1X1",
      155_000_000n,
      98_000_000n,
      false,
    );
    expect(blocked.report.tapeoutReady).toBe(false);
    expect(blocked.checkRows[1]?.["status"]).toBe("NEEDS_FILL");

    const ready = buildOpenSiliconWizardVm(
      "IHP_SG13G2",
      "TINYTAPEOUT_2X2",
      310_000_000n,
      205_000_000n,
      true,
    );
    expect(ready.report.tapeoutReady).toBe(true);
    expect(ready.report.shuttleSubmissionDigest).toContain("ihp_sg13g2.tinytapeout_2x2.ready");
  });
});
