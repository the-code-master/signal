// Module: web/libs/specs/src/lib/specs-bom-workbench/specs-bom-workbench.model.ts — Executable PRD & FEAT-077 BOM Provenance Models
// Invariants: SI thresholds (_ps, _uv) and unit prices (_micro_usd) stored strictly as ES2020 bigint.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type ProvenanceStatus = "FRANCHISE_VERIFIED" | "LOT_UNVERIFIED_WARN" | "GREY_MARKET_BLOCKED";

export interface ExecutableRequirementItem {
  readonly reqId: string;
  readonly domain: "PRD" | "HFS";
  readonly title: string;
  readonly operator: ">=" | "<=";
  readonly limitSi: bigint;
  readonly measuredSi: bigint;
  readonly unitSuffix: string;
}

export interface BomDistributorOffer {
  readonly distributorCode: string;
  readonly unitPriceMicroUsd: bigint;
  readonly stockQty: number;
  readonly lotSignature: string;
}

export interface BomMatrixLine {
  readonly lineId: string;
  readonly refdesGroup: string;
  readonly mpn: string;
  readonly qtyPerBoard: number;
  readonly offers: readonly BomDistributorOffer[];
}

export interface SpecsBomWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly reqGateBadge: string;
  readonly reqGateTone: SigStatusTone;
  readonly reqColumns: readonly SigGridColumn[];
  readonly reqRows: readonly SigGridRow[];
  readonly bomProvenanceBadge: string;
  readonly bomProvenanceTone: SigStatusTone;
  readonly totalBoardCostText: string;
  readonly bomColumns: readonly SigGridColumn[];
  readonly bomRows: readonly SigGridRow[];
}
