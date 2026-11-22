// Module: web/apps/signal-web/src/app/nl-constraint-composer/nl-constraint-composer.logic.spec.ts — FEAT-105 Compiler Tests
// Invariants: Verifies deterministic DRC, UPF, SDC, and SVA compilation from natural-language prompts.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { compileNlPromptToRule } from "./nl-constraint-composer.logic";

describe("NlConstraintComposer rule compiler (FEAT-105)", () => {
  it("compiles NL prompts into deterministic DRC, UPF, SDC, and SVA rules", () => {
    expect(compileNlPromptToRule("Route PCIe6 at 85 ohm diff", "DRC").compiledRuleDsl).toContain("85000");
    expect(compileNlPromptToRule("Retention domain at 800 mV", "UPF").compiledRuleDsl).toContain("create_power_domain");
    expect(compileNlPromptToRule("Core clock 250 MHz", "SDC").compiledRuleDsl).toContain("create_clock");
    expect(compileNlPromptToRule("Never turn on HS and LS within 4 cycles", "SVA").compiledRuleDsl).toContain("assert property");
  });
});
