// Module: web/libs/canvas-core/src/viewport/dual-plane-viewport.logic.ts — Pure Dual-Plane Codec & BigInt HUD Formatter
// Invariants: Formats >2^53-1 nm coordinates with bigint math; selects Plane 1 Zstd-QOI vs Plane 2 HW video (F-126).
// Spec: architecture/07-security-encryption-dlp-and-sftp.md, architecture/16-architectural-audit-flaws-and-futuristic-innovations.md
import type { ViewportReticleVm } from "./dual-plane-viewport.model";

export function buildViewportReticleVm(
  domainTitle: string,
  cursorXNm: bigint,
  cursorYNm: bigint,
  traceWidthNm: bigint,
  orbiting3d: boolean,
  hudSignals: readonly string[],
): ViewportReticleVm {
  const planeModeLabel = orbiting3d
    ? "Plane 2: HW HEVC 4:4:4 / Dual-Luma 120 FPS"
    : "Plane 1: Lossless Zstd-QOI RGBA8 Dirty Tiles";
  return {
    domainTitle,
    planeModeLabel,
    cursorXText: `X: ${cursorXNm.toString(10)} nm`,
    cursorYText: `Y: ${cursorYNm.toString(10)} nm`,
    traceWidthText: `Constraint: ${traceWidthNm.toString(10)} nm`,
    watermarkBadge: "Zero-Leak DLP • Tardos + Constellation Active",
    hudSignals,
  };
}
