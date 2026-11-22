// Module: web/libs/plm-ops/src/lib/multi-bom-passport/multi-bom-passport.logic.ts — Synchronized Multi-BOM & DPP Logic
// Invariants: Compiles eBOM, mBOM, and sBOM views in exact bigint _micros_usd / _mg_co2e with RoHS/REACH/PFAS check.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, contracts/proto/signal/v1/plm_ops.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  MultiBomPassportVm,
  PlmBomItemRow,
  PlmBomViewMode,
} from "./multi-bom-passport.model";

export const DEFAULT_MULTI_BOM_ITEMS: readonly PlmBomItemRow[] = [
  {
    lineId: "LN-SIC-01",
    orderableMpn: "C3M0016120K (6x SiC MOSFET)",
    domainOrigin: "ECAD",
    qtyPerUnit: 6n,
    unitCostMicrosUsd: 18_500_000n,
    embodiedCo2eMg: 145_000n,
    rohsReachPfasClean: true,
    workstationStepId: "WS-10-SMT-TOP",
    fieldSpareReplaceable: false,
  },
  {
    lineId: "LN-HSG-02",
    orderableMpn: "AUR-COLDPLATE-ALSI10MG",
    domainOrigin: "MCAD",
    qtyPerUnit: 1n,
    unitCostMicrosUsd: 34_000_000n,
    embodiedCo2eMg: 620_000n,
    rohsReachPfasClean: true,
    workstationStepId: "WS-40-CNC-MECH",
    fieldSpareReplaceable: true,
  },
  {
    lineId: "LN-INS-03",
    orderableMpn: "AUR-BUSBAR-INSULATOR-PA12",
    domainOrigin: "CAM_PRINT",
    qtyPerUnit: 2n,
    unitCostMicrosUsd: 4_800_000n,
    embodiedCo2eMg: 95_000n,
    rohsReachPfasClean: true,
    workstationStepId: "WS-30-SLS-CELL",
    fieldSpareReplaceable: true,
  },
  {
    lineId: "LN-GSK-04",
    orderableMpn: "AUR-IP67-EPDM-SEAL-KIT",
    domainOrigin: "PACKAGING_SERVICE",
    qtyPerUnit: 1n,
    unitCostMicrosUsd: 2_900_000n,
    embodiedCo2eMg: 38_000n,
    rohsReachPfasClean: true,
    workstationStepId: "WS-50-FINAL-TORQUE",
    fieldSpareReplaceable: true,
  },
];

const BOM_COLUMNS: readonly SigGridColumn[] = [
  { key: "mpn", header: "Synchronized Multi-Domain Line", mono: true },
  { key: "origin", header: "Domain / Step", mono: true },
  { key: "extCost", header: "Ext Cost", mono: true },
  { key: "co2", header: "Carbon", mono: true },
  { key: "dpp", header: "RoHS/REACH/PFAS (F-112)", mono: true },
];

export function filterItemsForBomView(
  viewMode: PlmBomViewMode,
  items: readonly PlmBomItemRow[] = DEFAULT_MULTI_BOM_ITEMS,
): readonly PlmBomItemRow[] {
  if (viewMode === "EBOM") {
    return items.filter((i) => i.domainOrigin !== "PACKAGING_SERVICE");
  }
  if (viewMode === "SBOM") {
    return items.filter((i) => i.fieldSpareReplaceable);
  }
  return items;
}

export function buildMultiBomPassportVm(
  viewMode: PlmBomViewMode,
  items: readonly PlmBomItemRow[] = DEFAULT_MULTI_BOM_ITEMS,
): MultiBomPassportVm {
  const visible = filterItemsForBomView(viewMode, items);
  const totalCostMicrosUsd = visible.reduce(
    (acc, i) => acc + i.unitCostMicrosUsd * i.qtyPerUnit,
    0n,
  );
  const totalEmbodiedCo2eMg = visible.reduce(
    (acc, i) => acc + i.embodiedCo2eMg * i.qtyPerUnit,
    0n,
  );
  const dppCompliant = visible.every((i) => i.rohsReachPfasClean);

  const usdText = (Number(totalCostMicrosUsd) / 1_000_000).toFixed(2);
  const kgCo2Text = (Number(totalEmbodiedCo2eMg) / 1_000_000).toFixed(3);
  const rollupBadge = `${viewMode} Rollup (${visible.length} Lines): $${usdText} • ${kgCo2Text} kgCO2e`;
  const dppTone: SigStatusTone = dppCompliant ? "ok" : "danger";
  const dppBadge = dppCompliant
    ? "EU Digital Product Passport: RoHS 3 + REACH SVHC-Free + PFAS-Free (F-112)"
    : "DPP Compliance Blocked";

  const rows: SigGridRow[] = visible.map((i) => {
    const extUsd = (Number(i.unitCostMicrosUsd * i.qtyPerUnit) / 1_000_000).toFixed(2);
    const gCo2 = ((i.embodiedCo2eMg * i.qtyPerUnit) / 1_000n).toString(10);
    return {
      id: i.lineId,
      mpn: `${i.qtyPerUnit.toString(10)}x ${i.orderableMpn}`,
      origin: `${i.domainOrigin} -> ${i.workstationStepId}`,
      extCost: `$${extUsd}`,
      co2: `${gCo2} gCO2e`,
      dpp: i.rohsReachPfasClean ? "PASS_DPP" : "SUBSTANCE_ALERT",
    };
  });

  return {
    viewMode,
    viewModes: ["EBOM", "MBOM", "SBOM"],
    totalCostMicrosUsd,
    totalEmbodiedCo2eMg,
    dppCompliant,
    rollupBadge,
    dppBadge,
    dppTone,
    columns: BOM_COLUMNS,
    rows,
  };
}
