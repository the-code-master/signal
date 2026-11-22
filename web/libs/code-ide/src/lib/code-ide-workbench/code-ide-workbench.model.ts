// Module: web/libs/code-ide/src/lib/code-ide-workbench/code-ide-workbench.model.ts — F-069 & FEAT-050 Code IDE Models
// Invariants: Models on-demand windowed LSP buffers (zero whole-repo browser clone) and RTL GLIFT taint paths.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type ActiveIdeLanguage = "RUST_NO_STD" | "SYSTEMVERILOG_RTL" | "UEFI_ASL_DTS";

export interface CodeIdeWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly language: ActiveIdeLanguage;
  readonly languages: readonly ActiveIdeLanguage[];
  readonly gliftZeroizerApplied: boolean;
  readonly activeBufferRangeBadge: string;
  readonly gliftBadge: string;
  readonly gliftTone: SigStatusTone;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
