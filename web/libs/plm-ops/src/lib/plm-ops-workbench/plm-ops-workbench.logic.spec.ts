// Module: web/libs/plm-ops/src/lib/plm-ops-workbench/plm-ops-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies ERP/MES connector selection and signal-plm engine badge.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  PLM_OPS_WORKBENCH_DESCRIPTOR,
  buildPlmOpsWorkbenchVm,
} from "./plm-ops-workbench.logic";

describe("plm-ops-workbench.logic (CU-P5-05)", () => {
  it("builds PLM Ops workbench VM with HMAC-signed ERP/MES sync badge", () => {
    const vm = buildPlmOpsWorkbenchVm(PLM_OPS_WORKBENCH_DESCRIPTOR, "SAP_S4HANA");
    expect(vm.erpSyncBadge).toContain("SAP_S4HANA");
    expect(vm.engineBadge).toContain("signal-plm");
  });
});
