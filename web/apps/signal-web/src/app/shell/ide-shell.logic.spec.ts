// Module: web/apps/signal-web/src/app/shell/ide-shell.logic.spec.ts — Progressive UI Profile Filter Unit Tests
// Invariants: Verifies EXPLORER < MAKER < PRO workbench visibility without TestBed (14-*).
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { describe, expect, it } from "vitest";
import { filterNavLinksForProfile } from "./ide-shell.logic";

describe("filterNavLinksForProfile", () => {
  it("progressively reveals domain workbenches from EXPLORER to MAKER to PRO", () => {
    const explorer = filterNavLinksForProfile("signal-design-inc", "aurora", "EXPLORER");
    const maker = filterNavLinksForProfile("signal-design-inc", "aurora", "MAKER");
    const pro = filterNavLinksForProfile("signal-design-inc", "aurora", "PRO");
    expect(explorer.length).toBe(6);
    expect(maker.length).toBe(12);
    expect(pro.length).toBe(25);
  });
});
