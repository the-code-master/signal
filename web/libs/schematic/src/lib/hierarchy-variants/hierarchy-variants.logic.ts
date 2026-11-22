// Module: web/libs/schematic/src/lib/hierarchy-variants/hierarchy-variants.logic.ts — Cross-Sheet Portals, Repeat(N) & DNP Variants
// Invariants: Deterministic Repeat(N) RefDes offset calculation and assembly stuffing resolution without HTML @if chains.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type {
  AssemblyVariantId,
  HierarchyVariantsVm,
  MultiChannelRuleInput,
  NetPortalDescriptor,
  VariantBomItem,
} from "./hierarchy-variants.model";

export const DEFAULT_PORTALS: readonly NetPortalDescriptor[] = [
  {
    id: "p1",
    netName: "HV_DC_BUS_800V",
    sourceSheetId: "S01_DC_LINK",
    targetSheetId: "S02_SIC_BRIDGE",
    anchorXNm: 14_500_000n,
    anchorYNm: 8_200_000n,
  },
  {
    id: "p2",
    netName: "PWM_SYNC_MASTER",
    sourceSheetId: "S03_DSP_CTRL",
    targetSheetId: "S02_SIC_BRIDGE",
    anchorXNm: 22_100_000n,
    anchorYNm: 11_750_000n,
  },
  {
    id: "p3",
    netName: "SAFE_TORQUE_OFF_N",
    sourceSheetId: "S04_ISO26262_SAFETY",
    targetSheetId: "S02_SIC_BRIDGE",
    anchorXNm: 6_400_000n,
    anchorYNm: 19_000_000n,
  },
];

export const DEFAULT_VARIANT_BOM: readonly VariantBomItem[] = [
  {
    entityId: "Q1",
    primaryMpn: "C3M0016120K (1200V SiC)",
    alternateMpn: "C3M0021120K (1200V Econ)",
    dnpVariants: [],
    useAltInCostReduced: true,
  },
  {
    entityId: "C_SNUB_1",
    primaryMpn: "C2220C473JCGAC (47nF C0G)",
    alternateMpn: "C1812C473KCGAC",
    dnpVariants: ["COST_REDUCED"],
    useAltInCostReduced: false,
  },
  {
    entityId: "J_BENCH_PROBE",
    primaryMpn: "MMCX-73415-0961",
    alternateMpn: "MMCX-73415-0961",
    dnpVariants: ["PROD_800V", "COST_REDUCED"],
    useAltInCostReduced: false,
  },
];

const CHANNEL_COLUMNS: readonly SigGridColumn[] = [
  { key: "channel", header: "Channel", mono: true },
  { key: "hsSwitch", header: "HS Switch RefDes", mono: true },
  { key: "driver", header: "Gate Driver RefDes", mono: true },
  { key: "phaseNet", header: "Expanded Phase Net", mono: true },
];

const VARIANT_COLUMNS: readonly SigGridColumn[] = [
  { key: "entityId", header: "RefDes", mono: true },
  { key: "status", header: "Stuffing State", mono: true },
  { key: "resolvedMpn", header: "Resolved Orderable MPN", mono: true },
];

export function expandMultiChannelMatrix(rule: MultiChannelRuleInput): readonly SigGridRow[] {
  const count = Math.max(1, Math.min(64, Math.round(rule.repeatCount)));
  const rows: SigGridRow[] = [];
  for (let ch = 1; ch <= count; ch += 1) {
    const base = ch * rule.refdesBaseStep;
    const phaseNet = rule.netTemplate.replace("{ch}", String(ch));
    rows.push({
      id: `${rule.blockId}-ch-${ch}`,
      channel: `CH${ch} (${rule.blockId})`,
      hsSwitch: `Q${base + 1}`,
      driver: `U${base + 2}`,
      phaseNet,
    });
  }
  return rows;
}

export function resolveVariantStuffingRows(
  items: readonly VariantBomItem[],
  variantId: AssemblyVariantId,
): { readonly rows: readonly SigGridRow[]; readonly populatedCount: number; readonly dnpCount: number } {
  let populatedCount = 0;
  let dnpCount = 0;
  const rows: SigGridRow[] = items.map((item) => {
    const isDnp = item.dnpVariants.includes(variantId);
    if (isDnp) {
      dnpCount += 1;
      return {
        id: item.entityId,
        entityId: item.entityId,
        status: "DNP (Curtain Masked)",
        resolvedMpn: "DO_NOT_POPULATE",
      };
    }
    populatedCount += 1;
    const resolvedMpn =
      variantId === "COST_REDUCED" && item.useAltInCostReduced ? item.alternateMpn : item.primaryMpn;
    return {
      id: item.entityId,
      entityId: item.entityId,
      status: "POPULATED",
      resolvedMpn,
    };
  });
  return { rows, populatedCount, dnpCount };
}

export function buildHierarchyVariantsVm(
  selectedPortalId: string,
  repeatCount: number,
  variantId: AssemblyVariantId,
): HierarchyVariantsVm {
  const portal = DEFAULT_PORTALS.find((p) => p.id === selectedPortalId) ?? DEFAULT_PORTALS[0];
  const channelRows = expandMultiChannelMatrix({
    blockId: "HALF_BRIDGE",
    repeatCount,
    refdesBaseStep: 100,
    netTemplate: "SW_PHASE_CH{ch}",
  });
  const variantRes = resolveVariantStuffingRows(DEFAULT_VARIANT_BOM, variantId);
  return {
    portals: DEFAULT_PORTALS,
    selectedPortalSummary: `${portal.netName}: ${portal.sourceSheetId} -> ${portal.targetSheetId} @ (${portal.anchorXNm.toString(10)}, ${portal.anchorYNm.toString(10)}) nm`,
    channelBadge: `Repeat(${channelRows.length}): ${channelRows.length * 2} Instantiated Power Devices`,
    channelColumns: CHANNEL_COLUMNS,
    channelRows,
    variantSummaryBadge: `${variantId}: ${variantRes.populatedCount} Populated / ${variantRes.dnpCount} DNP`,
    variantColumns: VARIANT_COLUMNS,
    variantRows: variantRes.rows,
  };
}
