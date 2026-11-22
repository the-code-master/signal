// Module: web/libs/specs/src/lib/specs-bom-workbench/specs-bom-workbench.logic.ts — Executable PRD & FEAT-077 Provenance Logic
// Invariants: Zero Angular decorators; blocks grey-market brokers lacking ECIA franchise lot signatures (FEAT-077).
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  BomDistributorOffer,
  BomMatrixLine,
  ExecutableRequirementItem,
  ProvenanceStatus,
  SpecsBomWorkbenchVm,
} from "./specs-bom-workbench.model";

export const SPECS_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "specs",
  title: "Executable PRD & System Requirements",
  subtitle: "Live requirement-to-simulation assertion links & FEAT-077 BOM Provenance",
  rustCrate: "signal-graph",
  subprojectRepo: "aurora-ev-inverter-prd",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Linked Assertions: 48/48", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Passing: 100%", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Peak Eff >= 99.1%", "FEAT-077 Chain-of-Custody"],
  gridTitle: "Executable Requirements",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "REQ-01", item: "Peak Inverter Efficiency >= 99.0%", metric: "99.18% PASS" },
    { id: "REQ-02", item: "Short-Circuit Turn-Off <= 1.5 us", metric: "1.12 us PASS" },
  ],
};

const AUTHORIZED_FRANCHISES = new Set(["DIGIKEY", "MOUSER", "ARROW", "AVNET", "TI_DIRECT", "WOLFSPEED_DIRECT"]);

export const DEFAULT_REQUIREMENTS: readonly ExecutableRequirementItem[] = [
  { reqId: "PRD-001", domain: "PRD", title: "Short-Circuit Desat Turn-Off", operator: "<=", limitSi: 1_500_000n, measuredSi: 1_120_000n, unitSuffix: "ps" },
  { reqId: "HFS-002", domain: "HFS", title: "HV DC-Link Peak Ripple", operator: "<=", limitSi: 8_000_000n, measuredSi: 5_400_000n, unitSuffix: "uV" },
  { reqId: "HFS-003", domain: "HFS", title: "Gate Isolation Withstand", operator: ">=", limitSi: 5_700_000_000n, measuredSi: 6_100_000_000n, unitSuffix: "uV" },
];

export const DEFAULT_BOM_LINES: readonly BomMatrixLine[] = [
  {
    lineId: "L1",
    refdesGroup: "Q1..Q6",
    mpn: "C3M0016120K",
    qtyPerBoard: 6,
    offers: [
      { distributorCode: "WOLFSPEED_DIRECT", unitPriceMicroUsd: 18_400_000n, stockQty: 4200, lotSignature: "lot_sig_ws_2026_c3m_8812" },
      { distributorCode: "SHENZHEN_SPOT_BROKER", unitPriceMicroUsd: 11_900_000n, stockQty: 900, lotSignature: "unverified_spot" },
    ],
  },
  {
    lineId: "L2",
    refdesGroup: "U1..U6",
    mpn: "UCC21750QDWR",
    qtyPerBoard: 6,
    offers: [
      { distributorCode: "DIGIKEY", unitPriceMicroUsd: 4_850_000n, stockQty: 12500, lotSignature: "lot_sig_dk_ti_2026_4410" },
      { distributorCode: "MOUSER", unitPriceMicroUsd: 4_920_000n, stockQty: 8300, lotSignature: "lot_sig_mou_ti_2026_1102" },
    ],
  },
];

const REQ_COLUMNS: readonly SigGridColumn[] = [
  { key: "reqId", header: "Req ID", mono: true },
  { key: "domain", header: "Spec Tier", mono: true },
  { key: "title", header: "Executable Requirement" },
  { key: "target", header: "SI Limit", mono: true },
  { key: "measured", header: "Sim Measured", mono: true },
  { key: "verdict", header: "Gate", mono: true },
];

