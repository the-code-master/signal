// Module: web/libs/academy/src/lib/breadboard-twin/breadboard-twin.component.ts — FEAT-111 Breadboard Twin Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildBreadboardTwinVm } from "./breadboard-twin.logic";

@Component({
  selector: "sig-breadboard-twin",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./breadboard-twin.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class BreadboardTwinComponent {
  readonly shortInjected = signal<boolean>(false);
  readonly shortStateChanged = output<boolean>();

  readonly vm = computed(() => buildBreadboardTwinVm(this.shortInjected()));

  toggleShortTest(): void {
    const next = !this.shortInjected();
    this.shortInjected.set(next);
    this.shortStateChanged.emit(next);
  }
}
