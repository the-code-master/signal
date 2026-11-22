// Module: web/libs/sim-viewer/src/lib/anomaly-debugger/anomaly-debugger.logic.ts — FEAT-106 Root-Cause Attribution Logic
// Invariants: Ranks parasitic/pole-zero root causes and deterministic mitigations over bigint ps windows.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type {
  AnomalyAttributionCase,
  AnomalyDebuggerVm,
  AnomalySymptomId,
} from "./anomaly-debugger.model";

export const DEFAULT_ANOMALY_CASES: readonly AnomalyAttributionCase[] = [
  {
    id: "SW_RINGING_180MHZ",
    netName: "HV_SW_PHASE_U",
    windowStartPs: 120_000n,
    windowEndPs: 165_000n,
    dominantRootCause: "LC Resonance: L_loop_stray (4.8 nH) x C_oss(Q2_LS, 162 pF) @ 180.4 MHz",
    culpritEntityId: "LOOP_HB_DECAP_U1",
    rankedMitigations: [
      "1. Place RC Snubber (3.3 Ω + 2.2 nF C0G) across Q2_LS",
      "2. Move C_HF_DECAP within 1.5 mm of Q1 Drain / Q2 Source",
      "3. Increase R_gate_on from 2.2 Ω to 3.3 Ω (-28% dV/dt)",
    ],
  },
  {
    id: "LOAD_STEP_DROOP",
    netName: "VOUT_12V_AUX",
    windowStartPs: 2_000_000n,
    windowEndPs: 8_500_000n,
    dominantRootCause: "MLCC DC-Bias Derating: C_OUT_1..4 lose 62% effective capacitance at 12V",
    culpritEntityId: "C_OUT_BANK",
    rankedMitigations: [
      "1. Upgrade 0805 25V X5R to 1210 50V X7R (ConditionCurveSet verified)",
      "2. Shift Type-III zero f_z1 from 18 kHz to 11 kHz",
    ],
  },
  {
    id: "MILLER_GLITCH",
    netName: "GATE_LS_U",
    windowStartPs: 122_000n,
    windowEndPs: 130_000n,
    dominantRootCause: "C_gd * dV_ds/dt displacement current exceeds gate pull-down R_off=2.2 Ω",
    culpritEntityId: "U2_GATE_DRV",
    rankedMitigations: [
      "1. Route dedicated OUTL Miller Clamp pin directly to Q2 Gate",
      "2. Apply -4V negative turn-off bias rail VEE_DRV",
    ],
  },
];

const MITIGATION_COLUMNS: readonly SigGridColumn[] = [
  { key: "rank", header: "Rank", mono: true },
  { key: "mitigation", header: "Deterministic Root-Cause Mitigation" },
  { key: "culprit", header: "Attributed Entity", mono: true },
];

export function buildAnomalyDebuggerVm(
  symptomId: AnomalySymptomId,
  appliedMitigation: string | null,
  cases: readonly AnomalyAttributionCase[] = DEFAULT_ANOMALY_CASES,
): AnomalyDebuggerVm {
  const selectedCase = cases.find((c) => c.id === symptomId) ?? cases[0];
  const windowBadge = `Net ${selectedCase.netName} [${selectedCase.windowStartPs.toString(10)}..${selectedCase.windowEndPs.toString(10)} ps]`;
  const rows: SigGridRow[] = selectedCase.rankedMitigations.map((m, idx) => ({
    id: `${selectedCase.id}-m-${idx}`,
    rank: `#${idx + 1}`,
    mitigation: m,
    culprit: selectedCase.culpritEntityId,
  }));

  return {
    cases,
    selectedCase,
    appliedMitigation,
    windowBadge,
    columns: MITIGATION_COLUMNS,
    rows,
  };
}
