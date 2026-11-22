// Module: web/libs/fpga-studio/src/lib/die-io-sdc-planner/die-io-sdc-planner.logic.spec.ts — Unit Tests
// Invariants: Verifies BGA escape crossing elimination via FPGA pin-swap and Gray-code CDC safety.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import { buildDieIoSdcPlannerVm } from "./die-io-sdc-planner.logic";

describe("die-io-sdc-planner.logic (FEAT-051/052)", () => {
  it("eliminates 23 PCB BGA ratline crossings via bank-legal pin swap and verifies 0 unsafe CDC paths", () => {
    const rawVm = buildDieIoSdcPlannerVm(false, false);
    expect(rawVm.totalEscapeCrossings).toBe(23);
    expect(rawVm.ioSdcTone).toBe("warn");

    const cleanVm = buildDieIoSdcPlannerVm(true, true);
    expect(cleanVm.totalEscapeCrossings).toBe(0);
    expect(cleanVm.unsafeCdcPaths).toBe(0);
    expect(cleanVm.ioSdcTone).toBe("ok");
  });
});
