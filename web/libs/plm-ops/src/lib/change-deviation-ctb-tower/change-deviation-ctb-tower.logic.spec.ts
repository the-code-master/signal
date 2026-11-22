// Module: web/libs/plm-ops/src/lib/change-deviation-ctb-tower/change-deviation-ctb-tower.logic.spec.ts — Unit Tests
// Invariants: Verifies hard Deviation qty-exhaustion SFTP block, Part 11 ECN release promotion, and 3D work instructions.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import { describe, expect, it } from "vitest";
import {
  buildChangeDeviationCtbTowerVm,
  isDeviationReleasePermitted,
} from "./change-deviation-ctb-tower.logic";

describe("change-deviation-ctb-tower.logic (CU-P5-05)", () => {
  it("permits SFTP at 450/500 consumed (+50 lot) and hard-blocks when consumed reaches 480/500", () => {
    expect(isDeviationReleasePermitted(450n, 50n, 500n)).toBe(true);
    expect(isDeviationReleasePermitted(480n, 50n, 500n)).toBe(false);

    const blockedVm = buildChangeDeviationCtbTowerVm(true, 480n, 3);
    expect(blockedVm.deviationGateTone).toBe("danger");
    expect(blockedVm.workInstructionBadge).toContain("8.50 N·m");
  });
});
