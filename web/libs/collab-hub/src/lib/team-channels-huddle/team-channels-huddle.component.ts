// Module: web/libs/collab-hub/src/lib/team-channels-huddle/team-channels-huddle.component.ts — Channels & Huddle UI
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; emits bigint CanvasJumpEvent on authorized @entity click.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import {
  SEEDED_RAW_MESSAGES,
  buildTeamChannelsHuddleVm,
  createHuddleTicketState,
  toCanvasJumpEvent,
} from "./team-channels-huddle.logic";
import type {
  CanvasJumpEvent,
  CollabEntityAnchor,
  CollabThreadMessage,
  CollabViewerRole,
} from "./team-channels-huddle.model";

@Component({
  selector: "sig-team-channels-huddle",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./team-channels-huddle.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class TeamChannelsHuddleComponent {
  readonly jumpToCanvas = output<CanvasJumpEvent>();

  readonly viewerRole = signal<CollabViewerRole>("ORG_ADMIN");
  readonly selectedChannelId = signal<string>("ch-all-hands");
  readonly messages = signal<readonly CollabThreadMessage[]>(SEEDED_RAW_MESSAGES);
  readonly huddleActive = signal<boolean>(false);
  readonly followPresenter = signal<boolean>(true);

  readonly huddleState = computed(() =>
    createHuddleTicketState(
      this.selectedChannelId(),
      this.huddleActive(),
      this.followPresenter(),
      1_795_324_800_000n,
    ),
  );

  readonly vm = computed(() =>
    buildTeamChannelsHuddleVm(
      this.viewerRole(),
      this.selectedChannelId(),
      this.messages(),
      this.huddleState(),
    ),
  );

  selectRole(role: CollabViewerRole): void {
    this.viewerRole.set(role);
  }

  selectChannel(channelId: string): void {
    this.selectedChannelId.set(channelId);
  }

  toggleHuddle(): void {
    this.huddleActive.update((v) => !v);
  }

  toggleFollowPresenter(): void {
    this.followPresenter.update((v) => !v);
  }

  triggerJump(anchor: CollabEntityAnchor): void {
    const evt = toCanvasJumpEvent(anchor);
    if (evt !== null) {
      this.jumpToCanvas.emit(evt);
    }
  }
}
