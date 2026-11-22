// Module: web/libs/cam-print/src/lib/direct-brep-slicer/direct-brep-slicer.component.ts — Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/19-cad-cam-3d-printing-and-plm-operations.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildDirectBrepSlicerVm } from "./direct-brep-slicer.logic";
import type { AdditiveProcessMode } from "./direct-brep-slicer.model";

@Component({
  selector: "sig-direct-brep-slicer",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./direct-brep-slicer.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DirectBrepSlicerComponent {
  readonly process = signal<AdditiveProcessMode>("DMLS_METAL_LPBF");
  readonly treeSupports = signal<boolean>(true);
  readonly thermalWarpPrecomp = signal<boolean>(true);

  readonly vm = computed(() =>
    buildDirectBrepSlicerVm(
      this.process(),
      this.treeSupports(),
      this.thermalWarpPrecomp(),
    ),
  );

  selectProcess(proc: AdditiveProcessMode): void {
    this.process.set(proc);
  }

  toggleTreeSupports(): void {
    this.treeSupports.update((v) => !v);
  }

  toggleThermalWarp(): void {
    this.thermalWarpPrecomp.update((v) => !v);
  }
}
