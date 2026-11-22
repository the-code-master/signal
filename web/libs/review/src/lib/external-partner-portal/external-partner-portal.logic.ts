// Module: web/libs/review/src/lib/external-partner-portal/external-partner-portal.logic.ts — FEAT-119 Stripped Guest Filter
// Invariants: Redacts 100% of internalCostUsd, internalNdaNote & hidden NDA entities before guest projection; enforces TTL.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  ExternalPartnerPortalVm,
  InternalReviewEntity,
  StrippedPartnerEntity,
} from "./external-partner-portal.model";

export const DEFAULT_INTERNAL_ENTITIES: readonly InternalReviewEntity[] = [
  { entityId: "e1", publicRefdes: "J1_HV_CONN", packageOutline: "TE_HVP800_2P", internalCostUsd: "$42.10", internalNdaNote: "Tier-1 OEM custom rebate", exposeToPartner: true },
  { entityId: "e2", publicRefdes: "HS1_COLDPLATE", packageOutline: "CAST_AL_240X160MM", internalCostUsd: "$68.00", internalNdaNote: "Friction-stir supplier NDA", exposeToPartner: true },
  { entityId: "e3", publicRefdes: "U9_CRYPTO_SE", packageOutline: "VQFN-32", internalCostUsd: "$4.85", internalNdaNote: "Key provisioning ROM rev", exposeToPartner: false },
];

const PARTNER_COLUMNS: readonly SigGridColumn[] = [
  { key: "refdes", header: "Visible RefDes", mono: true },
  { key: "outline", header: "Mechanical Keepout / Envelope", mono: true },
  { key: "redaction", header: "DLP Filter Status", mono: true },
];

export function stripSensitiveFieldsForExternalPartner(
  entities: readonly InternalReviewEntity[],
): readonly StrippedPartnerEntity[] {
  return entities
    .filter((e) => e.exposeToPartner)
    .map((e) => ({
      entityId: e.entityId,
      publicRefdes: e.publicRefdes,
      packageOutline: e.packageOutline,
    }));
}

export function buildExternalPartnerPortalVm(
  nowUtcMs: bigint,
  expiresAtUtcMs: bigint,
  entities: readonly InternalReviewEntity[] = DEFAULT_INTERNAL_ENTITIES,
): ExternalPartnerPortalVm {
  const expired = nowUtcMs >= expiresAtUtcMs;
  const ttlRemainingSec = expired ? 0n : (expiresAtUtcMs - nowUtcMs) / 1_000n;
  const visible = expired ? [] : stripSensitiveFieldsForExternalPartner(entities);
  const strippedCount = entities.length - visible.length;
  const portalTone: SigStatusTone = expired ? "danger" : "ok";
  const portalBadge = expired
    ? "Partner Guest Link EXPIRED (Viewport Blackout)"
    : `Zero-Leak Partner View Active (TTL ${ttlRemainingSec.toString(10)} s • Tardos Watermarked)`;

  const rows: SigGridRow[] = visible.map((v) => ({
    id: v.entityId,
    refdes: v.publicRefdes,
    outline: v.packageOutline,
    redaction: "Cost/NDA Notes Stripped",
  }));

  return {
    partnerOrg: "enclosure-vendor-guest",
    expired,
    ttlRemainingSec,
    portalBadge,
    portalTone,
    strippedCount,
    columns: PARTNER_COLUMNS,
    rows,
  };
}
