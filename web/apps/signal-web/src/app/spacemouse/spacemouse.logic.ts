// Module: web/apps/signal-web/src/app/spacemouse/spacemouse.logic.ts — Radial 8-Slice Pie-Menu & WebHID 6-DOF Exponential Curve Math
// Invariants: Symmetric deadzone + exponential curve eliminates 6-DOF sensor zero-drift; 8 slices span 45° sectors.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type {
  RadialPieSlice,
  RawSpaceMouseAxes,
  SpaceMouseDeltaMilli,
  SpacemouseVm,
} from "./spacemouse.model";

export const RADIAL_PIE_SLICES: readonly RadialPieSlice[] = [
  { index: 0, label: "E • Place Part", commandId: "PLACE_PART", centerDeg: 0 },
  { index: 1, label: "SE • Route Wire", commandId: "ROUTE_WIRE", centerDeg: 45 },
  { index: 2, label: "S • Bus Bundle", commandId: "BUS_BUNDLE", centerDeg: 90 },
  { index: 3, label: "SW • Run SPICE", commandId: "RUN_SPICE", centerDeg: 135 },
  { index: 4, label: "W • Measure Cursor", commandId: "MEASURE_CURSOR", centerDeg: 180 },
  { index: 5, label: "NW • Pin/Gate Swap", commandId: "PIN_SWAP", centerDeg: 225 },
  { index: 6, label: "N • Auto-Beautify", commandId: "AUTO_BEAUTIFY", centerDeg: 270 },
  { index: 7, label: "NE • Variant DNP", commandId: "VARIANT_DNP", centerDeg: 315 },
];

export function resolvePieSliceFromVector(
  dxPx: number,
  dyPx: number,
  deadzonePx = 16,
): RadialPieSlice | null {
  const radius = Math.hypot(dxPx, dyPx);
  if (radius < deadzonePx) {
    return null;
  }
  const rawDeg = (Math.atan2(dyPx, dxPx) * 180) / Math.PI;
  const normDeg = ((rawDeg % 360) + 360) % 360;
  const idx = Math.floor(((normDeg + 22.5) % 360) / 45);
  return RADIAL_PIE_SLICES[idx] ?? RADIAL_PIE_SLICES[0];
}

function applyDeadzoneExponentialAxis(
  normInput: number,
  deadzone: number,
  exponent: number,
  scaleMilli: number,
): number {
  const clamped = Math.max(-1, Math.min(1, normInput));
  const mag = Math.abs(clamped);
  if (mag <= deadzone) {
    return 0;
  }
  const rescaled = (mag - deadzone) / (1 - deadzone);
  const curved = Math.pow(rescaled, exponent);
  return Math.round(Math.sign(clamped) * curved * scaleMilli);
}

export function translate6DofSpaceMouseAxes(
  raw: RawSpaceMouseAxes,
  deadzone = 0.08,
  exponent = 2.2,
): SpaceMouseDeltaMilli {
  return {
    txMilli: applyDeadzoneExponentialAxis(raw.tx, deadzone, exponent, 1000),
    tyMilli: applyDeadzoneExponentialAxis(raw.ty, deadzone, exponent, 1000),
    tzMilli: applyDeadzoneExponentialAxis(raw.tz, deadzone, exponent, 1000),
    rxMilliRad: applyDeadzoneExponentialAxis(raw.rx, deadzone, exponent, 500),
    ryMilliRad: applyDeadzoneExponentialAxis(raw.ry, deadzone, exponent, 500),
    rzMilliRad: applyDeadzoneExponentialAxis(raw.rz, deadzone, exponent, 500),
  };
}

export function buildSpacemouseVm(
  selectedSliceIndex: number | null,
  rawAxes: RawSpaceMouseAxes,
): SpacemouseVm {
  const activeSlice =
    selectedSliceIndex !== null ? (RADIAL_PIE_SLICES[selectedSliceIndex] ?? null) : null;
  const delta = translate6DofSpaceMouseAxes(rawAxes);
  return {
    slices: RADIAL_PIE_SLICES,
    activeSlice,
    delta,
    deltaBadge: `6-DOF ΔT=(${delta.txMilli}, ${delta.tyMilli}, ${delta.tzMilli}) mU | ΔR=(${delta.rxMilliRad}, ${delta.ryMilliRad}, ${delta.rzMilliRad}) mrad`,
  };
}
