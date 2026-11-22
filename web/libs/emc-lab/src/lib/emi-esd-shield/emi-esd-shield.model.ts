// Module: web/libs/emc-lab/src/lib/emi-esd-shield/emi-esd-shield.model.ts — FEAT-032/033/034 EMI Filter, ESD & Shield Models
// Invariants: Voltages (_uv), frequencies (_hz), and aperture lengths (_nm) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type EsdStressLevel = "IEC_8KV_CONTACT" | "IEC_15KV_AIR" | "ISO7637_LOAD_DUMP";

export interface EmiEsdShieldVm {
  readonly esdLevel: EsdStressLevel;
  readonly esdLevels: readonly EsdStressLevel[];
  readonly dampingNetworkEnabled: boolean;
  readonly middlebrookSeparationDb: number;
  readonly tvsClampedPeakUv: bigint;
  readonly protectedPinAbsMaxUv: bigint;
  readonly cavityFundamentalHz: bigint;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
