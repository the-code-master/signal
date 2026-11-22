// Module: web/libs/emc-lab/src/lib/emc-lab-workbench/emc-lab-workbench.logic.ts — FEAT-031/039 Spectrum & Piezo Logic
// Invariants: Evaluates conducted/radiated LISN peaks vs CISPR/FCC masks and 20 Hz..20 kHz MLCC piezo whine risk.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type {
  EmcHarmonicPeak,
  EmcLabWorkbenchVm,
  EmcStandardMask,
} from "./emc-lab-workbench.model";

export const EMC_LAB_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "emc-lab",
  title: "Pre-Compliance EMC / EMI Lab",
  subtitle: "CISPR-25/32 & FCC Part 15 emissions, Middlebrook filter, ESD/TVS, Shield & Piezo",
  rustCrate: "signal-emc",
  subprojectRepo: "aurora-ev-inverter-pcb",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "CISPR-25 Class 5: PASS", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "Margin: +6.4 dBuV", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "1 Project = 1 Cell", tone: "ok" },
  ],
  hudSignals: ["Peak: 31.6 dBuV @ 68 MHz", "CM Choke: Active"],
  gridTitle: "Scan Peaks",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "P1", item: "68.0 MHz Switching Harmonic", metric: "31.6 dBuV" },
    { id: "P2", item: "204.0 MHz Gate Ringing", metric: "27.1 dBuV" },
  ],
};

export const DEFAULT_EMC_PEAKS: readonly EmcHarmonicPeak[] = [
  { peakId: "PK-68M", freqHz: 68_000_000n, measuredDbuv: 31.6, sourceNet: "HV_SW_PHASE_U" },
  { peakId: "PK-180M", freqHz: 180_400_000n, measuredDbuv: 29.2, sourceNet: "GATE_LS_U" },
  { peakId: "PK-408M", freqHz: 408_000_000n, measuredDbuv: 26.5, sourceNet: "CLK_SERDES_REF" },
];

const SPECTRUM_COLUMNS: readonly SigGridColumn[] = [
  { key: "freq", header: "Frequency (FEAT-031)", mono: true },
  { key: "net", header: "Radiating Net", mono: true },
  { key: "level", header: "LISN / 3m Dipole", mono: true },
  { key: "margin", header: "Mask Margin", mono: true },
];

export function isPiezoAcousticAudibleRisk(burstFreqHz: bigint, dielectricClass: "X7R" | "C0G"): boolean {
  if (dielectricClass === "C0G") return false;
  return burstFreqHz >= 20n && burstFreqHz <= 20_000n;
}

export function limitDbuvForStandard(std: EmcStandardMask): number {
  if (std === "CISPR_25_CLASS_5") return 38.0;
  if (std === "CISPR_32_CLASS_B") return 40.0;
  return 43.5;
}

export function buildEmcLabWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  standard: EmcStandardMask,
  pwmBurstFreqHz: bigint,
  peaks: readonly EmcHarmonicPeak[] = DEFAULT_EMC_PEAKS,
): EmcLabWorkbenchVm {
  const limit = limitDbuvForStandard(standard);
  const margins = peaks.map((p) => Number((limit - p.measuredDbuv).toFixed(1)));
  const worstMarginDb = Math.min(...margins);
  const piezoAudibleRisk = isPiezoAcousticAudibleRisk(pwmBurstFreqHz, "X7R");

  const spectrumTone: SigStatusTone =
    worstMarginDb >= 6.0 && !piezoAudibleRisk ? "ok" : worstMarginDb >= 0 ? "warn" : "danger";
  const spectrumBadge = `${standard} Limit=${limit} dBuV • Worst Margin: +${worstMarginDb} dB (FEAT-031)`;
  const piezoBadge = piezoAudibleRisk
    ? `FEAT-039 PIEZO WHINE RISK: X7R MLCC @ ${pwmBurstFreqHz.toString(10)} Hz in 20 Hz..20 kHz Audible Band`
    : `FEAT-039 Piezo Acoustic Safe: Ultrasonic Burst @ ${(pwmBurstFreqHz / 1_000n).toString(10)} kHz (> 20 kHz)`;

  const rows: SigGridRow[] = peaks.map((p) => {
    const m = (limit - p.measuredDbuv).toFixed(1);
    return {
      id: p.peakId,
      freq: `${(Number(p.freqHz) / 1_000_000).toFixed(1)} MHz`,
      net: p.sourceNet,
      level: `${p.measuredDbuv.toFixed(1)} dBuV`,
      margin: `+${m} dB`,
    };
  });

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    standard,
    standards: ["CISPR_25_CLASS_5", "CISPR_32_CLASS_B", "FCC_PART_15B"],
    pwmBurstFreqHz,
    piezoAudibleRisk,
    worstMarginDb,
    spectrumBadge,
    spectrumTone,
    piezoBadge,
    columns: SPECTRUM_COLUMNS,
    rows,
  };
}
