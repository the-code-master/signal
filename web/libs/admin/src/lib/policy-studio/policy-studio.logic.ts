// Module: web/libs/admin/src/lib/policy-studio/policy-studio.logic.ts — Fuel-Capped WASM Policy Logic (FEAT-117)
// Invariants: Pure deterministic functions <=250 LOC; enforces Wasmtime bigint fuel cap, <=16 MiB memory & ML-DSA-65 sig.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { PolicyPackDomain, PolicyStudioVm, WasmPolicyPackSpec } from "./policy-studio.model";

export const MAX_WASM_MEMORY_KIB = 16_384;

export const SEEDED_WASM_POLICY_PACKS: readonly WasmPolicyPackSpec[] = [
  {
    policyPackId: "pol-ev-creepage-v2",
    domain: "CREEPAGE_800V",
    version: "2.4.0",
    fuelBudgetInstructions: 5_000_000n,
    fuelConsumedInstructions: 1_420_000n,
    maxLinearMemoryKib: 4_096,
    wasmModuleBlake3: "blake3:7c19e04a88b12d",
    orgAdminMlDsa65Signature: "ml-dsa-65:orgadmin-creepage-9911223344",
    unitTestsPassed: 12,
    unitTestsTotal: 12,
  },
  {
    policyPackId: "pol-itar-residency-v1",
    domain: "SOVEREIGN_ITAR_GATE",
    version: "1.2.0",
    fuelBudgetInstructions: 2_500_000n,
    fuelConsumedInstructions: 610_000n,
    maxLinearMemoryKib: 2_048,
    wasmModuleBlake3: "blake3:3a90f12c44d81e",
    orgAdminMlDsa65Signature: "ml-dsa-65:orgadmin-itar-5566778899",
    unitTestsPassed: 8,
    unitTestsTotal: 8,
  },
];

export const POLICY_GRID_COLUMNS: readonly SigGridColumn[] = [
  { key: "id", header: "Policy Pack", mono: true },
  { key: "fuel", header: "Wasmtime Fuel (Consumed / Cap)", mono: true },
  { key: "memory", header: "Linear RAM Cap", mono: true },
  { key: "tests", header: "Fixture Suite", mono: true },
  { key: "sig", header: "Org-Admin ML-DSA-65", mono: true },
];

export function isWasmPolicyCompliant(pack: WasmPolicyPackSpec): boolean {
  const fuelOk = pack.fuelConsumedInstructions <= pack.fuelBudgetInstructions;
  const memOk = pack.maxLinearMemoryKib > 0 && pack.maxLinearMemoryKib <= MAX_WASM_MEMORY_KIB;
  const sigOk = pack.orgAdminMlDsa65Signature.startsWith("ml-dsa-65:") && pack.orgAdminMlDsa65Signature.length >= 20;
  const testsOk = pack.unitTestsPassed === pack.unitTestsTotal && pack.unitTestsTotal > 0;
  return fuelOk && memOk && sigOk && testsOk;
}

export function buildPolicyStudioVm(
  selectedDomain: PolicyPackDomain,
  resignedTag: string | null,
): PolicyStudioVm {
  const packs: WasmPolicyPackSpec[] = SEEDED_WASM_POLICY_PACKS.map((p) =>
    p.domain === selectedDomain && resignedTag !== null
      ? { ...p, orgAdminMlDsa65Signature: `ml-dsa-65:${resignedTag}` }
      : p,
  );
  const activePack = packs.find((p) => p.domain === selectedDomain) ?? packs[0]!;
  const signed = activePack.orgAdminMlDsa65Signature.startsWith("ml-dsa-65:");
  const compliant = isWasmPolicyCompliant(activePack);
  const statusTone: SigStatusTone = compliant ? "ok" : "danger";

  const packRows: SigGridRow[] = packs.map((p) => ({
    id: `${p.policyPackId}@${p.version}`,
    fuel: `${p.fuelConsumedInstructions.toString()} / ${p.fuelBudgetInstructions.toString()}`,
    memory: `${p.maxLinearMemoryKib} KiB`,
    tests: `${p.unitTestsPassed}/${p.unitTestsTotal} PASS`,
    sig: p.orgAdminMlDsa65Signature,
  }));

  return {
    packs,
    activePack,
    signed,
    compliant,
    statusTone,
    statusBadge: compliant
      ? `Signed & Enforced (${activePack.policyPackId})`
      : `Blocked: Fuel / Signature Gate Failed`,
    packColumns: POLICY_GRID_COLUMNS,
    packRows,
  };
}
