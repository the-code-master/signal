// Module: web/libs/specs/src/lib/specs-bom-workbench/specs-bom-workbench.logic.spec.ts — CU-P2-04 Specs & BOM Tests
// Invariants: Verifies bigint PRD/HFS assertion checks and FEAT-077 grey-market broker blocking.
// Spec: architecture/18-universal-component-and-datasheet-repository.md
import { describe, expect, it } from "vitest";
import {
  buildSpecsBomWorkbenchVm,
  SPECS_WORKBENCH_DESCRIPTOR,
  verifyBomOfferProvenance,
} from "./specs-bom-workbench.logic";

describe("SpecsBomWorkbench Executable PRD & FEAT-077 Provenance", () => {
  it("verifies franchise lot signatures and blocks grey-market brokers", () => {
    expect(
      verifyBomOfferProvenance({
        distributorCode: "DIGIKEY",
        unitPriceMicroUsd: 4_850_000n,
        stockQty: 1000,
        lotSignature: "lot_sig_dk_ti_2026_4410",
      }),
    ).toBe("FRANCHISE_VERIFIED");

    expect(
      verifyBomOfferProvenance({
        distributorCode: "SHENZHEN_SPOT_BROKER",
        unitPriceMicroUsd: 2_000_000n,
        stockQty: 500,
        lotSignature: "lot_sig_fake_broker_00001",
      }),
    ).toBe("GREY_MARKET_BLOCKED");

    const franchiseVm = buildSpecsBomWorkbenchVm(SPECS_WORKBENCH_DESCRIPTOR, false);
    expect(franchiseVm.bomProvenanceTone).toBe("ok");
    expect(franchiseVm.reqGateTone).toBe("ok");

    const brokerVm = buildSpecsBomWorkbenchVm(SPECS_WORKBENCH_DESCRIPTOR, true);
    expect(brokerVm.bomProvenanceTone).toBe("danger");
    expect(brokerVm.bomProvenanceBadge).toContain("BLOCKED");
  });
});
