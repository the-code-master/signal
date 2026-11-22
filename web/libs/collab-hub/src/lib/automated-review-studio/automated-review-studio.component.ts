// Module: web/libs/collab-hub/src/lib/automated-review-studio/automated-review-studio.component.ts — Auto-Review Studio UI
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; Zero-Leak presentation deck without export/download/print.
// Spec: architecture/21-collaboration-space-project-status-and-automated-reviews.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import {
  buildAutomatedReviewStudioVm,
  compileDeterministicReviewPacket,
  signOffStageGatePacket,
} from "./automated-review-studio.logic";
import type { CompiledReviewPacket, ReviewCadenceKind } from "./automated-review-studio.model";

@Component({
  selector: "sig-automated-review-studio",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./automated-review-studio.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AutomatedReviewStudioComponent {
  readonly packet = signal<CompiledReviewPacket>(
    compileDeterministicReviewPacket("WEEKLY_ENGINEERING", 1_795_324_800_000n),
  );
  readonly presentationMode = signal<boolean>(false);
  readonly slideIndex = signal<number>(0);
  readonly burnedNonces = signal<readonly string[]>([]);
  readonly quorumSeq = signal<bigint>(1n);
  readonly lastStatus = signal<string>("Ready for deterministic compilation or Stage-Gate sign-off.");

  readonly vm = computed(() =>
    buildAutomatedReviewStudioVm(
      this.packet(),
      this.presentationMode(),
      this.slideIndex(),
      this.lastStatus(),
    ),
  );

  generateCadence(cadence: ReviewCadenceKind): void {
    this.packet.set(compileDeterministicReviewPacket(cadence, 1_795_324_800_000n));
    this.slideIndex.set(0);
    this.lastStatus.set(`Compiled ${cadence} packet from 100% deterministic Rust + PLM KPIs.`);
  }

  togglePresentationDeck(): void {
    this.presentationMode.update((v) => !v);
  }

  nextSlide(): void {
    this.slideIndex.update((idx) => (idx + 1) % this.packet().slides.length);
  }

  executeFreshSignOff(): void {
    const freshNonce = `stepup-fido2-${this.quorumSeq().toString()}-9901`;
    const res = signOffStageGatePacket(
      this.packet(),
      freshNonce,
      "ml-dsa-65:approver-lead-445566778899",
      3,
      this.burnedNonces(),
      this.quorumSeq(),
    );
    this.packet.set(res.packet);
    this.burnedNonces.set(res.burnedNonces);
    this.quorumSeq.update((s) => s + 1n);
    this.lastStatus.set(res.statusMessage);
  }
}
