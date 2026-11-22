// Module: web/libs/schematic/src/lib/schematic-workbench/schematic-workbench.logic.ts — Pure Schematic Workbench View-Model Builder
// Invariants: Precomputes tab navigation, descriptor, and status summary without template @if chains or Angular imports.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { SchematicTabId, SchematicTabOption, SchematicWorkbenchVm } from "./schematic-workbench.model";

export const SCHEMATIC_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "schematic",
  title: "Schematic Capture & Hierarchy",
  subtitle: "Multi-sheet hierarchical schematic, bus bundles & ERC",
  rustCrate: "signal-graph",
  subprojectRepo: "aurora-ev-inverter-schematic",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "ERC: 0 Errors", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Sheets: 14", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Net: HV_DC_BUS_800V", "Gate: U4_SIC_HS"],
  gridTitle: "Active Schematic Entities",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "U1", item: "C3M0016120K SiC MOSFET", metric: "800_000_000 uV" },
    { id: "U2", item: "UCC21750QDWR Isolated Gate Driver", metric: "5_700_000_000 uV" },
  ],
};

export const SCHEMATIC_TABS: readonly SchematicTabOption[] = [
  { id: "BEAUTIFY_BUS", label: "01 • Auto-Beautify & Bus Slices" },
  { id: "LIVE_HUD", label: "02 • Live Controls & Current-River" },
  { id: "HIERARCHY_VARIANTS", label: "03 • Portals, Repeat(N) & Variants" },
  { id: "PINSWAP_EQUATIONS", label: "04 • Pin Swap, Reannotate & Equations" },
];

export function buildSchematicWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  activeTab: SchematicTabId,
  beautified: boolean,
  lastNavigatedPortal: string,
): SchematicWorkbenchVm {
  const layoutState = beautified ? "Sugiyama Untangled (2 Crossings)" : "Manual Capture Grid";
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    activeTab,
    tabs: SCHEMATIC_TABS,
    statusBanner: `${layoutState} • Active Portal Focus: ${lastNavigatedPortal}`,
  };
}
