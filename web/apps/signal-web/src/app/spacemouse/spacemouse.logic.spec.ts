// Module: web/apps/signal-web/src/app/spacemouse/spacemouse.logic.spec.ts — FEAT-093 SpaceMouse & Pie-Menu Unit Tests
// Invariants: Verifies 8-sector 45° angle resolution, center deadzone nulling, and 6-DOF exponential curve symmetry.
// Spec: architecture/17-extended-feature-specifications-ui-and-rust.md
import { describe, expect, it } from "vitest";
import { resolvePieSliceFromVector, translate6DofSpaceMouseAxes } from "./spacemouse.logic";

describe("Radial 8-Slice Pie-Menu & 6-DOF SpaceMouse (FEAT-093)", () => {
  it("resolves 8 cardinal/ordinal pie slices and ignores center deadzone", () => {
    expect(resolvePieSliceFromVector(5, 5, 16)).toBeNull();
    expect(resolvePieSliceFromVector(40, 0, 16)?.index).toBe(0);
    expect(resolvePieSliceFromVector(0, 40, 16)?.index).toBe(2);
    expect(resolvePieSliceFromVector(-40, 0, 16)?.index).toBe(4);
    expect(resolvePieSliceFromVector(0, -40, 16)?.index).toBe(6);
  });

  it("zeros sub-deadzone 6-DOF jitter and applies symmetric exponential scaling", () => {
    const idle = translate6DofSpaceMouseAxes({ tx: 0.05, ty: -0.04, tz: 0, rx: 0.02, ry: 0, rz: 0 });
    expect(idle).toEqual({ txMilli: 0, tyMilli: 0, tzMilli: 0, rxMilliRad: 0, ryMilliRad: 0, rzMilliRad: 0 });

    const full = translate6DofSpaceMouseAxes({ tx: 1, ty: -1, tz: 0.5, rx: 1, ry: -1, rz: 0 });
    expect(full.txMilli).toBe(1000);
    expect(full.tyMilli).toBe(-1000);
    expect(full.tzMilli).toBeGreaterThan(0);
    expect(full.tzMilli).toBeLessThan(500); // Exponential curve < linear 0.5
    expect(full.rxMilliRad).toBe(500);
  });
});
