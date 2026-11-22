// Module: web/libs/ui-kit/src/si-input/sig-si-input.logic.ts — Pure SI Prefix Parser (m milli vs M/Meg Mega)
// Invariants: Rejects ambiguous lowercase `meg`; returns exact ES2020 bigint nano-units (F-075, F-144).
// Spec: architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigSiParseResult } from "./sig-si-input.model";

const NANO_MULTIPLIERS: Readonly<Record<string, bigint>> = {
  n: 1n,
  u: 1_000n,
  "µ": 1_000n,
  m: 1_000_000n,
  "": 1_000_000_000n,
  k: 1_000_000_000_000n,
  M: 1_000_000_000_000_000n,
  Meg: 1_000_000_000_000_000n,
  G: 1_000_000_000_000_000_000n,
};

export function parseSiInputToNano(raw: string, unitSymbol: string): SigSiParseResult {
  const trimmed = raw.trim();
  const match = /^(-?\d+)\s*([A-Za-zµ]*)$/.exec(trimmed);
  if (!match) {
    return { valid: false, nanoUnits: 0n, resolvedHint: "Enter integer + SI prefix (n, u, m, k, M/Meg, G)" };
  }
  const intPart = BigInt(match[1]);
  let suffix = match[2];
  if (unitSymbol && suffix.endsWith(unitSymbol)) {
    suffix = suffix.slice(0, -unitSymbol.length);
  }
  if (suffix === "meg") {
    return { valid: false, nanoUnits: 0n, resolvedHint: "Ambiguous `meg`: use `m` (milli) or `Meg`/`M` (Mega)" };
  }
  const mult = NANO_MULTIPLIERS[suffix];
  if (mult === undefined) {
    return { valid: false, nanoUnits: 0n, resolvedHint: `Unknown SI prefix '${suffix}'` };
  }
  const nanoUnits = intPart * mult;
  const label = suffix === "m" ? "milli (1e-3)" : suffix === "M" || suffix === "Meg" ? "Mega (1e6)" : suffix || "base";
  return { valid: true, nanoUnits, resolvedHint: `${nanoUnits.toString(10)} n${unitSymbol} [${label}]` };
}
