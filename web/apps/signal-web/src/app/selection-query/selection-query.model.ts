// Module: web/apps/signal-web/src/app/selection-query/selection-query.model.ts — FEAT-099 Structured Selection Query Models
// Invariants: Zero eval(); evaluates conjunctive/relational CAD entity queries deterministically.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";

export interface QueryableDesignEntity {
  readonly entityId: string;
  readonly type: "cap" | "res" | "ind" | "mosfet" | "ic";
  readonly voltageRatingV: number;
  readonly netMaxV: number;
  readonly powerRatingMw: number;
  readonly netName: string;
}

export interface SelectionQueryVm {
  readonly queryText: string;
  readonly valid: boolean;
  readonly matchedEntityIds: readonly string[];
  readonly evaluatedCount: number;
  readonly summaryBadge: string;
  readonly tone: SigStatusTone;
  readonly presets: readonly string[];
}
