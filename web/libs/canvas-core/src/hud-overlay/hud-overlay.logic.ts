// Module: web/libs/canvas-core/src/hud-overlay/hud-overlay.logic.ts — BigInt World-to-Screen Projection & 120 FPS HUD Math
// Invariants: Computes exact bigint nanometer deltas before dividing by nmPerPixel so >2^53-1 nm origins never overflow IEEE-754.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigStatusTone } from "@signal/ui-kit";
import type { HudWorldPrimitive, ProjectedHudPrimitive, ViewportTransformNm } from "./hud-overlay.model";

const CLAMP_SCREEN_BOUND_PX = 65_536n;

function clampBigIntToPx(pxBig: bigint): number {
  if (pxBig > CLAMP_SCREEN_BOUND_PX) return 65_536;
  if (pxBig < -CLAMP_SCREEN_BOUND_PX) return -65_536;
  return Number(pxBig);
}

export function worldNmToScreenPx(
  worldXNm: bigint,
  worldYNm: bigint,
  transform: ViewportTransformNm,
): { readonly xPx: number; readonly yPx: number } {
  const scale = transform.nmPerPixel > 0n ? transform.nmPerPixel : 1n;
  const dxPx = (worldXNm - transform.panOriginXNm) / scale;
  const dyPx = (worldYNm - transform.panOriginYNm) / scale;
  return { xPx: clampBigIntToPx(dxPx), yPx: clampBigIntToPx(dyPx) };
}

export function screenPxToWorldNm(
  screenXPx: number,
  screenYPx: number,
  transform: ViewportTransformNm,
): { readonly xNm: bigint; readonly yNm: bigint } {
  const scale = transform.nmPerPixel > 0n ? transform.nmPerPixel : 1n;
  return {
    xNm: transform.panOriginXNm + BigInt(Math.round(screenXPx)) * scale,
    yNm: transform.panOriginYNm + BigInt(Math.round(screenYPx)) * scale,
  };
}

function formatHudBadge(item: HudWorldPrimitive): { readonly badgeText: string; readonly tone: SigStatusTone } {
  if (item.kind === "LENGTH_TUNE_GAUGE" && item.targetLengthNm !== undefined && item.actualLengthNm !== undefined) {
    const deltaNm = item.actualLengthNm - item.targetLengthNm;
    const absDeltaNm = deltaNm < 0n ? -deltaNm : deltaNm;
    const sign = deltaNm >= 0n ? "+" : "";
    const tone: SigStatusTone = absDeltaNm <= 25_000n ? "ok" : absDeltaNm <= 150_000n ? "warn" : "danger";
    return { badgeText: `${item.label} ΔL=${sign}${deltaNm.toString(10)} nm`, tone };
  }
  if (item.kind === "SOA_SMOKE_BADGE" && item.junctionTempMkelvin !== undefined) {
    const celsiusInt = (item.junctionTempMkelvin - 273_150n) / 1_000n;
    const tone: SigStatusTone = celsiusInt >= 150n ? "danger" : celsiusInt >= 115n ? "warn" : "ok";
    return { badgeText: `SOA ${item.label}: Tj=${celsiusInt.toString(10)} °C`, tone };
  }
  if (item.kind === "CURRENT_RIVER_PARTICLE" && item.currentNa !== undefined) {
    const milliAmps = item.currentNa / 1_000_000n;
    return { badgeText: `${item.label}: ${milliAmps.toString(10)} mA`, tone: "ok" };
  }
  return { badgeText: item.label, tone: "neutral" };
}

export function projectHudPrimitives(
  primitives: readonly HudWorldPrimitive[],
  transform: ViewportTransformNm,
  frameTick120Hz: number,
): readonly ProjectedHudPrimitive[] {
  const marginPx = 64;
  return primitives.map((item) => {
    const p1 = worldNmToScreenPx(item.originXNm, item.originYNm, transform);
    const p2 = worldNmToScreenPx(item.targetXNm, item.targetYNm, transform);
    const speedFactor = item.currentNa !== undefined && item.currentNa > 1_000_000_000n ? 3 : 1;
    const phase = (((frameTick120Hz * speedFactor) % 120) + 120) % 120 / 120;
    const particlePxX = Math.round(p1.xPx + (p2.xPx - p1.xPx) * phase);
    const particlePxY = Math.round(p1.yPx + (p2.yPx - p1.yPx) * phase);
    const visible =
      Math.max(p1.xPx, p2.xPx) >= -marginPx &&
      Math.min(p1.xPx, p2.xPx) <= transform.viewportWidthPx + marginPx &&
      Math.max(p1.yPx, p2.yPx) >= -marginPx &&
      Math.min(p1.yPx, p2.yPx) <= transform.viewportHeightPx + marginPx;
    const { badgeText, tone } = formatHudBadge(item);
    return {
      id: item.id,
      kind: item.kind,
      x1Px: p1.xPx,
      y1Px: p1.yPx,
      x2Px: p2.xPx,
      y2Px: p2.yPx,
      particlePxX,
      particlePxY,
      visible,
      badgeText,
      tone,
    };
  });
}
