// Module: web/libs/library-studio/src/lib/library-studio-workbench/library-studio-workbench.model.ts — Library Studio Models
// Invariants: Organizes 18-* Universal Parts, Padstacks, IPC-7351/STEP & Curve-Fitter studios cleanly.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { WorkbenchDescriptor } from "../../../../canvas-core/src/workbench-stage/workbench-stage.model";

export type LibraryStudioTabId =
  | "PIN_SYMBOL"
  | "PADSTACK_STEP"
  | "CURVE_SDK_SNIPPETS"
  | "DATASHEET_TESTBENCH";

export interface LibraryStudioTabOption {
  readonly id: LibraryStudioTabId;
  readonly label: string;
}

export interface LibraryStudioWorkbenchVm {
  readonly descriptor: WorkbenchDescriptor;
  readonly engineBadge: string;
  readonly activeTab: LibraryStudioTabId;
  readonly tabs: readonly LibraryStudioTabOption[];
  readonly lastStampNote: string;
}
