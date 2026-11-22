// Module: web/libs/canvas-core/src/hud-overlay/hud-overlay.model.ts — 120 FPS Zero-Leak HudOverlayPrimitive Models
// Invariants: World coordinates stay in ES2020 bigint nanometers (_nm); browser renders vector chrome without raw IP netlists.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";

export type HudOverlayKind =
  | "SNAP_CROSSHAIR"
  | "RUBBERBAND_WIRE"
  | "LENGTH_TUNE_GAUGE"
  | "CURRENT_RIVER_PARTICLE"
  | "SOA_SMOKE_BADGE";

export interface ViewportTransformNm {
  readonly panOriginXNm: bigint;
  readonly panOriginYNm: bigint;
  readonly nmPerPixel: bigint;
  readonly viewportWidthPx: number;
  readonly viewportHeightPx: number;
}

export interface HudWorldPrimitive {
  readonly id: string;
  readonly kind: HudOverlayKind;
  readonly originXNm: bigint;
  readonly originYNm: bigint;
  readonly targetXNm: bigint;
  readonly targetYNm: bigint;
  readonly label: string;
  readonly targetLengthNm?: bigint;
  readonly actualLengthNm?: bigint;
  readonly currentNa?: bigint;
  readonly junctionTempMkelvin?: bigint;
}

export interface ProjectedHudPrimitive {
  readonly id: string;
  readonly kind: HudOverlayKind;
  readonly x1Px: number;
  readonly y1Px: number;
  readonly x2Px: number;
  readonly y2Px: number;
  readonly particlePxX: number;
  readonly particlePxY: number;
  readonly visible: boolean;
  readonly badgeText: string;
  readonly tone: SigStatusTone;
}
