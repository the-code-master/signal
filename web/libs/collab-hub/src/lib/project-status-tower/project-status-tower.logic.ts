// Module: web/libs/collab-hub/src/lib/project-status-tower/project-status-tower.logic.ts — 5-Team RAG & Dependency Logic
// Invariants: Pure deterministic functions <=250 LOC; escalates overdue dependencies to BLOCKED & propagates RED RAG.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { CollabTeamKind } from "../team-channels-huddle/team-channels-huddle.model";
import type {
  CrossTeamDependencyItem,
  DependencyHandshakeKind,
  ProjectStatusTowerVm,
  RagHealthKind,
  SignedDecisionEntry,
  StageGateMilestoneItem,
  TeamRagHealthCard,
} from "./project-status-tower.model";

export const AT_RISK_HORIZON_MS = 172_800_000n;

export const FIVE_ENGINEERING_TEAMS: readonly CollabTeamKind[] = [
  "DESIGN",
  "DEVELOPMENT_BRINGUP",
  "SOFTWARE_FIRMWARE",
  "MANUFACTURING",
  "OPERATIONS_SUPPLY_CHAIN",
];

export const SEEDED_MILESTONES: readonly StageGateMilestoneItem[] = [
  { gateId: "gate-evt", gateName: "EVT", targetDateUtcMs: 1_790_000_000_000n, health: "GREEN", openBlockerCount: 0 },
  { gateId: "gate-dvt", gateName: "DVT", targetDateUtcMs: 1_796_000_000_000n, health: "GREEN", openBlockerCount: 0 },
  { gateId: "gate-pvt", gateName: "PVT", targetDateUtcMs: 1_801_000_000_000n, health: "GREEN", openBlockerCount: 0 },
  { gateId: "gate-mp", gateName: "MP", targetDateUtcMs: 1_808_000_000_000n, health: "GREEN", openBlockerCount: 0 },
];

export const SEEDED_DEPENDENCIES: readonly CrossTeamDependencyItem[] = [
  {
    dependencyId: "dep-01",
    producerTeam: "DESIGN",
    consumerTeam: "SOFTWARE_FIRMWARE",
    deliverableTitle: "Frozen SiC Gate-Driver PinMux & CMSIS-SVD",
    dueAtUtcMs: 1_796_500_000_000n,
    handshakeStatus: "ACCEPTED",
  },
  {
    dependencyId: "dep-02",
    producerTeam: "DESIGN",
    consumerTeam: "MANUFACTURING",
    deliverableTitle: "Cold-Plate Boss CNC Tolerance STEP Rev D",
    dueAtUtcMs: 1_795_400_000_000n,
    handshakeStatus: "PROPOSED",
  },
];

export const SEEDED_DECISIONS: readonly SignedDecisionEntry[] = [
  {
    decisionId: "adr-001",
    recordKind: "ADR",
    title: "Adopt 800V Creepage Slotting Under High-Side Gate Driver",
    rationale: "Maintains IEC 60664-1 Pollution Degree 2 margin >= 8.0 mm across PCB rev D.",
    decidedAtUtcMs: 1_795_100_000_000n,
    mlDsa65Signature: "ml-dsa-65:9f4a8b12c701e3d4456a7890",
  },
];

export const MILESTONE_COLUMNS: readonly SigGridColumn[] = [
  { key: "gate", header: "Stage Gate", mono: true },
  { key: "targetUtcMs", header: "Target Epoch (ms)", mono: true },
  { key: "health", header: "RAG Health", mono: true },
  { key: "blockers", header: "Open Blockers", mono: true },
];

export const DEPENDENCY_COLUMNS: readonly SigGridColumn[] = [
  { key: "id", header: "Dep ID", mono: true },
  { key: "flow", header: "Producer -> Consumer", mono: true },
  { key: "deliverable", header: "Deliverable Handshake" },
  { key: "status", header: "Evaluated Status", mono: true },
];

export const DECISION_COLUMNS: readonly SigGridColumn[] = [
  { key: "id", header: "Record", mono: true },
  { key: "title", header: "Immutable ADR / EDR Decision" },
  { key: "signature", header: "Hybrid PQC Signature", mono: true },
];

