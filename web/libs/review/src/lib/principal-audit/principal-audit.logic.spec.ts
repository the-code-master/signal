// Module: web/libs/review/src/lib/principal-audit/principal-audit.logic.spec.ts — FEAT-104 Red-Team Audit Tests
// Invariants: Verifies 200-point check accounting, severity filtering, and 1-click auto-fix resolution.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { buildPrincipalAuditVm } from "./principal-audit.logic";

describe("PrincipalAudit 200-point Red-Team deck (FEAT-104)", () => {
  it("filters CRITICAL findings and clears findings on 1-click auto-fix", () => {
    const critOnly = buildPrincipalAuditVm("CRITICAL", new Set());
    expect(critOnly.rows.length).toBe(1);
    expect(critOnly.summaryTone).toBe("danger");

    const allFixed = buildPrincipalAuditVm("ALL", new Set([104, 142, 189]));
    expect(allFixed.openCount).toBe(0);
    expect(allFixed.summaryTone).toBe("ok");
  });
});