const BOM_COLUMNS: readonly SigGridColumn[] = [
  { key: "refdes", header: "RefDes", mono: true },
  { key: "mpn", header: "Orderable MPN", mono: true },
  { key: "distributor", header: "Selected Source", mono: true },
  { key: "extPrice", header: "Line Cost", mono: true },
  { key: "provenance", header: "BomProvenanceBadge (FEAT-077)", mono: true },
];

export function verifyBomOfferProvenance(offer: BomDistributorOffer): ProvenanceStatus {
  const code = offer.distributorCode.trim().toUpperCase();
  if (!AUTHORIZED_FRANCHISES.has(code)) {
    return "GREY_MARKET_BLOCKED";
  }
  if (!offer.lotSignature.startsWith("lot_sig_") || offer.lotSignature.length < 20) {
    return "LOT_UNVERIFIED_WARN";
  }
  return "FRANCHISE_VERIFIED";
}

export function evaluateRequirementItem(item: ExecutableRequirementItem): boolean {
  return item.operator === "<=" ? item.measuredSi <= item.limitSi : item.measuredSi >= item.limitSi;
}

export function buildSpecsBomWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  allowBrokerCheapestFallback: boolean,
): SpecsBomWorkbenchVm {
  let passReqs = 0;
  const reqRows: SigGridRow[] = DEFAULT_REQUIREMENTS.map((r) => {
    const ok = evaluateRequirementItem(r);
    if (ok) passReqs += 1;
    return {
      id: r.reqId,
      reqId: r.reqId,
      domain: r.domain,
      title: r.title,
      target: `${r.operator} ${r.limitSi.toString(10)} ${r.unitSuffix}`,
      measured: `${r.measuredSi.toString(10)} ${r.unitSuffix}`,
      verdict: ok ? "PASS" : "FAIL",
    };
  });

  let blockedLines = 0;
  let totalMicroUsd = 0n;
  const bomRows: SigGridRow[] = DEFAULT_BOM_LINES.map((line) => {
    const chosen =
      allowBrokerCheapestFallback && line.offers.length > 1 ? line.offers[1] : line.offers[0];
    const prov = verifyBomOfferProvenance(chosen);
    if (prov !== "FRANCHISE_VERIFIED") blockedLines += 1;
    const extMicroUsd = chosen.unitPriceMicroUsd * BigInt(line.qtyPerBoard);
    totalMicroUsd += extMicroUsd;
    const usdWhole = extMicroUsd / 1_000_000n;
    const usdCents = ((extMicroUsd % 1_000_000n) / 10_000n).toString(10).padStart(2, "0");
    return {
      id: line.lineId,
      refdes: `${line.refdesGroup} (x${line.qtyPerBoard})`,
      mpn: line.mpn,
      distributor: `${chosen.distributorCode} [stk:${chosen.stockQty}]`,
      extPrice: `$${usdWhole.toString(10)}.${usdCents}`,
      provenance: prov,
    };
  });

  const reqAllOk = passReqs === DEFAULT_REQUIREMENTS.length;
  const reqGateTone: SigStatusTone = reqAllOk ? "ok" : "danger";
  const bomTone: SigStatusTone = blockedLines === 0 ? "ok" : "danger";
  const totalWhole = totalMicroUsd / 1_000_000n;
  const totalCents = ((totalMicroUsd % 1_000_000n) / 10_000n).toString(10).padStart(2, "0");

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    reqGateBadge: `PRD/HFS Gate: ${passReqs}/${DEFAULT_REQUIREMENTS.length} Assertions Passing`,
    reqGateTone,
    reqColumns: REQ_COLUMNS,
    reqRows,
    bomProvenanceBadge:
      blockedLines === 0
        ? "FEAT-077 Provenance: 100% ECIA Franchise Attested"
        : `FEAT-077 BLOCKED: ${blockedLines} Grey-Market / Unverified Line(s)`,
    bomProvenanceTone: bomTone,
    totalBoardCostText: `Board BOM: $${totalWhole.toString(10)}.${totalCents}`,
    bomColumns: BOM_COLUMNS,
    bomRows,
  };
}
