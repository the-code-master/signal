// Module: web/libs/schematic/src/lib/hierarchy-variants/hierarchy-variants.model.ts — FEAT-005..007 Portals, Repeat(N) & Variants Models
// Invariants: Portal anchors use ES2020 bigint nanometers (_nm); Repeat(N) capped <=64 channels.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";

export type AssemblyVariantId = "PROD_800V" | "DEV_BENCH_400V" | "COST_REDUCED";

export interface NetPortalDescriptor {
  readonly id: string;
  readonly netName: string;
  readonly sourceSheetId: string;
  readonly targetSheetId: string;
  readonly anchorXNm: bigint;
  readonly anchorYNm: bigint;
}

export interface MultiChannelRuleInput {
  readonly blockId: string;
  readonly repeatCount: number;
  readonly refdesBaseStep: number;
  readonly netTemplate: string;
}

export interface VariantBomItem {
  readonly entityId: string;
  readonly primaryMpn: string;
  readonly alternateMpn: string;
  readonly dnpVariants: readonly AssemblyVariantId[];
  readonly useAltInCostReduced: boolean;
}

export interface HierarchyVariantsVm {
  readonly portals: readonly NetPortalDescriptor[];
  readonly selectedPortalSummary: string;
  readonly channelBadge: string;
  readonly channelColumns: readonly SigGridColumn[];
  readonly channelRows: readonly SigGridRow[];
  readonly variantSummaryBadge: string;
  readonly variantColumns: readonly SigGridColumn[];
  readonly variantRows: readonly SigGridRow[];
}
