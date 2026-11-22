// Module: web/libs/library-studio/src/lib/library-studio-workbench/library-studio-workbench.logic.ts — Library Studio Logic
// Invariants: Zero Angular decorators; owns canonical LIBRARY_STUDIO_WORKBENCH_DESCRIPTOR and tab config.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  LibraryStudioTabId,
  LibraryStudioTabOption,
  LibraryStudioWorkbenchVm,
} from "./library-studio-workbench.model";

export const LIBRARY_STUDIO_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "library-studio",
  title: "3-Tier Universal Parts & Padstack Studio",
  subtitle: "Master Pin Table, IEEE+IEC symbols, IPC-7351, 3D STEP & Curve-Fitter (18-*)",
  rustCrate: "signal-parts",
  subprojectRepo: "aurora-ev-inverter-meta",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "Pinned: parts.lock.json", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "5-Gate GOLD: 100%", tone: "ok" },
    { id: "kpi-3", label: "Isolation Cell", value: "Zero NDA Leak", tone: "ok" },
  ],
  hudSignals: ["CAS: BLAKE3 Verified", "AVL: signal-org-parts"],
  gridTitle: "Locked Project Parts",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "P1", item: "urn:signal:part:ti:TPS546D24ARVFR", metric: "GOLD" },
    { id: "P2", item: "urn:signal:part:wolfspeed:C3M0016120K", metric: "GOLD" },
  ],
};

export const LIBRARY_STUDIO_TABS: readonly LibraryStudioTabOption[] = [
  { id: "PIN_SYMBOL", label: "01 • Master Pins & Bank Splitter (FEAT-072)" },
  { id: "PADSTACK_STEP", label: "02 • Padstack, IPC-7351 & 3D STEP (FEAT-071/076)" },
  { id: "CURVE_SDK_SNIPPETS", label: "03 • Curve-Fitter, WASM SDK & Snippets (FEAT-073/074/079)" },
  { id: "DATASHEET_TESTBENCH", label: "04 • Datasheet-to-Testbench (FEAT-107)" },
];

export function buildLibraryStudioWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  activeTab: LibraryStudioTabId,
  lastStampNote: string,
): LibraryStudioWorkbenchVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    activeTab,
    tabs: LIBRARY_STUDIO_TABS,
    lastStampNote,
  };
}
