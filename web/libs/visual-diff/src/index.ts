// Module: web/libs/visual-diff/src/index.ts — Public Barrel for @signal/visual-diff
// Invariants: Sole public entrypoint for lazy route loading and FEAT-097 Onion-Skin / History Scrubber tools.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { HistoryTimeScrubberComponent } from "./lib/history-time-scrubber/history-time-scrubber.component";
export { buildHistoryTimeScrubberVm } from "./lib/history-time-scrubber/history-time-scrubber.logic";
export type {
  HistoryTimeScrubberVm,
  SuperCommitCheckpoint,
} from "./lib/history-time-scrubber/history-time-scrubber.model";
export { VisualDiffWorkbenchComponent } from "./lib/visual-diff-workbench/visual-diff-workbench.component";
export { buildVisualDiffWorkbenchVm } from "./lib/visual-diff-workbench/visual-diff-workbench.logic";
export type {
  DiffDomainFilter,
  SemanticDiffRecord,
  VisualDiffWorkbenchVm,
} from "./lib/visual-diff-workbench/visual-diff-workbench.model";
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./visual-diff.routes";
