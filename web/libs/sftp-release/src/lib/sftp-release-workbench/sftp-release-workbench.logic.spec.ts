// Module: web/libs/sftp-release/src/lib/sftp-release-workbench/sftp-release-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies <=60s Step-Up freshness and single-use TOTP burn replay rejection (F-076).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
import { describe, expect, it } from "vitest";
import {
  SFTP_RELEASE_WORKBENCH_DESCRIPTOR,
  buildSftpReleaseWorkbenchVm,
  evaluateStepUpEligibility,
} from "./sftp-release-workbench.logic";

describe("sftp-release-workbench.logic (CU-P4-05)", () => {
  it("accepts fresh <=60s WebAuthn/TOTP and blocks >60s stale assertions or replayed burned TOTP", () => {
    expect(evaluateStepUpEligibility("WEBAUTHN_FIDO2", 12n, false).eligible).toBe(true);
    expect(evaluateStepUpEligibility("WEBAUTHN_FIDO2", 75n, false).eligible).toBe(false);
    expect(evaluateStepUpEligibility("RFC6238_BURNED_TOTP", 10n, true).reason).toContain(
      "TOTP_NONCE_ALREADY_BURNED",
    );

    const vm = buildSftpReleaseWorkbenchVm(
      SFTP_RELEASE_WORKBENCH_DESCRIPTOR,
      "STEP_3_ATOMIC_RECEIPT",
      "FAB_GERBER_ODB_IPC2581",
      "WEBAUTHN_FIDO2",
      14n,
      false,
    );
    expect(vm.canDispatch).toBe(true);
    expect(vm.gateTone).toBe("ok");
  });
});
