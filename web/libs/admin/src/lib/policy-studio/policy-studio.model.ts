// Module: web/libs/admin/src/lib/policy-studio/policy-studio.model.ts — WASM Policy-as-Code Models (FEAT-117)
// Invariants: Pure readonly TypeScript types; 64-bit Wasmtime fuel_budget_instructions uses bigint ONLY.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type PolicyPackDomain = "CREEPAGE_800V" | "NET_NAMING_ISO" | "STACKUP_IMPEDANCE" | "SOVEREIGN_ITAR_GATE";

export interface WasmPolicyPackSpec {
  readonly policyPackId: string;
  readonly domain: PolicyPackDomain;
  readonly version: string;
  readonly fuelBudgetInstructions: bigint;
  readonly fuelConsumedInstructions: bigint;
  readonly maxLinearMemoryKib: number;
  readonly wasmModuleBlake3: string;
  readonly orgAdminMlDsa65Signature: string;
  readonly unitTestsPassed: number;
  readonly unitTestsTotal: number;
}

export interface PolicyStudioVm {
  readonly packs: readonly WasmPolicyPackSpec[];
  readonly activePack: WasmPolicyPackSpec;
  readonly signed: boolean;
  readonly compliant: boolean;
  readonly statusTone: SigStatusTone;
  readonly statusBadge: string;
  readonly packColumns: readonly SigGridColumn[];
  readonly packRows: readonly SigGridRow[];
}
