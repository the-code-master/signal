// Module: web/libs/academy/src/lib/academy-workbench/academy-workbench.component.ts — Academy Workbench Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; wires FEAT-111, FEAT-113 & FEAT-114.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { BreadboardTwinComponent } from "../breadboard-twin/breadboard-twin.component";
import { QuickCalcHudComponent } from "../quick-calc-hud/quick-calc-hud.component";
import type { SchematicInsertPayload } from "../quick-calc-hud/quick-calc-hud.model";
import { VisualFailureFxComponent } from "../visual-failure-fx/visual-failure-fx.component";
import type { AutoFixUpgradeEvent } from "../visual-failure-fx/visual-failure-fx.model";
import { buildAcademyWorkbenchVm } from "./academy-workbench.logic";

@Component({
  selector: "sig-academy-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    BreadboardTwinComponent,
    VisualFailureFxComponent,
    QuickCalcHudComponent,
  ],
  templateUrl: "./academy-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AcademyWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly lastAction = signal<string>("Interactive Sandbox Ready");

  readonly vm = computed(() => buildAcademyWorkbenchVm(this.workbench(), this.lastAction()));

  onBreadboardShortChanged(shorted: boolean): void {
    this.lastAction.set(shorted ? "Breadboard Alert: VCC-to-GND 5-Hole Strip Short!" : "Breadboard Twin Clean");
  }

  onAutoFixed(ev: AutoFixUpgradeEvent): void {
    this.lastAction.set(`Auto-Fixed ${ev.entityId} -> ${ev.upgradeMpn}`);
  }

  onInserted(payload: SchematicInsertPayload): void {
    this.lastAction.set(`Inserted ${payload.calculatorKind}: ${payload.description}`);
  }
}
