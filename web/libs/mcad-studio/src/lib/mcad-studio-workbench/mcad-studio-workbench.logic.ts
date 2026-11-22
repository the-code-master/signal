// Module: web/libs/mcad-studio/src/lib/mcad-studio-workbench/mcad-studio-workbench.logic.ts — MCAD Host Logic
// Invariants: Zero Angular decorators; owns MCAD_STUDIO_WORKBENCH_DESCRIPTOR and Exploded/Section-Plane math.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md
import type { SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { McadStudioWorkbenchVm } from "./mcad-studio-workbench.model";

export const MCAD_STUDIO_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "mcad-studio",
  title: "Mechanical Parametric B-Rep CAD & Sheet Metal",
  subtitle: "Exact nm B-Rep feature tree, enclosure collision & ECAD anchors (19-*)",
  rustCrate: "signal-mcad",
  subprojectRepo: "aurora-ev-inverter-cad",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Interference: 0 Collisions", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Min Clearance: 2.5 mm", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Housing: Die-Cast AlSi10Mg", "Anchors: 12 M4 Bosses"],
  gridTitle: "Parametric Feature Tree",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "F1", item: "extrude_coldplate_base", metric: "14_000_000 nm" },
    { id: "F2", item: "fillet_seal_groove", metric: "1_500_000 nm" },
  ],
};

export function buildMcadStudioWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  explodedPct: number,
  sectionPlaneOffsetNm: bigint,
): McadStudioWorkbenchVm {
  const clampedExplode = Math.max(0, Math.min(100, explodedPct));
  const minEnclosureClearanceNm = 2_500_000n;
  const collisionCount = 0;
  const clearanceTone: SigStatusTone =
    collisionCount === 0 && minEnclosureClearanceNm >= 1_500_000n ? "ok" : "danger";
  const secMm = (Number(sectionPlaneOffsetNm) / 1_000_000).toFixed(1);
  const viewBadge = `Exploded View: ${clampedExplode}% • Z-Section Plane=${secMm} mm • BVH Collisions=${collisionCount}`;

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    explodedPct: clampedExplode,
    explodedPresets: [0, 35, 75, 100],
    sectionPlaneOffsetNm,
    minEnclosureClearanceNm,
    collisionCount,
    viewBadge,
    clearanceTone,
  };
}
