// Module: web/libs/admin/src/lib/compliance-center/compliance-center.logic.spec.ts — Unit Tests (CU-P8-04)
// Invariants: Verifies 5/5 compliance frameworks and proves GDPR user_pii_salt shredding never mutates WORM Merkle tip.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
import { describe, expect, it } from "vitest";
import { IMMUTABLE_WORM_TIP_HASH, buildComplianceCenterVm } from "./compliance-center.logic";

describe("compliance-center.logic", () => {
  it("attests SOC2/ISO27001/ITAR/GDPR/21CFR11 and shreds user_pii_salt without mutating WORM Merkle chain", () => {
    const activeVm = buildComplianceCenterVm(false);
    expect(activeVm.frameworks.length).toBe(5);
    expect(activeVm.gdprShred.actorHmacUnlinkable).toBe(false);

    const shreddedVm = buildComplianceCenterVm(true);
    expect(shreddedVm.gdprShred.actorHmacUnlinkable).toBe(true);
    expect(shreddedVm.gdprShred.wormChainUnmutated).toBe(true);
    expect(shreddedVm.gdprShred.wormMerkleTipHashAfter).toBe(IMMUTABLE_WORM_TIP_HASH);
  });
});
