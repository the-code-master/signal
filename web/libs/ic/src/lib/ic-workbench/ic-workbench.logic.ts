// Module: web/libs/ic/src/lib/ic-workbench/ic-workbench.logic.ts — CU-P6-01 IC / ASIC Workbench Host Logic
// Invariants: Zero Angular decorators; owns canonical IC_WORKBENCH_DESCRIPTOR and studio tab catalog.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { IcStudioTabId, IcStudioTabOption, IcWorkbenchVm } from "./ic-workbench.model";

export const IC_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "ic",
  title: "Analog/Mixed-Signal IC, SoC, UCIe, CPO & Cryo Layout",
  subtitle: "GDSII/OASIS, IEEE 1801 UPF, Pelgrom mismatch, LVF STA/ECO, UCIe Halo & TVLA",
  rustCrate: "signal-ic",
  subprojectRepo: "aurora-ev-inverter-ic-analog",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "LVS & UPF: CLEAN", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "PDK: 22nm BCD HV + 3nm FinFET", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "Signed 5um Halo", tone: "ok" },
  ],
  hudSignals: ["Halo: 5_000 nm", "UPF Domains: 4"],
  gridTitle: "Macro Cells",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "M1", item: "hv_isolated_sigma_delta_mod", metric: "CLEAN" },
    { id: "M2", item: "bandgap_ref_0p5ulp", metric: "CLEAN" },
  ],
};

export const IC_STUDIO_TABS: readonly IcStudioTabOption[] = [
  { id: "ANALOG_GMID_BSPDN", label: "01 • gm/Id, Pelgrom, .lib & BSPDN (FEAT-041..044)" },
  { id: "SOC_STA_UCIE_LITHO", label: "02 • Floorplan, CTS, STA/ECO, UCIe & CMP (FEAT-045..049)" },
  { id: "UPF_CPO_CRYO_SEC", label: "03 • UPF, 193 THz CPO, 4 K Cryo & TVLA (I-07/F-148/050)" },
];

export function buildIcWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  activeTab: IcStudioTabId,
): IcWorkbenchVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    activeTab,
    tabs: IC_STUDIO_TABS,
  };
}
