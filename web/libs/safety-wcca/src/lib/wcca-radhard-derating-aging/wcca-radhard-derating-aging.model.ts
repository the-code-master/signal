// Module: web/libs/safety-wcca/src/lib/wcca-radhard-derating-aging/wcca-radhard-derating-aging.model.ts — FEAT-037/038/040 & I-05 Models
// Invariants: WCCA voltage bounds (_uv), SET pulse widths (_ps), and NBTI threshold shifts (_uv) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export type DeratingPolicyStandard = "ECSS_SPACE_CLASS_1" | "NAVSEA_MILITARY" | "ENTERPRISE_AUTOMOTIVE";

export interface WccaRadhardDeratingAgingVm {
  readonly policy: DeratingPolicyStandard;
  readonly policies: readonly DeratingPolicyStandard[];
  readonly missionYears: number;
  readonly yearPresets: readonly number[];
  readonly nbtiThresholdShiftUv: bigint;
  readonly setTransientPulsePs: bigint;
  readonly wccaWorstCaseUv: bigint;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly radDeratingBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
