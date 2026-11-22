// Module: web/libs/visual-diff/src/lib/visual-diff-workbench/visual-diff-workbench.logic.ts — Onion-Skin Semantic Diff Logic
// Invariants: Zero Angular decorators; filters multi-domain semantic diffs in bigint nanometers.
// Spec: architecture/13-vcs-git-server-and-storage-analysis.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigGridColumn, SigGridRow } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  DiffDomainFilter,
  SemanticDiffRecord,
  VisualDiffWorkbenchVm,
} from "./visual-diff-workbench.model";

export const VISUAL_DIFF_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "visual-diff",
  title: "Topological Schematic, PCB & 3D B-Rep Diff",
  subtitle: "Semantic Onion-Skin entity diff across CoW branches & Super-Commits",
  rustCrate: "signal-graph",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Delta Ops: 4", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Rail Shorts: 0", tone: "ok" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Base: main@100", "Head: main@101"],
  gridTitle: "Changed Entities",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "D1", item: "trace_pcie6_tx0_p", metric: "+85_000 nm route" },
    { id: "D2", item: "heatsink_fin_pitch", metric: "4_500_000 nm" },
  ],
};

export const DEFAULT_SEMANTIC_DIFFS: readonly SemanticDiffRecord[] = [
  { id: "D1", domain: "SCHEMATIC", entityId: "R_GATE_U1", changeKind: "MODIFIED", deltaMetricNm: 0n, summary: "2.2 Ω -> 3.3 Ω Kelvin damping" },
  { id: "D2", domain: "PCB", entityId: "trace_kelvin_src_u", changeKind: "MODIFIED", deltaMetricNm: 85_000n, summary: "+85_000 nm length-matched meander" },
  { id: "D3", domain: "CAD_MCAD", entityId: "heatsink_fin_pitch", changeKind: "MODIFIED", deltaMetricNm: 4_500_000n, summary: "B-Rep fin pitch -> 4_500_000 nm" },
  { id: "D4", domain: "PCB", entityId: "via_stitch_hv_guard", changeKind: "ADDED", deltaMetricNm: 300_000n, summary: "Added guard via ring (300_000 nm)" },
];

const DIFF_COLUMNS: readonly SigGridColumn[] = [
  { key: "domain", header: "Domain", mono: true },
  { key: "entityId", header: "Entity", mono: true },
  { key: "changeKind", header: "Delta Kind", mono: true },
  { key: "summary", header: "Semantic Engineering Delta", mono: true },
];

export function buildVisualDiffWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  activeDomain: DiffDomainFilter,
  activeRevision: bigint,
): VisualDiffWorkbenchVm {
  const filtered =
    activeDomain === "ALL"
      ? DEFAULT_SEMANTIC_DIFFS
      : DEFAULT_SEMANTIC_DIFFS.filter((d) => d.domain === activeDomain);
  const diffRows: SigGridRow[] = filtered.map((d) => ({
    id: d.id,
    domain: d.domain,
    entityId: d.entityId,
    changeKind: d.changeKind,
    summary: d.summary,
  }));
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    activeDomain,
    domains: ["ALL", "SCHEMATIC", "PCB", "CAD_MCAD"],
    activeRevisionText: `Viewing Super-Commit @${activeRevision.toString(10)} (${diffRows.length} Entities)`,
    diffColumns: DIFF_COLUMNS,
    diffRows,
  };
}
