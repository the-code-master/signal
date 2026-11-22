// Module: web/libs/code-ide/src/lib/memmap-pinmux-energy/memmap-pinmux-energy.logic.spec.ts — Unit Tests
// Invariants: Verifies 64-bit interval overlap detection, MaxSAT pinmux resolution, and bigint nJ battery life.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  DEFAULT_MEMMAP_REGIONS,
  buildMemmapPinmuxEnergyVm,
  countAddressRangeOverlaps,
} from "./memmap-pinmux-energy.logic";

describe("memmap-pinmux-energy.logic (FEAT-054/055/056)", () => {
  it("detects colliding SVD/LD regions and computes multi-year deep-sleep battery life in bigint nJ", () => {
    expect(countAddressRangeOverlaps(DEFAULT_MEMMAP_REGIONS)).toBe(0);
    const colliding = [
      ...DEFAULT_MEMMAP_REGIONS,
      { regionId: "BAD_HEAP", sourceKind: "LINKER_LD" as const, startAddr: 0x2007_0000n, endAddr: 0x2009_0000n },
    ];
    expect(countAddressRangeOverlaps(colliding)).toBe(1);

    const vm = buildMemmapPinmuxEnergyVm(true, true);
    expect(vm.summaryTone).toBe("ok");
    expect(vm.estimatedBatteryYears).toBeGreaterThan(3.0);
  });
});
