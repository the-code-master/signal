// Module: web/libs/review/src/lib/live-huddle-presence/live-huddle-presence.component.ts — FEAT-094 Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildLiveHuddlePresenceVm } from "./live-huddle-presence.logic";

@Component({
  selector: "sig-live-huddle-presence",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./live-huddle-presence.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class LiveHuddlePresenceComponent {
  readonly followedId = signal<string | null>("peer-lead-ee");
  readonly laserActive = signal<boolean>(true);

  readonly vm = computed(() =>
    buildLiveHuddlePresenceVm(this.followedId(), this.laserActive()),
  );

  toggleFollow(sessionId: string): void {
    this.followedId.update((cur) => (cur === sessionId ? null : sessionId));
  }

  toggleLaser(): void {
    this.laserActive.update((v) => !v);
  }
}
