// Module: web/libs/academy/src/lib/breadboard-twin/breadboard-twin.logic.ts — FEAT-111 5-Hole Strip Union-Find & Short Detector
// Invariants: Unions 0-ohm jumpers across 5-hole copper clips (A-E vs F-J); flags direct VCC_RAIL == GND_RAIL dead shorts.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type {
  BreadboardConnectivityAnalysis,
  BreadboardJumperWire,
  BreadboardTwinVm,
} from "./breadboard-twin.model";

export const SAFE_BREADBOARD_WIRES: readonly BreadboardJumperWire[] = [
  { wireId: "W1", kind: "JUMPER_0_OHM", fromHole: "VCC_RAIL", toHole: "A1", colorLabel: "Red Jumper (+5V -> Strip AE_1)" },
  { wireId: "R1", kind: "COMPONENT_LOAD", fromHole: "E1", toHole: "F1", colorLabel: "150 Ω Resistor (Across IC Ravine)" },
  { wireId: "D1", kind: "COMPONENT_LOAD", fromHole: "J1", toHole: "J5", colorLabel: "Red LED (Anode FJ_1 -> Cathode FJ_5)" },
  { wireId: "W2", kind: "JUMPER_0_OHM", fromHole: "F5", toHole: "GND_RAIL", colorLabel: "Black Jumper (Strip FJ_5 -> GND)" },
];

export const SHORTED_BREADBOARD_WIRE: BreadboardJumperWire = {
  wireId: "W_ERR",
  kind: "JUMPER_0_OHM",
  fromHole: "B1",
  toHole: "GND_RAIL",
  colorLabel: "Misplaced 0Ω Jumper (Same 5-Hole Strip AE_1 -> GND!)",
};

const STRIP_COLUMNS: readonly SigGridColumn[] = [
  { key: "wireId", header: "Element", mono: true },
  { key: "endpoints", header: "Breadboard Holes", mono: true },
  { key: "strips", header: "5-Hole Strip Equivalence", mono: true },
  { key: "note", header: "Schematic Twin Sync" },
];

export function holeToCanonical5HoleStrip(hole: string): string {
  const clean = hole.trim().toUpperCase();
  if (clean === "VCC_RAIL" || clean === "GND_RAIL") {
    return clean;
  }
  const match = /^([A-J])(\d{1,2})$/.exec(clean);
  if (!match) {
    return clean;
  }
  const rowChar = match[1];
  const colNum = match[2];
  const bank = rowChar <= "E" ? "AE" : "FJ";
  return `STRIP_${bank}_${colNum}`;
}

export function analyzeBreadboardConnectivity(
  wires: readonly BreadboardJumperWire[],
): BreadboardConnectivityAnalysis {
  const parent = new Map<string, string>();

  function find(node: string): string {
    const cur = parent.get(node);
    if (cur === undefined || cur === node) {
      parent.set(node, node);
      return node;
    }
    const root = find(cur);
    parent.set(node, root);
    return root;
  }

  function union(a: string, b: string): void {
    const ra = find(a);
    const rb = find(b);
    if (ra !== rb) parent.set(ra, rb);
  }

  for (const w of wires) {
    const sa = holeToCanonical5HoleStrip(w.fromHole);
    const sb = holeToCanonical5HoleStrip(w.toHole);
    find(sa);
    find(sb);
    if (w.kind === "JUMPER_0_OHM") {
      union(sa, sb);
    }
  }

  const vccRoot = find("VCC_RAIL");
  const gndRoot = find("GND_RAIL");
  const hasPowerGroundShort = vccRoot === gndRoot && wires.length > 0;

  const resolvedStripNets: Record<string, string> = {};
  for (const node of parent.keys()) {
    const r = find(node);
    resolvedStripNets[node] =
      r === vccRoot && !hasPowerGroundShort
        ? "NET_VCC_5V"
        : r === gndRoot && !hasPowerGroundShort
          ? "NET_GND"
          : hasPowerGroundShort && r === vccRoot
            ? "SHORT_VCC_TO_GND"
            : `NET_${r}`;
  }

  return {
    hasPowerGroundShort,
    resolvedStripNets,
    warningSummary: hasPowerGroundShort
      ? "SHORT CIRCUIT DETECTED: Holes A1..E1 share a single 5-hole copper clip — VCC_RAIL is shorted directly to GND_RAIL!"
      : "Breadboard <-> Schematic Bijection Clean (0 Rail Shorts across 5-Hole Strips)",
    tone: hasPowerGroundShort ? "danger" : "ok",
  };
}

export function buildBreadboardTwinVm(injectShort: boolean): BreadboardTwinVm {
  const wires = injectShort
    ? [...SAFE_BREADBOARD_WIRES, SHORTED_BREADBOARD_WIRE]
    : SAFE_BREADBOARD_WIRES;
  const analysis = analyzeBreadboardConnectivity(wires);
  const rows: SigGridRow[] = wires.map((w) => ({
    id: w.wireId,
    wireId: w.wireId,
    endpoints: `${w.fromHole} -> ${w.toHole}`,
    strips: `${holeToCanonical5HoleStrip(w.fromHole)} <-> ${holeToCanonical5HoleStrip(w.toHole)}`,
    note: w.colorLabel,
  }));

  return {
    analysis,
    wires,
    columns: STRIP_COLUMNS,
    rows,
  };
}
