// Module: web/libs/schematic/src/lib/pinswap-equations/pinswap-equations.logic.ts — Pin Swap, Spatial Reannotate & Safe Equation Solver
// Invariants: Zero eval(); pure recursive-descent parser for parametric equations; deterministic bigint spatial RefDes sort.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type {
  EquationEvalResult,
  PinSwapCandidate,
  PinswapEquationsVm,
  RefdesRemapResult,
  SpatialComponentAnchor,
  SpatialSweepDirection,
} from "./pinswap-equations.model";

export const DEFAULT_SWAP_CANDIDATES: readonly PinSwapCandidate[] = [
  { id: "sw-1", entityId: "U5_FPGA_BANK34", fromPin: 14, toPin: 19, netName: "ADC_SDO_A", wirelengthSavedNm: 6_400_000n },
  { id: "sw-2", entityId: "U5_FPGA_BANK34", fromPin: 22, toPin: 15, netName: "ADC_SDO_B", wirelengthSavedNm: 5_100_000n },
  { id: "sw-3", entityId: "U8_QUAD_OPAMP", fromPin: 1, toPin: 7, netName: "ISENSE_FILT_U", wirelengthSavedNm: 3_850_000n },
];

export const DEFAULT_SPATIAL_ANCHORS: readonly SpatialComponentAnchor[] = [
  { entityId: "e1", prefix: "R", oldRefdes: "R42", sheetIndex: 1, xNm: 30_000_000n, yNm: 10_000_000n },
  { entityId: "e2", prefix: "R", oldRefdes: "R9", sheetIndex: 1, xNm: 10_000_000n, yNm: 25_000_000n },
  { entityId: "e3", prefix: "R", oldRefdes: "R17", sheetIndex: 1, xNm: 10_000_000n, yNm: 10_000_000n },
  { entityId: "e4", prefix: "U", oldRefdes: "U9", sheetIndex: 2, xNm: 15_000_000n, yNm: 12_000_000n },
];

const SWAP_COLUMNS: readonly SigGridColumn[] = [
  { key: "entityId", header: "Component", mono: true },
  { key: "pins", header: "Pin Swap", mono: true },
  { key: "netName", header: "Signal Net", mono: true },
  { key: "saved", header: "Saved Length", mono: true },
];

const REANNOTATE_COLUMNS: readonly SigGridColumn[] = [
  { key: "oldRefdes", header: "Old RefDes", mono: true },
  { key: "newRefdes", header: "New RefDes", mono: true },
  { key: "coords", header: "Anchor (nm)", mono: true },
];

function compareBigInt(a: bigint, b: bigint): number {
  return a < b ? -1 : a > b ? 1 : 0;
}

export function reannotateSpatialEntities(
  anchors: readonly SpatialComponentAnchor[],
  direction: SpatialSweepDirection,
  sheetBaseStep = 100,
): readonly RefdesRemapResult[] {
  const sorted = anchors.slice().sort((a, b) => {
    if (a.sheetIndex !== b.sheetIndex) return a.sheetIndex - b.sheetIndex;
    if (direction === "LEFT_TO_RIGHT_TOP_DOWN") {
      return compareBigInt(a.yNm, b.yNm) || compareBigInt(a.xNm, b.xNm);
    }
    if (direction === "TOP_DOWN_LEFT_TO_RIGHT") {
      return compareBigInt(a.xNm, b.xNm) || compareBigInt(a.yNm, b.yNm);
    }
    return compareBigInt(a.yNm, b.yNm) || compareBigInt(a.xNm, b.xNm);
  });

  const counters = new Map<string, number>();
  return sorted.map((item) => {
    const key = direction === "HIERARCHICAL_SHEET_BLOCK" ? `${item.prefix}:${item.sheetIndex}` : item.prefix;
    const prev = counters.get(key) ?? (direction === "HIERARCHICAL_SHEET_BLOCK" ? item.sheetIndex * sheetBaseStep : 0);
    const next = prev + 1;
    counters.set(key, next);
    return {
      entityId: item.entityId,
      oldRefdes: item.oldRefdes,
      newRefdes: `${item.prefix}${next}`,
    };
  });
}

