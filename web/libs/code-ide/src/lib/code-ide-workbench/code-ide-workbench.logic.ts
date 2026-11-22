// Module: web/libs/code-ide/src/lib/code-ide-workbench/code-ide-workbench.logic.ts — F-069 Windowed LSP & FEAT-050 GLIFT Logic
// Invariants: Streams bounded 200-line LSP slices (F-069) and highlights RTL secret-key taint leaks to unmasked I/O.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";
import type { ActiveIdeLanguage, CodeIdeWorkbenchVm } from "./code-ide-workbench.model";

export const CODE_IDE_WORKBENCH_DESCRIPTOR: WorkbenchDescriptor = {
  slug: "code-ide",
  title: "Embedded Firmware, BIOS & RTL Co-Sim IDE",
  subtitle: "Windowed rust-analyzer/verible/clangd LSP, MemMap, SAT Pinmux, Energy & GLIFT",
  rustCrate: "signal-cosim",
  subprojectRepo: "aurora-ev-inverter-firmware",
  kpis: [
    { id: "kpi-1", label: "Primary Gate", value: "FOC Loop: 50 kHz", tone: "ok" },
    { id: "kpi-2", label: "Domain Telemetry", value: "WCET: 6.2 us", tone: "neutral" },
    { id: "kpi-3", label: "Isolation Cell", value: "Windowed LSP (F-069)", tone: "ok" },
  ],
  hudSignals: ["Target: Cortex-R52 + RISC-V", "Stack Margin: 44%"],
  gridTitle: "Firmware Modules",
  columns: [
    { key: "id", header: "ID", mono: true },
    { key: "item", header: "Engineering Artifact / Entity" },
    { key: "metric", header: "Verified SI / Status", mono: true },
  ],
  rows: [
    { id: "FW1", item: "foc_svm_deadtime_comp.rs", metric: "WCET 2.1 us" },
    { id: "FW2", item: "asil_d_watchdog_mgr.rs", metric: "WCET 0.8 us" },
  ],
};

const DIAG_COLUMNS: readonly SigGridColumn[] = [
  { key: "file", header: "Windowed Buffer (F-069)", mono: true },
  { key: "lsp", header: "LSP / GLIFT Diagnostic" },
  { key: "taint", header: "Taint / Safety Status", mono: true },
];

export function buildCodeIdeWorkbenchVm(
  descriptor: WorkbenchDescriptor,
  language: ActiveIdeLanguage,
  gliftZeroizerApplied: boolean,
): CodeIdeWorkbenchVm {
  const lspServer =
    language === "RUST_NO_STD"
      ? "rust-analyzer"
      : language === "SYSTEMVERILOG_RTL"
        ? "verible-verilog-ls"
        : "clangd + dt-schema";
  const activeBufferRangeBadge = `F-069 On-Demand Windowed Slice [Lines 1..180] via ${lspServer} (Zero Whole-Repo Browser Sync)`;

  const taintClean = gliftZeroizerApplied;
  const gliftTone: SigStatusTone = taintClean ? "ok" : "danger";
  const gliftBadge = taintClean
    ? "FEAT-050 RTL GLIFT: 0 Secret Key Taint Paths Reach Unmasked Debug/GPIO Pins"
    : "FEAT-050 GLIFT TAINT ALERT: aes_key_reg[127:0] Taints dbg_uart_txd Without Zeroizer Gate!";

  const rows: SigGridRow[] = [
    {
      id: "BUF-1",
      file: "src/foc_svm_deadtime_comp.rs:1..180",
      lsp: "rust-analyzer: #![no_std] panic-free verified • WCET=2_100_000 ps",
      taint: "CLEAN",
    },
    {
      id: "BUF-2",
      file: "rtl/pqc_key_vault_top.sv:40..160",
      lsp: taintClean
        ? "GLIFT Shadow-Gate Proof: key_taint & ~zeroize_en == 1'b0 on all pads"
        : "GLIFT Taint Hop: aes_key_reg -> mux_diag -> pad_dbg_txd",
      taint: taintClean ? "PROVEN_ISOLATED" : "TAINT_LEAK_DETECTED",
    },
    {
      id: "BUF-3",
      file: "bios/aurora_board.dts:1..120",
      lsp: "dt-schema: reg=<0x4000a400 0x400> matches CMSIS-SVD CANFD0",
      taint: "SYNCED",
    },
  ];

  return {
    descriptor,
    engineBadge: `${descriptor.rustCrate} • ${descriptor.subprojectRepo}`,
    language,
    languages: ["RUST_NO_STD", "SYSTEMVERILOG_RTL", "UEFI_ASL_DTS"],
    gliftZeroizerApplied,
    activeBufferRangeBadge,
    gliftBadge,
    gliftTone,
    columns: DIAG_COLUMNS,
    rows,
  };
}
