// Module: web/libs/fpga-studio/src/lib/fpga-studio-workbench/fpga-studio-workbench.logic.ts — FPGA Host Logic
// Invariants: Zero Angular decorators; owns FPGA_STUDIO_WORKBENCH_DESCRIPTOR and device preset catalog.
// Spec: architecture/03-rust-eda-engine.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { FpgaPartPreset, FpgaStudioWorkbenchVm } from "./fpga-studio-workbench.model";

export const FPGA_STUDIO_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "fpga-studio",
  title: "FPGA Die Floorplan, SDC/XDC, FSM & DSP Studio",
  subtitle: "Bank I/O standards, PCB ratline cross-probe, CDC & Q(m.n) DSP",
  rustCrate: "signal-fpga",
  subprojectRepo: "aurora-ev-inverter-fpga",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "WNS: +412_000 fs", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "CDC: 0 Unsafe", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Device: Kintex UltraScale+", "PWM Res: 125 ps"],
  gridTitle: "I/O Banks",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "B64", item: "LVDS_1V8 Resolver Bus", metric: "Matched +-250 fs" },
    { id: "B65", item: "LVCMOS_3V3 Gate Faults", metric: "Verified" },
  ],
};

export function buildFpgaStudioWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  partPreset: FpgaPartPreset,
): FpgaStudioWorkbenchVm {
  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    partPreset,
    partPresets: ["XCKU5P_ULTRASCALE_PLUS", "AGFB014_AGILEX7", "LFD2NX_CERTUS_NX"],
    deviceSummaryBadge: `Active Target Die: ${partPreset} • Yosys/nextpnr + Vendor SDC Bridge Synced`,
  };
}
