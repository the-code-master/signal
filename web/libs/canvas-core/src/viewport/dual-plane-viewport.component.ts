// Module: web/libs/canvas-core/src/viewport/dual-plane-viewport.component.ts — Dual-Plane Zero-Leak Viewport
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS; volatile closure op buffer & *.sigclip seal.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
import { ChangeDetectionStrategy, Component, computed, input, signal } from "@angular/core";
import { SigSiInputComponent, SigStatusPillComponent } from "@signal/ui-kit";
import {
  advanceSettlingStateMachine,
  createInitialSettlingState,
  onMotionInputSample,
} from "../dual-plane-client/dual-plane-client.logic";
import type { PlaneSettlingState } from "../dual-plane-client/dual-plane-client.model";
import { DlpGuardComponent } from "../lib/dlp-guard/dlp-guard.component";
import { createVolatileOpBuffer, sealCloudClipboardEnvelope } from "../volatile-reconnect/volatile-reconnect.logic";
import type { CloudClipboardEnvelope } from "../volatile-reconnect/volatile-reconnect.model";
import { buildViewportReticleVm } from "./dual-plane-viewport.logic";

@Component({
  selector: "sig-dual-plane-viewport",
  standalone: true,
  imports: [SigSiInputComponent, SigStatusPillComponent, DlpGuardComponent],
  templateUrl: "./dual-plane-viewport.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DualPlaneViewportComponent {
  private readonly volatileBuffer = createVolatileOpBuffer(60_000n, 256);
  private opSeq = 0;

  readonly domainTitle = input.required<string>();
  readonly hudSignals = input<readonly string[]>([]);

  readonly cursorXNm = signal<bigint>(9_007_199_254_740_993n);
  readonly cursorYNm = signal<bigint>(-9_007_199_254_740_995n);
  readonly traceWidthNm = signal<bigint>(85_000n);
  readonly orbiting3d = signal<boolean>(false);
  readonly settlingState = signal<PlaneSettlingState>(createInitialSettlingState(80n));
  readonly pendingVolatileOps = signal<number>(0);
  readonly clipboardEnvelope = signal<CloudClipboardEnvelope | null>(null);
  readonly frameTick120Hz = signal<number>(12);

  readonly vm = computed(() =>
    buildViewportReticleVm({
      domainTitle: this.domainTitle(),
      cursorXNm: this.cursorXNm(),
      cursorYNm: this.cursorYNm(),
      traceWidthNm: this.traceWidthNm(),
      orbiting3d: this.orbiting3d(),
      hudSignals: this.hudSignals(),
      settlingState: this.settlingState(),
      pendingVolatileOps: this.pendingVolatileOps(),
      clipboardEnvelope: this.clipboardEnvelope(),
      frameTick120Hz: this.frameTick120Hz(),
    }),
  );

  onPointerMove(event: MouseEvent): void {
    const base = 9_007_199_254_740_000n;
    const nowMs = BigInt(Date.now());
    this.cursorXNm.set(base + BigInt(Math.round(event.offsetX)) * 1_000n);
    this.cursorYNm.set(-base - BigInt(Math.round(event.offsetY)) * 1_000n);
    this.frameTick120Hz.update((t) => (t + 1) % 120);
    this.settlingState.update((s) => onMotionInputSample(s, nowMs, true));
  }

  togglePlaneMode(): void {
    this.orbiting3d.update((v) => !v);
  }

  settleLosslessNow(): void {
    const settledAtMs = this.settlingState().lastMotionUtcMs + 85n;
    this.settlingState.update((s) => advanceSettlingStateMachine(s, settledAtMs, s.settledFrameSeq + 1n));
  }

  onWidthCommitted(widthNm: bigint): void {
    this.traceWidthNm.set(widthNm);
    this.opSeq += 1;
    const nowMs = BigInt(Date.now());
    const count = this.volatileBuffer.enqueue(
      {
        idempotencyKey: `idem-width-${this.opSeq}`,
        opId: `op-${this.opSeq}`,
        kind: "UPDATE_PARAMETER",
        entityId: "CONSTRAINT_TRACE_WIDTH",
        enqueuedAtUtcMs: nowMs,
        paramSiValue: widthNm,
      },
      nowMs,
    );
    this.pendingVolatileOps.set(count);
  }

  sealSelectionToCloudClipboard(): void {
    const nowMs = BigInt(Date.now());
    this.clipboardEnvelope.set(
      sealCloudClipboardEnvelope("cell-aurora-01", "usr_guest_0001", 1n, ["U1", "U2"], nowMs),
    );
  }
}
