// Module: web/libs/schematic/src/lib/pinswap-equations/pinswap-equations.component.ts — FEAT-008..010 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildPinswapEquationsVm } from "./pinswap-equations.logic";
import type { SpatialSweepDirection } from "./pinswap-equations.model";

@Component({
  selector: "sig-pinswap-equations",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./pinswap-equations.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class PinswapEquationsComponent {
  readonly sweepDirections: readonly SpatialSweepDirection[] = [
    "LEFT_TO_RIGHT_TOP_DOWN",
    "TOP_DOWN_LEFT_TO_RIGHT",
    "HIERARCHICAL_SHEET_BLOCK",
  ];

  readonly acceptedSwaps = signal<boolean>(false);
  readonly sweepDirection = signal<SpatialSweepDirection>("HIERARCHICAL_SHEET_BLOCK");
  readonly equationExpr = signal<string>("r_bot_mohm * (v_out_mv / v_ref_mv - 1)");

  readonly vm = computed(() =>
    buildPinswapEquationsVm(this.acceptedSwaps(), this.sweepDirection(), this.equationExpr()),
  );

  toggleAcceptSwaps(): void {
    this.acceptedSwaps.update((v) => !v);
  }

  selectSweep(direction: SpatialSweepDirection): void {
    this.sweepDirection.set(direction);
  }

  onEquationInput(event: Event): void {
    const target = event.target as HTMLInputElement;
    this.equationExpr.set(target.value);
  }
}
