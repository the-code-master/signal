// Module: web/libs/review/src/lib/live-review-deck/live-review-deck.component.ts — FEAT-109 Live Deck Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; zero PPTX/PDF export routes.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildLiveReviewDeckVm, stepSlideIndex } from "./live-review-deck.logic";
import type { ReviewDeckMilestone } from "./live-review-deck.model";

@Component({
  selector: "sig-live-review-deck",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./live-review-deck.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class LiveReviewDeckComponent {
  readonly activeMilestone = signal<ReviewDeckMilestone>("CDR");
  readonly slideIndex = signal<number>(0);

  readonly vm = computed(() =>
    buildLiveReviewDeckVm(this.activeMilestone(), this.slideIndex()),
  );

  selectMilestone(milestone: ReviewDeckMilestone): void {
    this.activeMilestone.set(milestone);
    this.slideIndex.set(0);
  }

  prevSlide(): void {
    this.slideIndex.set(stepSlideIndex(this.vm().slideIndex, -1, this.vm().slides.length));
  }

  nextSlide(): void {
    this.slideIndex.set(stepSlideIndex(this.vm().slideIndex, 1, this.vm().slides.length));
  }
}
