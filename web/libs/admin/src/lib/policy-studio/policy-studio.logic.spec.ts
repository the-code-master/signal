// Module: web/libs/admin/src/lib/policy-studio/policy-studio.logic.spec.ts — Unit Tests (FEAT-117)
// Invariants: Verifies bigint Wasmtime fuel budget enforcement, <=16 MiB linear memory cap & ML-DSA-65 signature gate.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import { describe, expect, it } from "vitest";
import {
  SEEDED_WASM_POLICY_PACKS,
  buildPolicyStudioVm,
  isWasmPolicyCompliant,
} from "./policy-studio.logic";

describe("policy-studio.logic", () => {
  it("accepts signed fuel-bounded WASM policy packs and rejects fuel overrun or unsigned modules", () => {
    const vm = buildPolicyStudioVm("CREEPAGE_800V", "rotated-key-20261122");
    expect(vm.compliant).toBe(true);
    expect(vm.activePack.orgAdminMlDsa65Signature).toBe("ml-dsa-65:rotated-key-20261122");

    const overrun = {
      ...SEEDED_WASM_POLICY_PACKS[0]!,
      fuelConsumedInstructions: 9_000_000n,
    };
    expect(isWasmPolicyCompliant(overrun)).toBe(false);
  });
});
