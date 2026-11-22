// Module: web/libs/admin/src/lib/avl-where-used-console/avl-where-used-console.logic.spec.ts — FEAT-075 AVL Tests
// Invariants: Verifies cross-project blast radius instance totals and 1-click ECO draft status.
// Spec: architecture/18-universal-component-and-datasheet-repository.md
import { describe, expect, it } from "vitest";
import { buildAvlWhereUsedVm } from "./avl-where-used-console.logic";

describe("AvlWhereUsedConsole logic (FEAT-075)", () => {
  it("aggregates affected projects/placements and formats 1-click ECO draft", () => {
    const vm = buildAvlWhereUsedVm("TPS546D24ARVFR", "TPS546D24ARVFR");
    expect(vm.blastTone).toBe("danger");
    expect(vm.blastBadge).toContain("2 Projects / 10 Placements");
    expect(vm.ecoDraftBadge).toContain("TPS546D24SRVFR");
  });
});
