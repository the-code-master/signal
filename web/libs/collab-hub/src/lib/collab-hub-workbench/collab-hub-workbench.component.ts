// Module: web/libs/collab-hub/src/lib/collab-hub-workbench/collab-hub-workbench.component.ts — Collab Workbench Host
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; integrates 5-Team Channels, Status Tower & Auto-Reviews.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { DualPlaneViewportComponent, type WorkbenchDescriptor } from "@signal/canvas-core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { AutomatedReviewStudioComponent } from "../automated-review-studio/automated-review-studio.component";
import { ProjectStatusTowerComponent } from "../project-status-tower/project-status-tower.component";
import { TeamChannelsHuddleComponent } from "../team-channels-huddle/team-channels-huddle.component";
import type { CanvasJumpEvent } from "../team-channels-huddle/team-channels-huddle.model";
import {
  COLLAB_HUB_WORKBENCH_DESCRIPTOR,
  buildCollabHubWorkbenchVm,
} from "./collab-hub-workbench.logic";

@Component({
  selector: "sig-collab-hub-workbench",
  standalone: true,
  imports: [
    SigPanelComponent,
    SigStatusPillComponent,
    DualPlaneViewportComponent,
    TeamChannelsHuddleComponent,
    ProjectStatusTowerComponent,
    AutomatedReviewStudioComponent,
  ],
  templateUrl: "./collab-hub-workbench.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class CollabHubWorkbenchComponent {
  readonly workbench = input<WorkbenchDescriptor>(COLLAB_HUB_WORKBENCH_DESCRIPTOR);
  readonly lastJump = signal<CanvasJumpEvent | null>(null);

  readonly vm = computed(() => buildCollabHubWorkbenchVm(this.workbench(), this.lastJump()));

  handleCanvasJump(evt: CanvasJumpEvent): void {
    this.lastJump.set(evt);
  }
}
