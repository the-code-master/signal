// Module: web/libs/ai-copilot/src/index.ts — Public Barrel for @signal/ai-copilot
// Invariants: Sole public entrypoint for lazy route loading and CU-P3-03 AI Copilot & Clean-Room Import tools.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./ai-copilot.routes";
export { AiCopilotWorkbenchComponent } from "./lib/ai-copilot-workbench/ai-copilot-workbench.component";
export {
  buildAiCopilotWorkbenchVm,
  evaluateProofMergeState,
  validateCleanRoomImport,
} from "./lib/ai-copilot-workbench/ai-copilot-workbench.logic";
export type {
  AiCopilotWorkbenchVm,
  CleanRoomImportValidation,
  LegacyImportFormat,
  MainBranchDriftState,
  ProofCertificateState,
} from "./lib/ai-copilot-workbench/ai-copilot-workbench.model";
