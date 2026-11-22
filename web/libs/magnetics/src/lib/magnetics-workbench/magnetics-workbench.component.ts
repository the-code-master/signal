// Module: web/libs/magnetics/src/lib/magnetics-workbench/magnetics-workbench.component.ts — FEAT-021 Magnetics Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import {
  SigDataGridComponent,
  SigPanelComponent,
  SigSiInputComponent,
  SigStatusPillComponent,
} from "@signal/ui-kit";
import { buildMagneticsWorkbenchVm } from "./magnetics-workbench.logic";
import type { WindingGeometryKind } from "./magnetics-workbench.model";

@Component({
  selector: "sig-magnetics-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigSiInputComponent,
    SigDataGridComponent,
    DualPlaneViewportComponent,
  ],
  templateUrl: "./magnetics-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MagneticsWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly windingKind = signal<WindingGeometryKind>("PLANAR_HEAVY_CU_INTERLEAVED");
  readonly airGapNm = signal<bigint>(400_000n);

  readonly vm = computed(() =>
    buildMagneticsWorkbenchVm(this.workbench(), this.windingKind(), this.airGapNm()),
  );

  selectWinding(kind: WindingGeometryKind): void {
    this.windingKind.set(kind);
  }

  onGapCommitted(gapNm: bigint): void {
    this.airGapNm.set(gapNm);
  }
}
