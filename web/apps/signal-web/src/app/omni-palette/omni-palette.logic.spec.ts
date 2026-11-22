// Module: web/apps/signal-web/src/app/omni-palette/omni-palette.logic.spec.ts — FEAT-091 Omni-Palette Unit Tests
// Invariants: Verifies place/connect/route/sim/goto command parsing with bigint nm coordinates and fuzzy jump ranking.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { filterNavLinksForProfile } from "../shell/ide-shell.logic";
import { parseOmniCliCommand, rankFuzzyWorkbenchJumps } from "./omni-palette.logic";

describe("OmniPalette CLI parser & fuzzy jump (FEAT-091)", () => {
  it("parses place, connect, route, sim, and goto CLI commands", () => {
    const place = parseOmniCliCommand("place R 10k at 1250000,2500000");
    expect(place.valid).toBe(true);
    expect(place.verb).toBe("PLACE");
    expect(place.xNm).toBe(1_250_000n);
    expect(place.yNm).toBe(2_500_000n);

    expect(parseOmniCliCommand("connect U1.3 HV_BUS").verb).toBe("CONNECT");
    expect(parseOmniCliCommand("route HV_BUS width 250000").verb).toBe("ROUTE");
    expect(parseOmniCliCommand("sim tran 50u").targetWorkbenchSlug).toBe("sim-viewer");
    expect(parseOmniCliCommand("goto academy").targetWorkbenchSlug).toBe("academy");
  });

  it("ranks exact and substring workbench matches deterministically", () => {
    const links = filterNavLinksForProfile("signal-design-inc", "aurora", "PRO");
    const ranked = rankFuzzyWorkbenchJumps("magnet", links);
    expect(ranked[0].slug).toBe("magnetics");
  });
});
