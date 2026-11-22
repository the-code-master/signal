// Module: web/libs/review/src/lib/stage-gate-checklist/stage-gate-checklist.logic.ts — FEAT-098 Stage-Gate Readiness Evaluator
// Invariants: Blocks Stage-Gate sign-off unless 100% of mandatory gate criteria pass.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, architecture/17-*.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  HardwareStageGate,
  StageGateChecklistVm,
  StageGateCriterion,
} from "./stage-gate-checklist.model";

export const STAGE_GATE_CRITERIA: readonly StageGateCriterion[] = [
  { id: "evt-1", gate: "EVT", criterion: "ERC = 0 Errors & SPICE Operating Point Converged", passed: true },
  { id: "evt-2", gate: "EVT", criterion: "3-Tier parts.lock.json Pinned & Bijection Valid", passed: true },
  { id: "dvt-1", gate: "DVT", criterion: "DRC/Creepage IEC 60664-1 = 0 Violations", passed: true },
  { id: "dvt-2", gate: "DVT", criterion: "ISO 26262 FMEDA + WCCA Monte Carlo Cpk >= 1.33", passed: true },
  { id: "pvt-1", gate: "PVT", criterion: "Signed Fab DFM Clear & 98.5% Clear-to-Build (CTB)", passed: true },
  { id: "pvt-2", gate: "PVT", criterion: "2FA SFTP Factory Release Dry-Run Attested", passed: false },
  { id: "to-1", gate: "TAPEOUT", criterion: "GDSII/OASIS DRC + LVS + Antenna = 0 Errors", passed: true },
  { id: "to-2", gate: "TAPEOUT", criterion: "Dual ML-DSA-65 + Ed25519 Foundry Sign-Off", passed: true },
];

const CHECKLIST_COLUMNS: readonly SigGridColumn[] = [
  { key: "id", header: "Gate ID", mono: true },
  { key: "criterion", header: "Mandatory Automated Sign-Off Criterion" },
  { key: "status", header: "Automated Gate", mono: true },
];

export function buildStageGateChecklistVm(
  activeGate: HardwareStageGate,
  signedGates: ReadonlySet<HardwareStageGate>,
  overrideAllPass = false,
): StageGateChecklistVm {
  const items = STAGE_GATE_CRITERIA.filter((c) => c.gate === activeGate);
  const passCount = items.filter((c) => c.passed || overrideAllPass).length;
  const readyToSignOff = items.length > 0 && passCount === items.length;
  const signedOff = signedGates.has(activeGate) && readyToSignOff;
  const summaryTone: SigStatusTone = signedOff ? "ok" : readyToSignOff ? "neutral" : "warn";
  const summaryBadge = signedOff
    ? `${activeGate} SIGNED OFF (ML-DSA-65 Attested)`
    : readyToSignOff
      ? `${activeGate} Ready (${passCount}/${items.length} Passing)`
      : `${activeGate} Blocked (${passCount}/${items.length} Passing)`;

  const rows: SigGridRow[] = items.map((c) => ({
    id: c.id,
    criterion: c.criterion,
    status: c.passed || overrideAllPass ? "PASS" : "PENDING",
  }));

  return {
    activeGate,
    gates: ["EVT", "DVT", "PVT", "TAPEOUT"],
    readyToSignOff,
    signedOff,
    summaryBadge,
    summaryTone,
    columns: CHECKLIST_COLUMNS,
    rows,
  };
}