export function ragToTone(rag: RagHealthKind): SigStatusTone {
  if (rag === "RED") {
    return "danger";
  }
  if (rag === "AMBER") {
    return "warn";
  }
  return "ok";
}

export function evaluateDependencyHandshake(
  dep: CrossTeamDependencyItem,
  nowUtcMs: bigint,
): DependencyHandshakeKind {
  if (dep.handshakeStatus === "ACCEPTED" || dep.handshakeStatus === "BLOCKED") {
    return dep.handshakeStatus;
  }
  if (nowUtcMs > dep.dueAtUtcMs) {
    return "BLOCKED";
  }
  if (dep.dueAtUtcMs - nowUtcMs <= AT_RISK_HORIZON_MS) {
    return "AT_RISK";
  }
  return dep.handshakeStatus;
}

export function buildProjectStatusTowerVm(
  dependencies: readonly CrossTeamDependencyItem[],
  decisions: readonly SignedDecisionEntry[],
  nowUtcMs: bigint,
  fwOverrideRationale: string | null = null,
): ProjectStatusTowerVm {
  const evaluatedDeps: CrossTeamDependencyItem[] = dependencies.map((d) => ({
    ...d,
    handshakeStatus: evaluateDependencyHandshake(d, nowUtcMs),
  }));

  const teamCards: TeamRagHealthCard[] = FIVE_ENGINEERING_TEAMS.map((team) => {
    const blockers = evaluatedDeps.filter(
      (d) => d.consumerTeam === team && d.handshakeStatus === "BLOCKED",
    ).length;
    const warnings = evaluatedDeps.filter(
      (d) => d.consumerTeam === team && d.handshakeStatus === "AT_RISK",
    ).length;
    const autoHealth: RagHealthKind = blockers > 0 ? "RED" : warnings > 0 ? "AMBER" : "GREEN";
    const hasOverride = team === "SOFTWARE_FIRMWARE" && fwOverrideRationale !== null && fwOverrideRationale.trim().length >= 8;
    const effectiveHealth: RagHealthKind = hasOverride ? "AMBER" : autoHealth;
    return {
      team,
      autoHealth,
      effectiveHealth,
      tone: ragToTone(effectiveHealth),
      openBlockers: blockers,
      openWarnings: warnings,
      overrideRationale: hasOverride ? fwOverrideRationale : null,
    };
  });

  const overallHealth: RagHealthKind = teamCards.some((c) => c.effectiveHealth === "RED")
    ? "RED"
    : teamCards.some((c) => c.effectiveHealth === "AMBER")
      ? "AMBER"
      : "GREEN";
  const totalBlockers = teamCards.reduce((acc, c) => acc + c.openBlockers, 0);

  const milestoneRows: SigGridRow[] = SEEDED_MILESTONES.map((m) => ({
    id: m.gateId,
    gate: m.gateName,
    targetUtcMs: m.targetDateUtcMs.toString(),
    health: m.gateName === "DVT" ? overallHealth : m.health,
    blockers: (m.gateName === "DVT" ? totalBlockers : m.openBlockerCount).toString(),
  }));

  const dependencyRows: SigGridRow[] = evaluatedDeps.map((d) => ({
    id: d.dependencyId,
    flow: `${d.producerTeam} -> ${d.consumerTeam}`,
    deliverable: d.deliverableTitle,
    status: d.handshakeStatus,
  }));

  const decisionRows: SigGridRow[] = decisions.map((rec) => ({
    id: `${rec.recordKind}:${rec.decisionId}`,
    title: rec.title,
    signature: rec.mlDsa65Signature,
  }));

  return {
    overallHealth,
    overallTone: ragToTone(overallHealth),
    overallBadge: `Program RAG: ${overallHealth} (${totalBlockers} Blockers)`,
    teamCards,
    milestoneColumns: MILESTONE_COLUMNS,
    milestoneRows,
    dependencyColumns: DEPENDENCY_COLUMNS,
    dependencyRows,
    decisionColumns: DECISION_COLUMNS,
    decisionRows,
  };
}
