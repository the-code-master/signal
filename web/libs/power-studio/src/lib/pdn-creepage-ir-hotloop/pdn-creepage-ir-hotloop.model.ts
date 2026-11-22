// Module: web/libs/power-studio/src/lib/pdn-creepage-ir-hotloop/pdn-creepage-ir-hotloop.model.ts — FEAT-022..025 Models
// Invariants: Impedances (_mohm), creepage distances (_nm), IR-drop (_uv), and parasitic L (_ph) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface PdnFrequencyBand {
  readonly bandLabel: string;
  readonly freqHz: bigint;
  readonly zTargetMohm: bigint;
  readonly zMeasuredMohm: bigint;
}

export interface PdnCreepageIrHotloopVm {
  readonly slotRibAdded: boolean;
  readonly negativeGateBiasEnabled: boolean;
  readonly geodesicCreepageNm: bigint;
  readonly requiredCreepageNm: bigint;
  readonly busbarIrDropUv: bigint;
  readonly hotLoopInductancePh: bigint;
  readonly millerMarginUv: bigint;
  readonly summaryBadge: string;
  readonly summaryTone: SigStatusTone;
  readonly hotLoopBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
