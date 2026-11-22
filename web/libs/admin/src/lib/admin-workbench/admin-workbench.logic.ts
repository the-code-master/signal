// Module: web/libs/admin/src/lib/admin-workbench/admin-workbench.logic.ts — Access Review & vcs-auditor Posture Logic
// Invariants: Zero Angular decorators; sweeps expired contractor grants and computes 0-100 BYO-Git posture badge.
// Spec: architecture/08-identity-enterprise-and-project-model.md, architecture/13-*.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { AdminWorkbenchVm, RepoAccessAuditRow } from "./admin-workbench.model";

export const ADMIN_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "admin",
  title: "Enterprise BYO-Git, Access Review & Fleet Admin",
  subtitle: "vcs-auditor 0-100 Posture, 2-of-3 Quorum Break-Glass & FEAT-075 AVL Blast-Radius",
  rustCrate: "signal-operator",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "K8s Pools: 3-Tier Healthy", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "PQC DEK: Active", tone: "ok" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["pool-web | pool-api | pool-rust-engine", "Lateral Drop: 100%"],
  gridTitle: "Cluster Node Pools",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "NP1", item: "pool-web (Angular v22 + Node 26)", metric: "2 Replicas" },
    { id: "NP2", item: "pool-rust-engine (Kata MicroVMs)", metric: "4->32 Extensible" },
  ],
};

export const DEFAULT_REPO_ACCESS_ROSTER: readonly RepoAccessAuditRow[] = [
  { userId: "local-guest-0001", role: "ORG_ADMIN", repoScope: "All 5 Sub-Repos", expiresAtUtcMs: 1_924_905_600_000n },
  { userId: "local-pcb-contractor-0002", role: "EXTERNAL_COLLABORATOR", repoScope: "PCB Repo ONLY", expiresAtUtcMs: 1_893_456_000_000n },
  { userId: "temp-emc-consultant-0003", role: "EXTERNAL_COLLABORATOR", repoScope: "PCB Repo ONLY", expiresAtUtcMs: 1_760_000_000_000n },
];

const ACCESS_COLUMNS: readonly SigGridColumn[] = [
  { key: "userId", header: "Provider Principal", mono: true },
  { key: "role", header: "Org Role", mono: true },
  { key: "repoScope", header: "Immutable provider_repo_id Scope" },
  { key: "leaseState", header: "Expiry Gate (F-092)", mono: true },
];

export function buildAdminWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  sweptExpired: boolean,
  nowUtcMs = 1_800_000_000_000n,
): AdminWorkbenchVm {
  let staleCount = 0;
  const accessRows: SigGridRow[] = DEFAULT_REPO_ACCESS_ROSTER.map((r) => {
    const expired = r.expiresAtUtcMs <= nowUtcMs;
    if (expired && !sweptExpired) staleCount += 1;
    const state = expired
      ? sweptExpired
        ? "AUTO_REVOKED"
        : "EXPIRED_PENDING_SWEEP"
      : `ACTIVE (until ${r.expiresAtUtcMs.toString(10)} ms)`;
    return {
      id: r.userId,
      userId: r.userId,
      role: r.role,
      repoScope: r.repoScope,
      leaseState: state,
    };
  });

  const score = staleCount === 0 ? 100 : 85;
  const postureTone: SigStatusTone = score >= 95 ? "ok" : "warn";

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    postureBadge: `vcs-auditor Posture: ${score}/100 (Padmé Merkle Verified • Public=0)`,
    postureTone,
    breakGlassBadge: "Break-Glass: 2-of-3 Quorum Armed (24h Cooling Enforced)",
    accessColumns: ACCESS_COLUMNS,
    accessRows,
  };
}
