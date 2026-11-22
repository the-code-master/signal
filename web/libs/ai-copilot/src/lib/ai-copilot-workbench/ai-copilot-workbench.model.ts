// Module: web/libs/ai-copilot/src/lib/ai-copilot-workbench/ai-copilot-workbench.model.ts — CoW Copilot, ProofBundle & I-10 Import Models
// Invariants: Archive byte sizes and delta durations stay in ES2020 bigint; bomb-cap enforces <=20:1 ratio & <=64 depth.
// Spec: architecture/05-ai-agent-and-datasheet-rag.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type LegacyImportFormat =
  | "ALTIUM_DOC"
  | "KICAD_SEXPR"
  | "ORCAD_ALLEGRO"
  | "GDSII_STREAM"
  | "WHITEBOARD_VLM_QUARANTINE";

export type MainBranchDriftState = "UNCHANGED" | "ADVANCED_CLEAN" | "ADVANCED_CONFLICT";

export interface CleanRoomImportValidation {
  readonly format: LegacyImportFormat;
  readonly ratioWithinCap: boolean;
  readonly depthWithinCap: boolean;
  readonly netlistBijectionVerified: boolean;
  readonly accepted: boolean;
  readonly summaryBadge: string;
  readonly tone: SigStatusTone;
}

export interface ProofCertificateState {
  readonly proofBundleId: string;
  readonly cowBranchId: string;
  readonly mergeAllowed: boolean;
  readonly deltaReverified: boolean;
  readonly deltaDurationPs: bigint;
  readonly badgeText: string;
  readonly tone: SigStatusTone;
}

export interface AiCopilotWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly formats: readonly LegacyImportFormat[];
  readonly driftStates: readonly MainBranchDriftState[];
  readonly activeFormat: LegacyImportFormat;
  readonly activeDrift: MainBranchDriftState;
  readonly proof: ProofCertificateState;
  readonly importCheck: CleanRoomImportValidation;
  readonly cowColumns: readonly SigGridColumn[];
  readonly cowRows: readonly SigGridRow[];
}
