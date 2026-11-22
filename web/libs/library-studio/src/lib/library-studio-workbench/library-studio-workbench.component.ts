// Module: web/libs/library-studio/src/lib/library-studio-workbench/library-studio-workbench.component.ts — Library Studio Host
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; wires CU-P2-05 studios.
// Spec: architecture/18-universal-component-and-datasheet-repository.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { CurveSdkSnippetsComponent } from "../curve-sdk-snippets/curve-sdk-snippets.component";
import { DatasheetTestbenchComponent } from "../datasheet-testbench/datasheet-testbench.component";
import { PadstackFootprintStepComponent } from "../padstack-footprint-step/padstack-footprint-step.component";
import { PinSymbolSplitterComponent } from "../pin-symbol-splitter/pin-symbol-splitter.component";
import { buildLibraryStudioWorkbenchVm } from "./library-studio-workbench.logic";
import type { LibraryStudioTabId } from "./library-studio-workbench.model";

@Component({
  selector: "sig-library-studio-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    PinSymbolSplitterComponent,
    PadstackFootprintStepComponent,
    CurveSdkSnippetsComponent,
    DatasheetTestbenchComponent,
  ],
  templateUrl: "./library-studio-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class LibraryStudioWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();

  readonly activeTab = signal<LibraryStudioTabId>("PIN_SYMBOL");
  readonly lastStampNote = signal<string>("3-Tier Lockfile Verified (parts.lock.json)");

  readonly vm = computed(() =>
    buildLibraryStudioWorkbenchVm(this.workbench(), this.activeTab(), this.lastStampNote()),
  );

  selectTab(tab: LibraryStudioTabId): void {
    this.activeTab.set(tab);
  }

  onSnippetStamped(note: string): void {
    this.lastStampNote.set(`Stamped ${note}`);
  }
}
