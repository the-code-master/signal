// Module: web/libs/academy/src/lib/visual-failure-fx/visual-failure-fx.model.ts — FEAT-113 SOA Smoke & Thermal Failure Models
// Invariants: Voltage (_uv), power (_uw), and junction temperature (_mkelvin) stored strictly as ES2020 bigint.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigUiProfile } from "@signal/design-system";
import type { SigStatusTone } from "@signal/ui-kit";

export interface SoaFailureScenario {
  readonly id: string;
  readonly entityId: string;
  readonly partSummary: string;
  readonly failureMode: string;
  readonly peakVoltageUv: bigint;
  readonly ratedVoltageUv: bigint;
  readonly peakPowerUw: bigint;
  readonly ratedPowerUw: bigint;
  readonly junctionTempMkelvin: bigint;
  readonly maxTempMkelvin: bigint;
  readonly upgradeMpn: string;
  readonly upgradeVoltageUv: bigint;
  readonly upgradePowerUw: bigint;
}

export interface AutoFixUpgradeEvent {
  readonly entityId: string;
  readonly upgradeMpn: string;
  readonly newRatedVoltageUv: bigint;
  readonly newRatedPowerUw: bigint;
}

export interface VisualFailureFxVm {
  readonly scenario: SoaFailureScenario;
  readonly profile: SigUiProfile;
  readonly autoFixed: boolean;
  readonly fxBadge: string;
  readonly fxTone: SigStatusTone;
  readonly telemetryPills: readonly string[];
  readonly diagnosticExplanation: string;
  readonly actionButtonLabel: string;
}
