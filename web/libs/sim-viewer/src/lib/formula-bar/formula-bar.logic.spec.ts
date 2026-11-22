// Module: web/libs/sim-viewer/src/lib/formula-bar/formula-bar.logic.spec.ts — FEAT-082 Formula DSL Unit Tests
// Invariants: Verifies tokenizer classification, unbalanced paren rejection, trapezoidal integration, and RMS math.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import {
  buildWaveformFormulaBarVm,
  evaluateFormulaOnSampleVectors,
  suggestFormulaCompletions,
  tokenizeWaveformFormula,
} from "./formula-bar.logic";

describe("WaveformFormulaBar tokenizer & evaluator (FEAT-082)", () => {
  it("tokenizes nested waveform expressions and rejects unbalanced parentheses", () => {
    const ok = tokenizeWaveformFormula("integrate(v(out)*i(L1))");
    expect(ok.valid).toBe(true);
    expect(ok.tokens[0]).toEqual({ kind: "FUNC", text: "integrate" });
    expect(ok.tokens[2]).toEqual({ kind: "SIGNAL_ACCESS", text: "v" });

    const broken = tokenizeWaveformFormula("fft_db(v(sw)");
    expect(broken.valid).toBe(false);
    expect(broken.errorHint).toContain("Unclosed '('");
  });

  it("computes trapezoidal energy integral, true RMS, and autocomplete matches", () => {
    const energy = evaluateFormulaOnSampleVectors("integrate(v(out)*i(L1))", 1e-6, [10, 10], [2, 2]);
    expect(energy).toBe("E = 20.00 uJ");

    const rms = evaluateFormulaOnSampleVectors("rms(i(L1))", 1e-6, [10, 10], [3, 3]);
    expect(rms).toBe("I_rms = 3.000 A");

    expect(suggestFormulaCompletions("group")).toEqual(["group_delay(v(out))"]);
    expect(buildWaveformFormulaBarVm("rms(i(L1))").statusTone).toBe("ok");
  });
});
