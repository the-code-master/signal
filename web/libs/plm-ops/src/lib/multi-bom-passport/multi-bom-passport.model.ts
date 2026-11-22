// Module: web/libs/plm-ops/src/lib/multi-bom-passport/multi-bom-passport.model.ts — eBOM/mBOM/sBOM & DPP Models
// Invariants: Unit costs (_micros_usd) and embodied carbon (_mg_co2e) stay in ES2020 bigint across BOM views.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/plm_ops.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type PlmBomViewMode = "EBOM" | "MBOM" | "SBOM";

export interface PlmBomItemRow {
  readonly lineId: string;
  readonly orderableMpn: string;
  readonly domainOrigin: "ECAD" | "MCAD" | "CAM_PRINT" | "PACKAGING_SERVICE";
  readonly qtyPerUnit: bigint;
  readonly unitCostMicrosUsd: bigint;
  readonly embodiedCo2eMg: bigint;
  readonly rohsReachPfasClean: boolean;
  readonly workstationStepId: string;
  readonly fieldSpareReplaceable: boolean;
}

export interface MultiBomPassportVm {
  readonly viewMode: PlmBomViewMode;
  readonly viewModes: readonly PlmBomViewMode[];
  readonly totalCostMicrosUsd: bigint;
  readonly totalEmbodiedCo2eMg: bigint;
  readonly dppCompliant: boolean;
  readonly rollupBadge: string;
  readonly dppBadge: string;
  readonly dppTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
