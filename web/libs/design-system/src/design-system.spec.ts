// Module: web/libs/design-system/src/design-system.spec.ts — Design System & BigInt Contract Parity Unit Tests
// Invariants: Verifies >2^53-1 bigint fixture precision in TypeScript without IEEE-754 Number truncation (F-144).
// Spec: architecture/10-api-and-data-contracts.md
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { formatInt64Str, parseInt64Str } from "@signal/contracts";
import { SIG_PROFILES, SIG_THEMES } from "./index";

describe("@signal/design-system & bigint contract parity", () => {
  it("exposes Obsidian, High-Contrast, and Okabe-Ito CVD themes", () => {
    expect(SIG_THEMES.map((t) => t.id)).toEqual(["dark", "high-contrast", "okabe-ito"]);
    expect(SIG_PROFILES).toEqual(["EXPLORER", "MAKER", "PRO"]);
  });

  it("preserves exact >2^53-1 64-bit SI integer precision on golden fixtures", () => {
    const here = dirname(fileURLToPath(import.meta.url));
    const fxPath = resolve(here, "../../../../contracts/fixtures/graph_mutation_tx.fixture.json");
    const raw = JSON.parse(readFileSync(fxPath, "utf8"));
    const xNm = parseInt64Str(raw.operations[0].position_nm.x_nm);
    const yNm = parseInt64Str(raw.operations[0].position_nm.y_nm);
    expect(xNm).toBe(9007199254740993n);
    expect(yNm).toBe(-9007199254740995n);
    expect(formatInt64Str(xNm)).toBe("9007199254740993");
  });
});
