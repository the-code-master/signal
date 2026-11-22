// Module: web/apps/signal-web/src/app/accessibility/accessibility.model.ts — FEAT-100 WCAG 2.2 AA Keyboard CAD & ARIA Netlist Models
// Invariants: Cursor steps strictly in ES2020 bigint nanometers (_nm); supports KiCad, Altium, Virtuoso & Vim presets.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/17-extended-feature-specifications-ui-and-rust.md
export type CadKeymapPreset = "KiCad" | "Altium" | "Virtuoso" | "Vim";

export type CadKeyAction =
  | "STEP_NORTH"
  | "STEP_SOUTH"
  | "STEP_EAST"
  | "STEP_WEST"
  | "TOGGLE_WIRE"
  | "ROTATE_90"
  | "PROBE_NET"
  | "NONE";

export interface AccessibleCursorState {
  readonly preset: CadKeymapPreset;
  readonly cursorXNm: bigint;
  readonly cursorYNm: bigint;
  readonly gridStepNm: bigint;
  readonly wireActive: boolean;
  readonly rotationDeg: number;
  readonly ariaAnnouncement: string;
}

export interface AccessibilityCadVm {
  readonly state: AccessibleCursorState;
  readonly presets: readonly CadKeymapPreset[];
  readonly positionBadge: string;
  readonly modeBadge: string;
  readonly keyHintBadge: string;
}
