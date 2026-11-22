// Module: web/apps/signal-web/src/app/inline-translate/inline-translate.logic.spec.ts — FEAT-110 Unit Tests
// Invariants: Verifies all 15 technical locales preserve RefDes, NetName, and SI literals 100% verbatim.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md
import { describe, expect, it } from "vitest";
import {
  SUPPORTED_TECHNICAL_LOCALES,
  buildInlineTranslateVm,
  extractProtectedEngineeringTokens,
  translateWithQuarantinedGlossary,
} from "./inline-translate.logic";

describe("inline-translate.logic (FEAT-110)", () => {
  it("supports 15 technical locales and preserves RefDes, NetNames, and SI literals verbatim", () => {
    expect(SUPPORTED_TECHNICAL_LOCALES).toHaveLength(15);
    const note = "Place R14 10kΩ near U2 on HV_DC_BUS_800V with 8000000nm clearance";
    const tokens = extractProtectedEngineeringTokens(note);
    expect(tokens).toEqual(["R14", "10kΩ", "U2", "HV_DC_BUS_800V", "8000000nm"]);

    for (const locale of SUPPORTED_TECHNICAL_LOCALES) {
      const res = translateWithQuarantinedGlossary(note, locale);
      expect(res.allTokensIntact).toBe(true);
      for (const tok of tokens) {
        expect(res.translatedText).toContain(tok);
      }
    }
  });

  it("builds view-model with ZERO_TOOL_QUARANTINE badge", () => {
    const vm = buildInlineTranslateVm("Place C19 100nF near Q1", "ja");
    expect(vm.quarantineTone).toBe("ok");
    expect(vm.result.translatedText).toContain("C19");
    expect(vm.result.translatedText).toContain("100nF");
  });
});
