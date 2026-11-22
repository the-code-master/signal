// Module: web/libs/admin/src/lib/avl-where-used-console/avl-where-used-console.logic.ts — FEAT-075 Blast-Radius & ECO Logic
// Invariants: Computes cross-project affected board/instance totals and drafts pin-compatible replacement ECOs.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, architecture/17-*.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { AvlBlastRadiusEntry, AvlWhereUsedVm } from "./avl-where-used-console.model";

export const DEFAULT_AVL_BLAST_CATALOG: readonly AvlBlastRadiusEntry[] = [
  {
    targetMpn: "UCC21750QDWR",
    lifecycleStatus: "NRND",
    dropInMpn: "UCC21750ADWR (Pin-Equal Automotive Gen-2)",
    affectedProjects: [
      { projectSlug: "aurora-ev-inverter", instanceCount: 6, stageGate: "DVT" },
      { projectSlug: "helios-obc-22kw", instanceCount: 4, stageGate: "EVT" },
      { projectSlug: "atlas-solar-mppt", instanceCount: 2, stageGate: "PVT" },
    ],
  },
  {
    targetMpn: "TPS546D24ARVFR",
    lifecycleStatus: "OBSOLETE",
    dropInMpn: "TPS546D24SRVFR (GOLD Pin-Pad-STEP Bijection Verified)",
    affectedProjects: [
      { projectSlug: "aurora-ev-inverter", instanceCount: 2, stageGate: "DVT" },
      { projectSlug: "hyperion-flight-ctrl", instanceCount: 8, stageGate: "EVT" },
    ],
  },
];

const WHERE_USED_COLUMNS: readonly SigGridColumn[] = [
  { key: "projectSlug", header: "Affected Project", mono: true },
  { key: "instances", header: "RefDes Instances", mono: true },
  { key: "stageGate", header: "Stage-Gate", mono: true },
  { key: "replacement", header: "Drop-In Replacement MPN", mono: true },
];

export function buildAvlWhereUsedVm(
  selectedMpn: string,
  ecoDraftedForMpn: string | null,
  catalog: readonly AvlBlastRadiusEntry[] = DEFAULT_AVL_BLAST_CATALOG,
): AvlWhereUsedVm {
  const entry = catalog.find((c) => c.targetMpn === selectedMpn) ?? catalog[0];
  const totalInstances = entry.affectedProjects.reduce((acc, p) => acc + p.instanceCount, 0);
  const blastTone: SigStatusTone = entry.lifecycleStatus === "OBSOLETE" ? "danger" : "warn";
  const ecoDraftBadge =
    ecoDraftedForMpn === entry.targetMpn
      ? `Drafted Cross-Project ECO: ${entry.targetMpn} -> ${entry.dropInMpn}`
      : "ECO Draft: Idle";

  const rows: SigGridRow[] = entry.affectedProjects.map((p) => ({
    id: `${entry.targetMpn}-${p.projectSlug}`,
    projectSlug: p.projectSlug,
    instances: `${p.instanceCount} placements`,
    stageGate: p.stageGate,
    replacement: entry.dropInMpn,
  }));

  return {
    mpns: catalog.map((c) => c.targetMpn),
    selectedEntry: entry,
    blastBadge: `${entry.targetMpn} [${entry.lifecycleStatus}]: ${entry.affectedProjects.length} Projects / ${totalInstances} Placements`,
    blastTone,
    ecoDraftBadge,
    columns: WHERE_USED_COLUMNS,
    rows,
  };
}
