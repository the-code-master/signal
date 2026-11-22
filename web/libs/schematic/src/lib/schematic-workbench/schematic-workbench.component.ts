// Module: web/libs/schematic/src/lib/schematic-workbench/schematic-workbench.component.ts — Smart Schematic Workbench Host
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; integrates FEAT-001..FEAT-010.
// Spec: architecture/01-product-vision-and-features.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { BeautifyBusComponent } from "../beautify-bus/beautify-bus.component";
import { HierarchyVariantsComponent } from "../hierarchy-variants/hierarchy-variants.component";
import { LiveSimHudComponent } from "../live-sim-hud/live-sim-hud.component";
import { PinswapEquationsComponent } from "../pinswap-equations/pinswap-equations.component";
import { buildSchematicWorkbenchVm } from "./schematic-workbench.logic";
import type { SchematicTabId } from "./schematic-workbench.model";

@Component({
  selector: "sig-schematic-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    BeautifyBusComponent,
    LiveSimHudComponent,
    HierarchyVariantsComponent,
    PinswapEquationsComponent,
  ],
  templateUrl: "./schematic-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class SchematicWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();

  readonly activeTab = signal<SchematicTabId>("BEAUTIFY_BUS");
  readonly beautified = signal<boolean>(false);
  readonly lastPortal = signal<string>("p1");

  readonly vm = computed(() =>
    buildSchematicWorkbenchVm(this.workbench(), this.activeTab(), this.beautified(), this.lastPortal()),
  );

  selectTab(tab: SchematicTabId): void {
    this.activeTab.set(tab);
  }

  onBeautifyApplied(applied: boolean): void {
    this.beautified.set(applied);
  }

  onPortalNavigated(portalId: string): void {
    this.lastPortal.set(portalId);
  }
}
