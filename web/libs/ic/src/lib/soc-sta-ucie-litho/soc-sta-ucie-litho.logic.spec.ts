// Module: web/libs/ic/src/lib/soc-sta-ucie-litho/soc-sta-ucie-litho.logic.spec.ts — Unit Tests
// Invariants: Verifies Post-Mask Spare-Cell Metal ECO closes negative bigint fs slack and CMP fill window.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildSocStaUcieLithoVm,
  computePostEcoWnsFs,
} from "./soc-sta-ucie-litho.logic";

describe("soc-sta-ucie-litho.logic (FEAT-045..049 & F-125)", () => {
  it("closes -14,200 fs setup slack to +14,300 fs via Metal-Only Spare-Cell ECO and verifies 5000 nm Halo", () => {
    expect(computePostEcoWnsFs(-14_200n, false)).toBe(-14_200n);
    expect(computePostEcoWnsFs(-14_200n, true)).toBe(14_300n);

    const closedVm = buildSocStaUcieLithoVm(true, true);
    expect(closedVm.staGateTone).toBe("ok");
    expect(closedVm.haloRingWidthNm).toBe(5_000n);
  });
});
