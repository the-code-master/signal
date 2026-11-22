// Module: web/libs/mcad-studio/src/lib/mcad-studio-workbench/mcad-studio-workbench.component.ts — Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigSiInputComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { BrepSheetmetalAnchorsComponent } from "../brep-sheetmetal-anchors/brep-sheetmetal-anchors.component";
import { buildMcadStudioWorkbenchVm } from "./mcad-studio-workbench.logic";

@Component({
  selector: "sig-mcad-studio-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigSiInputComponent,
    DualPlaneViewportComponent,
    BrepSheetmetalAnchorsComponent,
  ],
  templateUrl: "./mcad-studio-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class McadStudioWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly explodedPct = signal<number>(35);
  readonly sectionOffsetNm = signal<bigint>(12_500_000n);

  readonly vm = computed(() =>
    buildMcadStudioWorkbenchVm(
      this.workbench(),
      this.explodedPct(),
      this.sectionOffsetNm(),
    ),
  );

  selectExploded(pct: number): void {
    this.explodedPct.set(pct);
  }

  onSectionOffsetCommitted(offsetNm: bigint): void {
    this.sectionOffsetNm.set(offsetNm);
  }
}
