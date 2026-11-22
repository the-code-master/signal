// Module: web/libs/library-studio/src/lib/pin-symbol-splitter/pin-symbol-splitter.logic.spec.ts — FEAT-072 Unit Tests
// Invariants: Verifies 4-way Pin/Pad/SPICE/STEP bijection detection and functional bank chunking.
// Spec: architecture/18-universal-component-and-datasheet-repository.md
import { describe, expect, it } from "vitest";
import {
  buildPinSymbolSplitterVm,
  DEFAULT_MASTER_PINS,
  splitMasterPinsIntoBanks,
  verifyMasterPinBijection,
} from "./pin-symbol-splitter.logic";

describe("MasterPinTable & FEAT-072 Functional Bank Splitter", () => {
  it("verifies 1:1 Pin/Pad/SPICE bijection and splits large banks cleanly", () => {
    expect(verifyMasterPinBijection(DEFAULT_MASTER_PINS)).toBe(true);
    expect(verifyMasterPinBijection([DEFAULT_MASTER_PINS[0], DEFAULT_MASTER_PINS[0]])).toBe(false);

    const banks4 = splitMasterPinsIntoBanks(DEFAULT_MASTER_PINS, 4);
    expect(banks4.length).toBe(4);
    const banks1 = splitMasterPinsIntoBanks(DEFAULT_MASTER_PINS, 1);
    expect(banks1.length).toBe(8);

    const vm = buildPinSymbolSplitterVm("IEC_60617", 2);
    expect(vm.standardBadge).toContain("IEC 60617");
    expect(vm.bijectionTone).toBe("ok");
  });
});
