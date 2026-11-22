// Module: web/libs/canvas-core/src/hud-overlay/hud-overlay.logic.spec.ts — 120 FPS HUD Projection Unit Tests
// Invariants: Verifies >2^53-1 bigint world-to-screen accuracy, length-tune delta tones, current-river particles, and SOA badges.
// Spec: architecture/06-frontend-angular-and-fastapi-control-plane.md
import { describe, expect, it } from "vitest";
import { projectHudPrimitives, screenPxToWorldNm, worldNmToScreenPx } from "./hud-overlay.logic";
import type { ViewportTransformNm } from "./hud-overlay.model";

describe("projectHudPrimitives & bigint coordinate transforms", () => {
  const transform: ViewportTransformNm = {
    panOriginXNm: 9_007_199_254_740_000n,
    panOriginYNm: -9_007_199_254_740_000n,
    nmPerPixel: 1_000n,
    viewportWidthPx: 1280,
    viewportHeightPx: 720,
  };

  it("projects and inverts >2^53-1 nm coordinates without IEEE-754 truncation", () => {
    const px = worldNmToScreenPx(9_007_199_254_940_000n, -9_007_199_254_340_000n, transform);
    expect(px).toEqual({ xPx: 200, yPx: 400 });
    const back = screenPxToWorldNm(200, 400, transform);
    expect(back).toEqual({ xNm: 9_007_199_254_940_000n, yNm: -9_007_199_254_340_000n });
  });

  it("projects length-tune gauges, current-river particles, and SOA smoke badges", () => {
    const projected = projectHudPrimitives(
      [
        {
          id: "len-1",
          kind: "LENGTH_TUNE_GAUGE",
          originXNm: 9_007_199_254_750_000n,
          originYNm: -9_007_199_254_700_000n,
          targetXNm: 9_007_199_254_870_000n,
          targetYNm: -9_007_199_254_700_000n,
          label: "DDR5_DQS0",
          targetLengthNm: 25_000_000n,
          actualLengthNm: 25_012_000n,
        },
        {
          id: "cur-1",
          kind: "CURRENT_RIVER_PARTICLE",
          originXNm: 9_007_199_254_740_000n,
          originYNm: -9_007_199_254_740_000n,
          targetXNm: 9_007_199_254_860_000n,
          targetYNm: -9_007_199_254_740_000n,
          label: "L_PHASE_U",
          currentNa: 4_500_000_000n,
        },
        {
          id: "soa-1",
          kind: "SOA_SMOKE_BADGE",
          originXNm: 9_007_199_254_800_000n,
          originYNm: -9_007_199_254_600_000n,
          targetXNm: 9_007_199_254_800_000n,
          targetYNm: -9_007_199_254_600_000n,
          label: "Q1_SIC",
          junctionTempMkelvin: 448_150n,
        },
      ],
      transform,
      20,
    );

    expect(projected[0].tone).toBe("ok");
    expect(projected[0].badgeText).toBe("DDR5_DQS0 ΔL=+12000 nm");
    expect(projected[1].particlePxX).toBe(60);
    expect(projected[2].tone).toBe("danger");
    expect(projected[2].badgeText).toBe("SOA Q1_SIC: Tj=175 °C");
  });
});
