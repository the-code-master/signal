// Module: web/libs/ui-kit/src/si-input/sig-si-input.logic.spec.ts — SI Unit Disambiguation Unit Tests
// Invariants: Verifies `m` (milli) vs `M`/`Meg` (Mega) and rejects ambiguous `meg` without TestBed (F-075).
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import { parseSiInputToNano } from "./sig-si-input.logic";

describe("parseSiInputToNano", () => {
  it("disambiguates m (milli) vs M/Meg (Mega) and rejects ambiguous meg", () => {
    expect(parseSiInputToNano("10m", "Ω").nanoUnits).toBe(10_000_000n);
    expect(parseSiInputToNano("10Meg", "Ω").nanoUnits).toBe(10_000_000_000_000_000n);
    expect(parseSiInputToNano("10M", "Ω").nanoUnits).toBe(10_000_000_000_000_000n);
    expect(parseSiInputToNano("10meg", "Ω").valid).toBe(false);
  });
});
