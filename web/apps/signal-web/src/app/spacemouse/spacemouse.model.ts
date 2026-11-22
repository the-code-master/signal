// Module: web/apps/signal-web/src/app/spacemouse/spacemouse.model.ts — FEAT-093 8-Slice Radial Pie-Menu & WebHID 6-DOF SpaceMouse Models
// Invariants: 6-DOF outputs match SpaceMouse6DofDelta integer milli-units / milli-radians; deadzone prevents drift.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
export interface RadialPieSlice {
  readonly index: number;
  readonly label: string;
  readonly commandId: string;
  readonly centerDeg: number;
}

export interface RawSpaceMouseAxes {
  readonly tx: number;
  readonly ty: number;
  readonly tz: number;
  readonly rx: number;
  readonly ry: number;
  readonly rz: number;
}

export interface SpaceMouseDeltaMilli {
  readonly txMilli: number;
  readonly tyMilli: number;
  readonly tzMilli: number;
  readonly rxMilliRad: number;
  readonly ryMilliRad: number;
  readonly rzMilliRad: number;
}

export interface SpacemouseVm {
  readonly slices: readonly RadialPieSlice[];
  readonly activeSlice: RadialPieSlice | null;
  readonly delta: SpaceMouseDeltaMilli;
  readonly deltaBadge: string;
}
