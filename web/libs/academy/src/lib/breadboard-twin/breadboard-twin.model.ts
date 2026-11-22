// Module: web/libs/academy/src/lib/breadboard-twin/breadboard-twin.model.ts — FEAT-111 Breadboard Twin Connectivity Models
// Invariants: Models 5-hole strip equivalence classes (A-E vs F-J across center IC ravine) and 0-ohm VCC<->GND short detection.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type BreadboardElementKind = "JUMPER_0_OHM" | "COMPONENT_LOAD";

export interface BreadboardJumperWire {
  readonly wireId: string;
  readonly kind: BreadboardElementKind;
  readonly fromHole: string;
  readonly toHole: string;
  readonly colorLabel: string;
}

export interface BreadboardConnectivityAnalysis {
  readonly hasPowerGroundShort: boolean;
  readonly resolvedStripNets: Readonly<Record<string, string>>;
  readonly warningSummary: string;
  readonly tone: SigStatusTone;
}

export interface BreadboardTwinVm {
  readonly analysis: BreadboardConnectivityAnalysis;
  readonly wires: readonly BreadboardJumperWire[];
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
