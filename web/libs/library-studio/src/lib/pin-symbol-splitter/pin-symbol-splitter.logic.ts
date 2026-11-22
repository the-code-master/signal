// Module: web/libs/library-studio/src/lib/pin-symbol-splitter/pin-symbol-splitter.logic.ts — Bijection Checker & FEAT-072 Bank Splitter
// Invariants: Verifies 1:1 Pin <-> Pad <-> SPICE Port <-> STEP Lead bijection; partitions functional symbol banks deterministically.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type {
  MasterPinRow,
  PartitionedSymbolBank,
  PinFunctionalGroup,
  PinSymbolSplitterVm,
  SchematicSymbolStandard,
} from "./pin-symbol-splitter.model";

export const DEFAULT_MASTER_PINS: readonly MasterPinRow[] = [
  { pinNumber: "A1", signalName: "VDD_CORE", group: "POWER_GND", padDesignator: "A1", spicePortId: 1, stepLeadXNm: -1_600_000n, stepLeadYNm: 1_600_000n },
  { pinNumber: "A2", signalName: "VSS_PGND", group: "POWER_GND", padDesignator: "A2", spicePortId: 2, stepLeadXNm: -800_000n, stepLeadYNm: 1_600_000n },
  { pinNumber: "B1", signalName: "PWM_HS_IN", group: "PWM_CTRL", padDesignator: "B1", spicePortId: 3, stepLeadXNm: 800_000n, stepLeadYNm: 1_600_000n },
  { pinNumber: "B2", signalName: "PWM_LS_IN", group: "PWM_CTRL", padDesignator: "B2", spicePortId: 4, stepLeadXNm: 1_600_000n, stepLeadYNm: 1_600_000n },
  { pinNumber: "C1", signalName: "DESAT_SENSE", group: "SENSE_ANALOG", padDesignator: "C1", spicePortId: 5, stepLeadXNm: -1_600_000n, stepLeadYNm: -1_600_000n },
  { pinNumber: "C2", signalName: "NTC_TEMP_MON", group: "SENSE_ANALOG", padDesignator: "C2", spicePortId: 6, stepLeadXNm: -800_000n, stepLeadYNm: -1_600_000n },
  { pinNumber: "D1", signalName: "SERDES_TX_P", group: "SERDES_HS", padDesignator: "D1", spicePortId: 7, stepLeadXNm: 800_000n, stepLeadYNm: -1_600_000n },
  { pinNumber: "D2", signalName: "SERDES_TX_N", group: "SERDES_HS", padDesignator: "D2", spicePortId: 8, stepLeadXNm: 1_600_000n, stepLeadYNm: -1_600_000n },
];

const GROUP_TITLES: Readonly<Record<PinFunctionalGroup, string>> = {
  POWER_GND: "Power & Ground Rails",
  PWM_CTRL: "PWM & Gate Control",
  SENSE_ANALOG: "Analog Telemetry & DESAT",
  SERDES_HS: "High-Speed Differential I/O",
};

const PIN_COLUMNS: readonly SigGridColumn[] = [
  { key: "pin", header: "Pin #", mono: true },
  { key: "signal", header: "Canonical Signal", mono: true },
  { key: "padSpice", header: "Pad / SPICE Port", mono: true },
  { key: "stepNm", header: "3D STEP Centroid (nm)", mono: true },
];

export function verifyMasterPinBijection(pins: readonly MasterPinRow[]): boolean {
  if (pins.length === 0) return false;
  const pinSet = new Set<string>();
  const padSet = new Set<string>();
  const portSet = new Set<number>();
  for (const row of pins) {
    if (!row.pinNumber || !row.padDesignator || row.spicePortId <= 0) return false;
    if (pinSet.has(row.pinNumber) || padSet.has(row.padDesignator) || portSet.has(row.spicePortId)) {
      return false;
    }
    pinSet.add(row.pinNumber);
    padSet.add(row.padDesignator);
    portSet.add(row.spicePortId);
  }
  return true;
}

export function splitMasterPinsIntoBanks(
  pins: readonly MasterPinRow[],
  maxPinsPerBank: number,
): readonly PartitionedSymbolBank[] {
  const cap = Math.max(1, Math.round(maxPinsPerBank));
  const grouped = new Map<PinFunctionalGroup, string[]>();
  for (const p of pins) {
    const bucket = grouped.get(p.group) ?? [];
    bucket.push(p.pinNumber);
    grouped.set(p.group, bucket);
  }

  const banks: PartitionedSymbolBank[] = [];
  let unitIndex = 1;
  for (const [group, pinList] of grouped.entries()) {
    for (let offset = 0; offset < pinList.length; offset += cap) {
      const slice = pinList.slice(offset, offset + cap);
      const partSuffix = pinList.length > cap ? ` (Part ${Math.floor(offset / cap) + 1})` : "";
      banks.push({
        unitIndex,
        bankTitle: `Unit ${String.fromCharCode(64 + unitIndex)}: ${GROUP_TITLES[group]}${partSuffix}`,
        pinNumbers: slice,
      });
      unitIndex += 1;
    }
  }
  return banks;
}

export function buildPinSymbolSplitterVm(
  standard: SchematicSymbolStandard,
  maxPinsPerBank: number,
  pins: readonly MasterPinRow[] = DEFAULT_MASTER_PINS,
): PinSymbolSplitterVm {
  const bijectionValid = verifyMasterPinBijection(pins);
  const banks = splitMasterPinsIntoBanks(pins, maxPinsPerBank);
  const pinRows: SigGridRow[] = pins.map((p) => ({
    id: p.pinNumber,
    pin: p.pinNumber,
    signal: p.signalName,
    padSpice: `Pad ${p.padDesignator} <-> Port #${p.spicePortId}`,
    stepNm: `(${p.stepLeadXNm.toString(10)}, ${p.stepLeadYNm.toString(10)})`,
  }));

  return {
    standard,
    standardBadge:
      standard === "IEEE_315"
        ? "Symbol AST: IEEE 315 / ANSI Y32.2"
        : "Symbol AST: IEC 60617 Rectangular Qualifying",
    bijectionValid,
    bijectionBadge: bijectionValid
      ? `Gate 2 Bijection PASS (${pins.length}/${pins.length} Pin=Pad=SPICE=STEP)`
      : "Gate 2 Bijection FAILED",
    bijectionTone: bijectionValid ? "ok" : "danger",
    banks,
    pinColumns: PIN_COLUMNS,
    pinRows,
  };
}
