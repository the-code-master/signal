// Module: web/libs/canvas-core/src/viewport/dual-plane-viewport.component.ts — Dual-Plane Zero-Leak Viewport
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; bigint nanometer cursor tracking.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { SigSiInputComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildViewportReticleVm } from "./dual-plane-viewport.logic";

@Component({
  selector: "sig-dual-plane-viewport",
  standalone: true,
  imports: [SigSiInputComponent, SigStatusPillComponent],
  templateUrl: "./dual-plane-viewport.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DualPlaneViewportComponent {
  readonly domainTitle = input.required<string>();
  readonly hudSignals = input<readonly string[]>([]);

  readonly cursorXNm = signal<bigint>(9_007_199_254_740_993n);
  readonly cursorYNm = signal<bigint>(-9_007_199_254_740_995n);
  readonly traceWidthNm = signal<bigint>(85_000n);
  readonly orbiting3d = signal<boolean>(false);

  readonly vm = computed(() =>
    buildViewportReticleVm(
      this.domainTitle(),
      this.cursorXNm(),
      this.cursorYNm(),
      this.traceWidthNm(),
      this.orbiting3d(),
      this.hudSignals(),
    ),
  );

  onPointerMove(event: MouseEvent): void {
    const base = 9_007_199_254_740_000n;
    this.cursorXNm.set(base + BigInt(Math.round(event.offsetX)) * 1_000n);
    this.cursorYNm.set(-base - BigInt(Math.round(event.offsetY)) * 1_000n);
  }

  togglePlaneMode(): void {
    this.orbiting3d.update((v) => !v);
  }

  onWidthCommitted(widthNm: bigint): void {
    this.traceWidthNm.set(widthNm);
  }
}
