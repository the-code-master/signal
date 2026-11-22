// Module: web/libs/schematic/src/lib/live-sim-hud/live-sim-hud.model.ts — FEAT-003 Interactive Controls & FEAT-004 Current-River HUD Models
// Invariants: Resistance (_mohm), current (_na), and temperature (_mkelvin) stored strictly as ES2020 bigint.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";

export type SchematicFaultMode =
  | "NONE"
  | "OPEN_CIRCUIT"
  | "SHORT_CIRCUIT"
  | "PARAMETRIC_DRIFT"
  | "STUCK_AT_LOGIC";

export interface LiveSimHudState {
  readonly potTotalMohm: bigint;
  readonly wiperPercent: number;
  readonly switchClosed: boolean;
  readonly faultMode: SchematicFaultMode;
  readonly currentRiverEnabled: boolean;
  readonly soaThermalEnabled: boolean;
  readonly rmsCurrentNa: bigint;
  readonly junctionTempMkelvin: bigint;
  readonly maxJunctionTempMkelvin: bigint;
}

export interface LiveSimHudVm {
  readonly wiperResistanceText: string;
  readonly upperLegMohm: bigint;
  readonly lowerLegMohm: bigint;
  readonly switchBadge: string;
  readonly switchTone: SigStatusTone;
  readonly faultBadge: string;
  readonly faultTone: SigStatusTone;
  readonly riverDensityBadge: string;
  readonly soaThermalBadge: string;
  readonly soaTone: SigStatusTone;
}
