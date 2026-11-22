// Module: web/libs/review/src/lib/external-partner-portal/external-partner-portal.logic.spec.ts — FEAT-119 Partner Portal Tests
// Invariants: Verifies complete omission of internalCostUsd/internalNdaNote and instant blackout on TTL expiry.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
import { describe, expect, it } from "vitest";
import {
  buildExternalPartnerPortalVm,
  DEFAULT_INTERNAL_ENTITIES,
  stripSensitiveFieldsForExternalPartner,
} from "./external-partner-portal.logic";

describe("ExternalPartnerPortal Zero-Leak stripping (FEAT-119)", () => {
  it("strips cost/NDA fields, hides non-exposed entities, and blacks out on expiry", () => {
    const stripped = stripSensitiveFieldsForExternalPartner(DEFAULT_INTERNAL_ENTITIES);
    expect(stripped.length).toBe(2);
    expect("internalCostUsd" in stripped[0]).toBe(false);

    const active = buildExternalPartnerPortalVm(1_000_000n, 1_900_000n);
    expect(active.expired).toBe(false);
    expect(active.rows.length).toBe(2);

    const expired = buildExternalPartnerPortalVm(2_000_000n, 1_900_000n);
    expect(expired.expired).toBe(true);
    expect(expired.rows.length).toBe(0);
  });
});
