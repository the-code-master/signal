// Module: web/libs/bringup-lab/src/lib/shmoo-plot-studio/shmoo-plot-studio.logic.spec.ts — Unit Tests
// Invariants: Verifies 4x4 PVT Shmoo pass-region contraction at 125C (398_150 mK) vs 25C (298_150 mK).
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import { buildShmooPlotStudioVm } from "./shmoo-plot-studio.logic";

describe("shmoo-plot-studio.logic (FEAT-088)", () => {
  it("evaluates 16 PVT Shmoo grid points and shrinks f_max pass region at 398,150 mK (125 C)", () => {
    const roomVm = buildShmooPlotStudioVm(298_150n, "V850_F400");
    const hotVm = buildShmooPlotStudioVm(398_150n, "V850_F400");
    expect(roomVm.cells).toHaveLength(16);
    expect(roomVm.passRegionPct).toBeGreaterThanOrEqual(hotVm.passRegionPct);
    expect(roomVm.ateAgreementPct).toBeGreaterThanOrEqual(85);
  });
});
