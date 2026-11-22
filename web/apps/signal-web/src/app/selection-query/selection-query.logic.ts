// Module: web/apps/signal-web/src/app/selection-query/selection-query.logic.ts — Structured CAD Predicate Evaluator
// Invariants: Parses `type:cap AND voltage_rating < 2*net_max_v` without eval(); supports numeric & field-scaled RHS.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";
import type { QueryableDesignEntity, SelectionQueryVm } from "./selection-query.model";

export const DEFAULT_QUERYABLE_ENTITIES: readonly QueryableDesignEntity[] = [
  { entityId: "C_IN_1", type: "cap", voltageRatingV: 25, netMaxV: 16, powerRatingMw: 0, netName: "VIN_16V" },
  { entityId: "C_LINK_HV", type: "cap", voltageRatingV: 1200, netMaxV: 800, powerRatingMw: 0, netName: "HV_DC_BUS_800V" },
  { entityId: "C_GATE_DEC", type: "cap", voltageRatingV: 50, netMaxV: 18, powerRatingMw: 0, netName: "VCC_GATE_18V" },
  { entityId: "R_SHUNT_U", type: "res", voltageRatingV: 200, netMaxV: 2, powerRatingMw: 2000, netName: "ISENSE_U" },
  { entityId: "Q1_SIC_HS", type: "mosfet", voltageRatingV: 1200, netMaxV: 800, powerRatingMw: 150000, netName: "HV_SW_PHASE_U" },
];

export const QUERY_PRESETS: readonly string[] = [
  "type:cap AND voltage_rating < 2*net_max_v",
  "type:mosfet AND net_max_v >= 400",
  "type:res AND power_rating >= 1000",
];

function resolveNumericField(entity: QueryableDesignEntity, field: string): number | null {
  if (field === "voltage_rating") return entity.voltageRatingV;
  if (field === "net_max_v") return entity.netMaxV;
  if (field === "power_rating") return entity.powerRatingMw;
  return null;
}

function resolveRhsValue(entity: QueryableDesignEntity, rhsRaw: string): number | null {
  const trimmed = rhsRaw.trim();
  if (/^-?\d+(?:\.\d+)?$/.test(trimmed)) {
    return Number(trimmed);
  }
  const scaled = /^(-?\d+(?:\.\d+)?)\s*\*\s*([a-z_]+)$/.exec(trimmed);
  if (scaled) {
    const factor = Number(scaled[1]);
    const fieldVal = resolveNumericField(entity, scaled[2]);
    return fieldVal !== null ? factor * fieldVal : null;
  }
  return resolveNumericField(entity, trimmed);
}

function evaluateClause(entity: QueryableDesignEntity, clause: string): boolean | null {
  const trimmed = clause.trim();
  const colonMatch = /^([a-z_]+):([A-Za-z0-9_]+)$/.exec(trimmed);
  if (colonMatch) {
    const key = colonMatch[1];
    const val = colonMatch[2].toLowerCase();
    if (key === "type") return entity.type === val;
    if (key === "net") return entity.netName.toLowerCase().includes(val);
    return null;
  }

  const cmpMatch = /^([a-z_]+)\s*(<=|>=|<|>|=)\s*(.+)$/.exec(trimmed);
  if (!cmpMatch) return null;
  const lhs = resolveNumericField(entity, cmpMatch[1]);
  const rhs = resolveRhsValue(entity, cmpMatch[3]);
  if (lhs === null || rhs === null) return null;

  const op = cmpMatch[2];
  if (op === "<") return lhs < rhs;
  if (op === "<=") return lhs <= rhs;
  if (op === ">") return lhs > rhs;
  if (op === ">=") return lhs >= rhs;
  return lhs === rhs;
}

export function evaluateSelectionQuery(
  queryText: string,
  entities: readonly QueryableDesignEntity[] = DEFAULT_QUERYABLE_ENTITIES,
): SelectionQueryVm {
  const trimmed = queryText.trim();
  if (!trimmed) {
    return {
      queryText,
      valid: false,
      matchedEntityIds: [],
      evaluatedCount: entities.length,
      summaryBadge: "Enter structured selection query",
      tone: "neutral",
      presets: QUERY_PRESETS,
    };
  }

  const andClauses = trimmed.split(/\s+AND\s+/i);
  const matched: string[] = [];
  for (const entity of entities) {
    let allTrue = true;
    for (const c of andClauses) {
      const verdict = evaluateClause(entity, c);
      if (verdict === null) {
        return {
          queryText,
          valid: false,
          matchedEntityIds: [],
          evaluatedCount: entities.length,
          summaryBadge: `Invalid query clause: '${c.trim()}'`,
          tone: "danger",
          presets: QUERY_PRESETS,
        };
      }
      if (!verdict) {
        allTrue = false;
      }
    }
    if (allTrue) matched.push(entity.entityId);
  }

  const tone: SigStatusTone = matched.length > 0 ? "warn" : "ok";
  return {
    queryText,
    valid: true,
    matchedEntityIds: matched,
    evaluatedCount: entities.length,
    summaryBadge: `Matched ${matched.length}/${entities.length}: [${matched.join(", ") || "none"}]`,
    tone,
    presets: QUERY_PRESETS,
  };
}
