// Module: web/apps/signal-web/src/app/omni-palette/omni-palette.logic.ts — Deterministic CLI Command Parser & Fuzzy Scorer
// Invariants: Parses place, connect, route, sim, goto commands deterministically; ranks workbench jumps without external libs.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { WorkbenchNavLink } from "../shell/ide-shell.model";
import type { OmniJumpCandidate, OmniPaletteVm, ParsedOmniCommand } from "./omni-palette.model";

export const OMNI_QUICK_EXAMPLES: readonly string[] = [
  "place R 10k at 1250000,2500000",
  "connect U1.3 HV_DC_BUS_800V",
  "route HV_DC_BUS_800V width 250000",
  "sim tran 50u",
  "goto sim-viewer",
];

export function parseOmniCliCommand(rawLine: string): ParsedOmniCommand {
  const trimmed = rawLine.trim();
  if (!trimmed) {
    return { valid: false, verb: null, normalizedAction: "Type CLI command or workbench name", targetWorkbenchSlug: null };
  }
  const parts = trimmed.split(/\s+/);
  const head = parts[0].toLowerCase();

  if (head === "place" && parts.length >= 2) {
    const partKind = parts[1];
    const val = parts[2] ?? "default";
    const atIdx = parts.findIndex((p) => p.toLowerCase() === "at");
    if (atIdx >= 0 && parts[atIdx + 1]?.includes(",")) {
      const [xs, ys] = parts[atIdx + 1].split(",");
      if (/^-?\d+$/.test(xs) && /^-?\d+$/.test(ys)) {
        const xNm = BigInt(xs);
        const yNm = BigInt(ys);
        return {
          valid: true,
          verb: "PLACE",
          normalizedAction: `PLACE ${partKind} (${val}) @ (${xNm.toString(10)}, ${yNm.toString(10)}) nm`,
          targetWorkbenchSlug: null,
          xNm,
          yNm,
        };
      }
    }
    return {
      valid: true,
      verb: "PLACE",
      normalizedAction: `PLACE ${partKind} (${val}) @ cursor`,
      targetWorkbenchSlug: null,
    };
  }

  if (head === "connect" && parts.length >= 3) {
    return {
      valid: true,
      verb: "CONNECT",
      normalizedAction: `CONNECT ${parts[1]} -> ${parts[2]}`,
      targetWorkbenchSlug: null,
    };
  }

  if (head === "route" && parts.length >= 2) {
    const widthIdx = parts.findIndex((p) => p.toLowerCase() === "width");
    const widthNm = widthIdx >= 0 && /^\d+$/.test(parts[widthIdx + 1] ?? "") ? `${parts[widthIdx + 1]} nm` : "rule-default";
    return {
      valid: true,
      verb: "ROUTE",
      normalizedAction: `ROUTE net ${parts[1]} (width=${widthNm})`,
      targetWorkbenchSlug: null,
    };
  }

  if (head === "sim" && parts.length >= 2) {
    return {
      valid: true,
      verb: "SIM",
      normalizedAction: `SIM ${parts[1].toUpperCase()} ${parts[2] ?? ""}`.trim(),
      targetWorkbenchSlug: "sim-viewer",
    };
  }

  if (head === "goto" && parts.length >= 2) {
    const slug = parts[1].toLowerCase();
    return {
      valid: true,
      verb: "GOTO",
      normalizedAction: `GOTO workbench '${slug}'`,
      targetWorkbenchSlug: slug,
    };
  }

  return {
    valid: false,
    verb: null,
    normalizedAction: "Fuzzy workbench search mode (or use place/connect/route/sim/goto)",
    targetWorkbenchSlug: null,
  };
}

export function rankFuzzyWorkbenchJumps(
  query: string,
  navLinks: readonly WorkbenchNavLink[],
): readonly OmniJumpCandidate[] {
  const q = query.trim().toLowerCase().replace(/^goto\s+/, "");
  if (!q) {
    return navLinks.slice(0, 6).map((n) => ({ slug: n.slug, label: n.label, score: 1 }));
  }
  const scored: OmniJumpCandidate[] = [];
  for (const item of navLinks) {
    const hay = `${item.slug} ${item.label}`.toLowerCase();
    if (item.slug === q) {
      scored.push({ slug: item.slug, label: item.label, score: 100 });
    } else if (item.slug.startsWith(q)) {
      scored.push({ slug: item.slug, label: item.label, score: 80 });
    } else if (hay.includes(q)) {
      scored.push({ slug: item.slug, label: item.label, score: 60 });
    } else {
      let qi = 0;
      for (let i = 0; i < hay.length && qi < q.length; i += 1) {
        if (hay[i] === q[qi]) qi += 1;
      }
      if (qi === q.length) {
        scored.push({ slug: item.slug, label: item.label, score: 30 });
      }
    }
  }
  return scored.sort((a, b) => b.score - a.score).slice(0, 6);
}

export function buildOmniPaletteVm(
  open: boolean,
  rawInput: string,
  navLinks: readonly WorkbenchNavLink[],
): OmniPaletteVm {
  const parsed = parseOmniCliCommand(rawInput);
  return {
    open,
    rawInput,
    parsed,
    statusTone: parsed.valid ? "ok" : "neutral",
    jumpCandidates: rankFuzzyWorkbenchJumps(rawInput, navLinks),
    quickExamples: OMNI_QUICK_EXAMPLES,
  };
}
