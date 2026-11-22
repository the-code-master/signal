// Module: web/libs/admin/src/lib/admin-workbench/admin-workbench.logic.spec.ts — Admin Posture & Access Review Tests
// Invariants: Verifies vcs-auditor posture improvement from 85/100 to 100/100 on expired contractor sweep.
// Spec: architecture/08-identity-enterprise-and-project-model.md
import { describe, expect, it } from "vitest";
import { ADMIN_WORKBENCH_DESCRIPTOR, buildAdminWorkbenchVm } from "./admin-workbench.logic";

describe("buildAdminWorkbenchVm", () => {
  it("detects expired contractor grants and restores 100/100 posture after sweep", () => {
    const before = buildAdminWorkbenchVm(ADMIN_WORKBENCH_DESCRIPTOR, false);
    expect(before.postureBadge).toContain("85/100");

    const after = buildAdminWorkbenchVm(ADMIN_WORKBENCH_DESCRIPTOR, true);
    expect(after.postureBadge).toContain("100/100");
    expect(after.postureTone).toBe("ok");
  });
});
