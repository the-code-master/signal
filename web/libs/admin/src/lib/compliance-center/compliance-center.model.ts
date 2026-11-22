// Module: web/libs/admin/src/lib/compliance-center/compliance-center.model.ts — Compliance & Shredder Models (CU-P8-04)
// Invariants: Pure readonly TypeScript types; 64-bit WORM sequence & _utc_ms timestamps use bigint ONLY.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/15-application-security-and-project-isolation.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type ComplianceFrameworkId =
  | "SOC2_TYPE_II"
  | "ISO_27001"
  | "ITAR_EAR_SOVEREIGN"
  | "GDPR_ARTICLE_17"
  | "FDA_21_CFR_PART_11";

export interface ComplianceFrameworkStatus {
  readonly framework: ComplianceFrameworkId;
  readonly controlSummary: string;
  readonly evidenceDigest: string;
  readonly verifiedAtUtcMs: bigint;
  readonly passing: boolean;
}

export interface GdprCryptoShredVerification {
  readonly subjectUserId: string;
  readonly saltShredded: boolean;
  readonly wormOutboxSeq: bigint;
  readonly wormMerkleTipHashBefore: string;
  readonly wormMerkleTipHashAfter: string;
  readonly wormChainUnmutated: boolean;
  readonly actorHmacUnlinkable: boolean;
}

export interface ComplianceCenterVm {
  readonly frameworks: readonly ComplianceFrameworkStatus[];
  readonly gdprShred: GdprCryptoShredVerification;
  readonly overallTone: SigStatusTone;
  readonly overallBadge: string;
  readonly shredBadge: string;
  readonly matrixColumns: readonly SigGridColumn[];
  readonly matrixRows: readonly SigGridRow[];
}
