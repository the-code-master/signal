// Module: web/libs/review/src/lib/external-partner-portal/external-partner-portal.model.ts — FEAT-119 Partner Portal Models
// Invariants: Strips BOM pricing, internal NDA notes & hidden sub-sheets; TTL timestamps in ES2020 bigint (_ms).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface InternalReviewEntity {
  readonly entityId: string;
  readonly publicRefdes: string;
  readonly packageOutline: string;
  readonly internalCostUsd: string;
  readonly internalNdaNote: string;
  readonly exposeToPartner: boolean;
}

export interface StrippedPartnerEntity {
  readonly entityId: string;
  readonly publicRefdes: string;
  readonly packageOutline: string;
}

export interface ExternalPartnerPortalVm {
  readonly partnerOrg: string;
  readonly expired: boolean;
  readonly ttlRemainingSec: bigint;
  readonly portalBadge: string;
  readonly portalTone: SigStatusTone;
  readonly strippedCount: number;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
