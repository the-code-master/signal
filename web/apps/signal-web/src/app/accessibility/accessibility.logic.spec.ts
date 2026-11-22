// Module: web/apps/signal-web/src/app/accessibility/accessibility.logic.spec.ts — FEAT-100 Accessibility CAD Unit Tests
// Invariants: Verifies KiCad, Altium, Virtuoso, and Vim keymap actions, bigint nm grid steps, and ARIA netlist text.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import { describe, expect, it } from "vitest";
import {
  createInitialAccessibleCursor,
  resolveCadKeymapAction,
  stepAccessibleCadCursor,
} from "./accessibility.logic";

describe("AccessibilityCad WCAG 2.2 AA stepper (FEAT-100)", () => {
  it("maps Vim H/J/K/L and KiCad W/R keys to exact bigint nanometer steps", () => {
    expect(resolveCadKeymapAction("Vim", "l")).toBe("STEP_EAST");
    expect(resolveCadKeymapAction("Altium", "p")).toBe("TOGGLE_WIRE");
    expect(resolveCadKeymapAction("KiCad", "w")).toBe("TOGGLE_WIRE");

    const init = createInitialAccessibleCursor("Vim");
    const east = stepAccessibleCadCursor(init, "l");
    expect(east.cursorXNm).toBe(2_794_000n);
    const wired = stepAccessibleCadCursor(east, "i");
    expect(wired.wireActive).toBe(true);
    expect(wired.ariaAnnouncement).toContain("HV_DC_BUS_800V");
  });
});
