// Module: web/libs/canvas-core/src/viewport/dual-plane-viewport.model.ts — Dual-Plane Zero-Leak Viewport Models
// Invariants: Coordinates stored strictly as ES2020 bigint nanometers; zero raw geometry persistence (F-126, F-144).
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/07-security-encryption-dlp-and-sftp.md
import type { ProjectedHudPrimitive } from "../hud-overlay/hud-overlay.model";

export interface ViewportReticleVm {
  readonly domainTitle: string;
  readonly planeModeLabel: string;
  readonly streamSessionBadge: string;
  readonly settlingBadge: string;
  readonly cursorXText: string;
  readonly cursorYText: string;
  readonly traceWidthText: string;
  readonly watermarkBadge: string;
  readonly volatileBufferBadge: string;
  readonly clipboardBadge: string;
  readonly hudSignals: readonly string[];
  readonly projectedHudPrimitives: readonly ProjectedHudPrimitive[];
}
