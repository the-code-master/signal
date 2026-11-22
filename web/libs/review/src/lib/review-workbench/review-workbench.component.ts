// Module: web/libs/review/src/lib/review-workbench/review-workbench.component.ts — Review Workbench Host Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; wires F-074, FEAT-094, 098, 119.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { ExternalPartnerPortalComponent } from "../external-partner-portal/external-partner-portal.component";
import { LiveHuddlePresenceComponent } from "../live-huddle-presence/live-huddle-presence.component";
import { StageGateChecklistComponent } from "../stage-gate-checklist/stage-gate-checklist.component";
import { buildReviewWorkbenchVm } from "./review-workbench.logic";

@Component({
  selector: "sig-review-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    SigDataGridComponent,
    DualPlaneViewportComponent,
    LiveHuddlePresenceComponent,
    StageGateChecklistComponent,
    ExternalPartnerPortalComponent,
  ],
  templateUrl: "./review-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ReviewWorkbenchComponent {
  readonly workbench = input.required<WorkbenchDescriptor>();
  readonly allResolved = signal<boolean>(false);

  readonly vm = computed(() =>
    buildReviewWorkbenchVm(this.workbench(), this.allResolved()),
  );

  toggleResolveThreads(): void {
    this.allResolved.update((v) => !v);
  }
}
