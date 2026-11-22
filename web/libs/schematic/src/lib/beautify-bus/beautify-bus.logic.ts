// Module: web/libs/schematic/src/lib/beautify-bus/beautify-bus.logic.ts — Pure Sugiyama Delta & Typed Bus Slice Expander
// Invariants: Deterministic bus expansion (DATA[31:0], AXI4_LITE, CAN_FD_DIFF, SPI_QUAD) capped <=128 lanes; bigint nm math.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { BeautifyBusVm, BeautifyPreviewInput, BusProtocolKind, BusSliceExpansion } from "./beautify-bus.model";

const PROTOCOL_PRESETS: Readonly<Record<Exclude<BusProtocolKind, "RAW_VECTOR">, readonly string[]>> = {
  AXI4_LITE: [
    "AWADDR[31:0]",
    "AWVALID",
    "AWREADY",
    "WDATA[31:0]",
    "WSTRB[3:0]",
    "WVALID",
    "WREADY",
    "BRESP[1:0]",
    "BVALID",
    "BREADY",
    "ARADDR[31:0]",
    "ARVALID",
    "ARREADY",
    "RDATA[31:0]",
    "RVALID",
    "RREADY",
  ],
  CAN_FD_DIFF: ["CAN_H", "CAN_L", "CAN_SHIELD", "CAN_STB_N"],
  SPI_QUAD: ["QSPI_CS_N", "QSPI_SCK", "QSPI_IO0", "QSPI_IO1", "QSPI_IO2", "QSPI_IO3"],
};

const NET_COLUMNS: readonly SigGridColumn[] = [
  { key: "lane", header: "Lane #", mono: true },
  { key: "net", header: "Expanded Member Net", mono: true },
  { key: "protocol", header: "Protocol / Slice" },
];

export function expandBusSliceExpression(
  expression: string,
  protocol: BusProtocolKind,
): BusSliceExpansion {
  if (protocol !== "RAW_VECTOR") {
    const members = PROTOCOL_PRESETS[protocol];
    return {
      valid: true,
      bundleLabel: `${protocol} (${expression.trim() || "BUS0"})`,
      bitWidth: members.length,
      directionBadge: "Protocol Bundle",
      expandedNets: members.map((m) => `${expression.trim() || "BUS0"}.${m}`),
      errorHint: "",
    };
  }

  const trimmed = expression.trim();
  const match = /^([A-Za-z_][A-Za-z0-9_]*)\[(\d+):(\d+)\]$/.exec(trimmed);
  if (!match) {
    return {
      valid: false,
      bundleLabel: trimmed || "INVALID",
      bitWidth: 0,
      directionBadge: "Syntax Error",
      expandedNets: [],
      errorHint: "Expected <NAME>[<MSB>:<LSB>], e.g. DATA[31:0]",
    };
  }

  const prefix = match[1];
  const left = Number(match[2]);
  const right = Number(match[3]);
  const step = left >= right ? -1 : 1;
  const bitWidth = Math.abs(left - right) + 1;
  if (bitWidth > 128) {
    return {
      valid: false,
      bundleLabel: trimmed,
      bitWidth,
      directionBadge: "Bounds Exceeded",
      expandedNets: [],
      errorHint: "Slice width exceeds 128-lane schematic bundle cap",
    };
  }

  const expandedNets: string[] = [];
  for (let idx = left; step < 0 ? idx >= right : idx <= right; idx += step) {
    expandedNets.push(`${prefix}_${idx}`);
  }
  return {
    valid: true,
    bundleLabel: trimmed,
    bitWidth,
    directionBadge: left >= right ? "MSB -> LSB" : "LSB -> MSB",
    expandedNets,
    errorHint: "",
  };
}

export function buildBeautifyBusVm(
  preview: BeautifyPreviewInput,
  sliceExpr: string,
  protocol: BusProtocolKind,
): BeautifyBusVm {
  const reduction = Math.max(0, preview.crossingCountBefore - preview.crossingCountAfter);
  const pct =
    preview.crossingCountBefore > 0
      ? Math.round((reduction / preview.crossingCountBefore) * 100)
      : 100;
  const crossingTone: SigStatusTone = preview.applied ? "ok" : "warn";
  const crossingBadge = preview.applied
    ? `Applied: ${preview.crossingCountAfter} Crossings (-${pct}%)`
    : `Preview: ${preview.crossingCountBefore} -> ${preview.crossingCountAfter} Crossings (-${pct}%)`;
  const busExpansion = expandBusSliceExpression(sliceExpr, protocol);
  const netRows: SigGridRow[] = busExpansion.expandedNets.map((net, i) => ({
    id: `lane-${i}`,
    lane: `[${i}]`,
    net,
    protocol: busExpansion.directionBadge,
  }));

  return {
    crossingBadge,
    crossingTone,
    wirelengthBadge: `Saved: ${(preview.wirelengthSavedNm / 1_000n).toString(10)} um`,
    gridPitchLabel: `Grid Pitch: ${preview.gridPitchNm.toString(10)} nm`,
    applyButtonLabel: preview.applied ? "Reset Beautify Preview" : "Apply Sugiyama + A* Untangle",
    busExpansion,
    netColumns: NET_COLUMNS,
    netRows,
  };
}
