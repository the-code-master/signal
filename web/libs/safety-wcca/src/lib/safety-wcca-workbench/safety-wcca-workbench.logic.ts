// Module: web/libs/safety-wcca/src/lib/safety-wcca-workbench/safety-wcca-workbench.logic.ts — FEAT-035/036 FMEDA & ZBDD FTA Logic
// Invariants: Evaluates ISO 26262 ASIL-B/C/D SPFM, LFM, and PMHF FIT thresholds plus ZBDD minimal cut-set order.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  AsilTargetLevel,
  SafetyWccaWorkbenchVm,
} from "./safety-wcca-workbench.model";

export const SAFETY_WCCA_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "safety-wcca",
  title: "ISO 26262 FMEDA, FTA, WCCA & 15-Year Aging Studio",
  subtitle: "SPFM/LFM/PMHF, ZBDD cut-sets, Extreme-Value WCCA, Rad-Hard & I-05 Aging",
  rustCrate: "signal-emc",
  subprojectRepo: "aurora-ev-inverter-prd",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "ASIL-D SPFM: 99.4%", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "PMHF: 4.8 FIT", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["LFM: 92.1%", "WCCA: 6-Sigma PASS"],
  gridTitle: "Safety Mechanisms",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "SM1", item: "Dual Desat + OC Crowbar", metric: "99.6%" },
    { id: "SM2", item: "Redundant Resolver ADC", metric: "99.1%" },
  ],
};

const FMEDA_COLUMNS: readonly SigGridColumn[] = [
  { key: "cutSet", header: "ZBDD Minimal Cut-Set (FEAT-036)", mono: true },
  { key: "order", header: "Cut Order", mono: true },
  { key: "coverage", header: "Diagnostic Mechanism (FEAT-035)", mono: true },
  { key: "fit", header: "Residual FIT", mono: true },
];

export function evaluateAsilGatePassing(
  target: AsilTargetLevel,
  spfmPct: number,
  lfmPct: number,
  pmhfFit: number,
): boolean {
  if (target === "ASIL_D") return spfmPct >= 99.0 && lfmPct >= 90.0 && pmhfFit < 10.0;
  if (target === "ASIL_C") return spfmPct >= 97.0 && lfmPct >= 80.0 && pmhfFit < 100.0;
  return spfmPct >= 90.0 && lfmPct >= 60.0 && pmhfFit < 100.0;
}

export function buildSafetyWccaWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  asilTarget: AsilTargetLevel,
  redundantCrowbarEnabled: boolean,
): SafetyWccaWorkbenchVm {
  const spfmPct = redundantCrowbarEnabled ? 99.4 : 96.2;
  const lfmPct = redundantCrowbarEnabled ? 92.1 : 84.5;
  const pmhfFit = redundantCrowbarEnabled ? 4.8 : 18.6;
  const passing = evaluateAsilGatePassing(asilTarget, spfmPct, lfmPct, pmhfFit);

  const fmedaTone: SigStatusTone = passing ? "ok" : "danger";
  const fmedaBadge = `${asilTarget} FMEDA (FEAT-035): SPFM=${spfmPct}% • LFM=${lfmPct}% • PMHF=${pmhfFit} FIT (${passing ? "PASS" : "FAIL"})`;
  const minOrder = redundantCrowbarEnabled ? 2 : 1;
  const ftaCutSetBadge = `ZBDD Fault-Tree (FEAT-036): Min Cut-Set Order=${minOrder} (${redundantCrowbarEnabled ? "Zero Single-Point Unintended Torque" : "1st-Order SPF Exposed!"})`;

  const rows: SigGridRow[] = [
    {
      id: "MCS-1",
      cutSet: redundantCrowbarEnabled ? "{U2_DESAT_FAIL ∧ U5_CROWBAR_STUCK}" : "{U2_DESAT_FAIL}",
      order: redundantCrowbarEnabled ? "Order 2 (Dual-Point)" : "Order 1 (Single-Point)",
      coverage: "SM-01 Hardware Desat + Safe-Torque-Off Crowbar",
      fit: redundantCrowbarEnabled ? "1.9 FIT" : "15.7 FIT",
    },
    {
      id: "MCS-2",
      cutSet: "{ADC_SIN_DRIFT ∧ ADC_COS_DRIFT}",
      order: "Order 2 (Dual-Point)",
      coverage: "SM-02 Sin²+Cos²=1 Hardware Plausibility Monitor",
      fit: "2.9 FIT",
    },
  ];

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    asilTarget,
    asilTargets: ["ASIL_B", "ASIL_C", "ASIL_D"],
    redundantCrowbarEnabled,
    spfmPct,
    lfmPct,
    pmhfFit,
    fmedaBadge,
    fmedaTone,
    ftaCutSetBadge,
    columns: FMEDA_COLUMNS,
    rows,
  };
}