export function evaluateParametricEquation(
  expression: string,
  variables: Readonly<Record<string, number>>,
): EquationEvalResult {
  const tokens = expression.match(/[A-Za-z_][A-Za-z0-9_]*|\d+(?:\.\d+)?|[+\-*/()]/g);
  if (!tokens || tokens.length === 0) {
    return { valid: false, siValue: 0n, summaryText: "Enter arithmetic expression", tone: "warn" };
  }
  let pos = 0;

  function parsePrimary(): number {
    const tok = tokens?.[pos];
    if (tok === undefined) throw new Error("Unexpected end");
    if (tok === "(") {
      pos += 1;
      const inner = parseAddSub();
      if (tokens?.[pos] !== ")") throw new Error("Unclosed '('");
      pos += 1;
      return inner;
    }
    pos += 1;
    if (/^\d/.test(tok)) return Number(tok);
    const val = variables[tok];
    if (val === undefined) throw new Error(`Unknown var '${tok}'`);
    return val;
  }

  function parseMulDiv(): number {
    let left = parsePrimary();
    while (pos < (tokens?.length ?? 0) && (tokens?.[pos] === "*" || tokens?.[pos] === "/")) {
      const op = tokens[pos++];
      const right = parsePrimary();
      if (op === "/" && right === 0) throw new Error("Divide by zero");
      left = op === "*" ? left * right : left / right;
    }
    return left;
  }

  function parseAddSub(): number {
    let left = parseMulDiv();
    while (pos < (tokens?.length ?? 0) && (tokens?.[pos] === "+" || tokens?.[pos] === "-")) {
      const op = tokens[pos++];
      const right = parseMulDiv();
      left = op === "+" ? left + right : left - right;
    }
    return left;
  }

  try {
    const val = parseAddSub();
    if (pos !== tokens.length || !Number.isFinite(val)) {
      return { valid: false, siValue: 0n, summaryText: "Syntax error in equation", tone: "danger" };
    }
    const siValue = BigInt(Math.round(val));
    return {
      valid: true,
      siValue,
      summaryText: `R_TOP = ${siValue.toString(10)} mΩ (${(siValue / 1_000n).toString(10)} Ω)`,
      tone: "ok",
    };
  } catch (err) {
    const msg = err instanceof Error ? err.message : "Invalid expression";
    return { valid: false, siValue: 0n, summaryText: msg, tone: "danger" };
  }
}

export function buildPinswapEquationsVm(
  acceptedSwaps: boolean,
  direction: SpatialSweepDirection,
  equationExpr: string,
): PinswapEquationsVm {
  const totalSavedNm = DEFAULT_SWAP_CANDIDATES.reduce((acc, item) => acc + item.wirelengthSavedNm, 0n);
  const swapSavingsBadge = acceptedSwaps
    ? `Swaps Committed: ${(totalSavedNm / 1_000n).toString(10)} um Saved`
    : `Proposed Savings: ${(totalSavedNm / 1_000n).toString(10)} um (3 Swaps)`;
  const swapRows: SigGridRow[] = DEFAULT_SWAP_CANDIDATES.map((c) => ({
    id: c.id,
    entityId: c.entityId,
    pins: `Pin ${c.fromPin} <-> Pin ${c.toPin}`,
    netName: c.netName,
    saved: `${(c.wirelengthSavedNm / 1_000n).toString(10)} um`,
  }));

  const remaps = reannotateSpatialEntities(DEFAULT_SPATIAL_ANCHORS, direction, 100);
  const reannotateRows: SigGridRow[] = remaps.map((r) => {
    const anchor = DEFAULT_SPATIAL_ANCHORS.find((a) => a.entityId === r.entityId);
    return {
      id: r.entityId,
      oldRefdes: r.oldRefdes,
      newRefdes: r.newRefdes,
      coords: anchor ? `(${anchor.xNm.toString(10)}, ${anchor.yNm.toString(10)})` : "",
    };
  });

  const equationResult = evaluateParametricEquation(equationExpr, {
    r_bot_mohm: 10_000_000,
    v_out_mv: 5_000,
    v_ref_mv: 800,
  });

  return {
    swapSavingsBadge,
    swapColumns: SWAP_COLUMNS,
    swapRows,
    reannotateColumns: REANNOTATE_COLUMNS,
    reannotateRows,
    equationResult,
  };
}
