// Module: web/libs/schematic/src/lib/hierarchy-variants/hierarchy-variants.component.ts — FEAT-005..007 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildHierarchyVariantsVm } from "./hierarchy-variants.logic";
import type { AssemblyVariantId } from "./hierarchy-variants.model";

@Component({
  selector: "sig-hierarchy-variants",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./hierarchy-variants.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class HierarchyVariantsComponent {
  readonly variants: readonly AssemblyVariantId[] = ["PROD_800V", "DEV_BENCH_400V", "COST_REDUCED"];
  readonly repeatOptions: readonly number[] = [2, 3, 6];

  readonly selectedPortalId = signal<string>("p1");
  readonly repeatCount = signal<number>(3);
  readonly activeVariant = signal<AssemblyVariantId>("PROD_800V");
  readonly portalNavigated = output<string>();

  readonly vm = computed(() =>
    buildHierarchyVariantsVm(this.selectedPortalId(), this.repeatCount(), this.activeVariant()),
  );

  selectPortal(id: string): void {
    this.selectedPortalId.set(id);
    this.portalNavigated.emit(id);
  }

  selectRepeatCount(count: number): void {
    this.repeatCount.set(count);
  }

  selectVariant(variant: AssemblyVariantId): void {
    this.activeVariant.set(variant);
  }
}
