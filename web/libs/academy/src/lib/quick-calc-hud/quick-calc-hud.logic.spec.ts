// Module: web/libs/academy/src/lib/quick-calc-hud/quick-calc-hud.logic.spec.ts — FEAT-114 Quick-Calc Unit Tests
// Invariants: Verifies 4/5/6-band decoding, EIA-96 `01C` -> 10 kΩ (10_000_000 mΩ), divider microvolts, and LED resistor math.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  computeLedSeriesResistor,
  computeVoltageDivider,
  decodeColorBandResistor,
  decodeEia96SmdCode,
} from "./quick-calc-hud.logic";

describe("QuickCalcHud engineering calculators (FEAT-114)", () => {
  it("decodes 4-band, 5-band, and 6-band color resistors in bigint milliohms", () => {
    const r4 = decodeColorBandResistor(["yellow", "violet", "red", "gold"]);
    expect(r4.nominalMohm).toBe(4_700_000n);
    expect(r4.tolerancePct).toBe(5);

    const r6 = decodeColorBandResistor(["brown", "black", "black", "red", "brown", "red"]);
    expect(r6.nominalMohm).toBe(10_000_000n);
    expect(r6.tolerancePct).toBe(1);
    expect(r6.tempcoPpmPerK).toBe(50);
  });

  it("decodes EIA-96 SMD `01C` -> 10 kΩ (10_000_000 mΩ) and computes divider & LED resistor", () => {
    const smd = decodeEia96SmdCode("01C");
    expect(smd.valid).toBe(true);
    expect(smd.nominalMohm).toBe(10_000_000n);
    expect(smd.formattedSummary).toContain("10 kΩ");

    const div = computeVoltageDivider(12_000_000n, 30_000_000n, 10_000_000n);
    expect(div.vOutUv).toBe(3_000_000n);

    const led = computeLedSeriesResistor(5_000_000n, 2_000_000n, 20_000_000n);
    expect(led.exactMohm).toBe(150_000n);
    expect(led.nearestE24Mohm).toBe(150_000n);
    expect(led.powerDissUw).toBe(60_000n);
  });
});
