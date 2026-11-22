// Module: web/libs/ic/src/lib/upf-cpo-cryo-sec/upf-cpo-cryo-sec.logic.spec.ts — Unit Tests
// Invariants: Verifies Welch |t| > 4.5 TVLA side-channel detection and 4000 mK Cryo-CMOS temperature state.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildUpfCpoCryoSecVm,
  isTvlaSideChannelLeaking,
} from "./upf-cpo-cryo-sec.logic";

describe("upf-cpo-cryo-sec.logic (I-07, F-148 & FEAT-050)", () => {
  it("flags unmasked AES/PQC |t|=11.4 > 4.5 as TVLA leakage and passes DOM-masked |t|=1.82 at 4000 mK", () => {
    expect(isTvlaSideChannelLeaking(11.4)).toBe(true);
    expect(isTvlaSideChannelLeaking(1.82)).toBe(false);

    const leakVm = buildUpfCpoCryoSecVm(false, true);
    expect(leakVm.securityTone).toBe("danger");
    const safeVm = buildUpfCpoCryoSecVm(true, true);
    expect(safeVm.securityTone).toBe("ok");
    expect(safeVm.stageTempMkelvin).toBe(4_000n);
  });
});
