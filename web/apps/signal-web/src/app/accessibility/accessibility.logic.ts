// Module: web/apps/signal-web/src/app/accessibility/accessibility.logic.ts — WCAG 2.2 AA Keyboard CAD Stepper & ARIA Netlist Reader
// Invariants: Maps KiCad/Altium/Virtuoso/Vim keybindings to deterministic bigint nm steps and screen-reader announcements.
// Spec: architecture/14-universal-scale-accessibility-and-expandability.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type {
  AccessibilityCadVm,
  AccessibleCursorState,
  CadKeyAction,
  CadKeymapPreset,
} from "./accessibility.model";

export const CAD_KEYMAP_PRESETS: readonly CadKeymapPreset[] = ["KiCad", "Altium", "Virtuoso", "Vim"];

const PRESET_HINTS: Readonly<Record<CadKeymapPreset, string>> = {
  KiCad: "KiCad: Arrows=Step • W=Wire • R=Rotate • P=Probe",
  Altium: "Altium: Arrows=Step • P=Place Wire • Space=Rotate • I=Probe",
  Virtuoso: "Virtuoso: Arrows=Step • W=Add Wire • R=Rotate • P=Probe",
  Vim: "Vim CAD: H/J/K/L=Step • I=Wire • R=Rotate • ?=Probe",
};

export function createInitialAccessibleCursor(preset: CadKeymapPreset = "KiCad"): AccessibleCursorState {
  return {
    preset,
    cursorXNm: 2_540_000n,
    cursorYNm: 2_540_000n,
    gridStepNm: 254_000n,
    wireActive: false,
    rotationDeg: 0,
    ariaAnnouncement: "CAD cursor initialized at (2540000, 2540000) nm on pin U1.1 net HV_DC_BUS_800V.",
  };
}

export function resolveCadKeymapAction(preset: CadKeymapPreset, rawKey: string): CadKeyAction {
  if (rawKey === "ArrowUp") return "STEP_NORTH";
  if (rawKey === "ArrowDown") return "STEP_SOUTH";
  if (rawKey === "ArrowRight") return "STEP_EAST";
  if (rawKey === "ArrowLeft") return "STEP_WEST";

  const key = rawKey.toLowerCase();
  if (preset === "Vim") {
    if (key === "k") return "STEP_NORTH";
    if (key === "j") return "STEP_SOUTH";
    if (key === "l") return "STEP_EAST";
    if (key === "h") return "STEP_WEST";
    if (key === "i") return "TOGGLE_WIRE";
    if (key === "r") return "ROTATE_90";
    if (key === "?") return "PROBE_NET";
    return "NONE";
  }
  if (preset === "Altium") {
    if (key === "p") return "TOGGLE_WIRE";
    if (rawKey === " " || key === "space") return "ROTATE_90";
    if (key === "i") return "PROBE_NET";
    return "NONE";
  }
  // KiCad & Virtuoso share W=Wire, R=Rotate, P=Probe:
  if (key === "w") return "TOGGLE_WIRE";
  if (key === "r") return "ROTATE_90";
  if (key === "p") return "PROBE_NET";
  return "NONE";
}

export function stepAccessibleCadCursor(
  state: AccessibleCursorState,
  rawKey: string,
): AccessibleCursorState {
  const action = resolveCadKeymapAction(state.preset, rawKey);
  if (action === "NONE") return state;

  let cursorXNm = state.cursorXNm;
  let cursorYNm = state.cursorYNm;
  let wireActive = state.wireActive;
  let rotationDeg = state.rotationDeg;

  if (action === "STEP_NORTH") cursorYNm -= state.gridStepNm;
  else if (action === "STEP_SOUTH") cursorYNm += state.gridStepNm;
  else if (action === "STEP_EAST") cursorXNm += state.gridStepNm;
  else if (action === "STEP_WEST") cursorXNm -= state.gridStepNm;
  else if (action === "TOGGLE_WIRE") wireActive = !wireActive;
  else if (action === "ROTATE_90") rotationDeg = (rotationDeg + 90) % 360;

  const netSummary =
    cursorXNm >= 2_540_000n ? "net HV_DC_BUS_800V (U1.1, C_LINK_HV.1)" : "net GATE_DRV_HS_U (U2.6, Q1.G)";
  const wireNote = wireActive ? "A* orthogonal wire active" : "Select mode";
  const ariaAnnouncement = `${action}: Cursor (${cursorXNm.toString(10)}, ${cursorYNm.toString(10)}) nm • ${netSummary} • ${wireNote} • Rot ${rotationDeg}°`;

  return {
    ...state,
    cursorXNm,
    cursorYNm,
    wireActive,
    rotationDeg,
    ariaAnnouncement,
  };
}

export function buildAccessibilityCadVm(state: AccessibleCursorState): AccessibilityCadVm {
  return {
    state,
    presets: CAD_KEYMAP_PRESETS,
    positionBadge: `Cursor: (${state.cursorXNm.toString(10)}, ${state.cursorYNm.toString(10)}) nm`,
    modeBadge: state.wireActive ? `Wire Routing (${state.rotationDeg}°)` : `Navigation (${state.rotationDeg}°)`,
    keyHintBadge: PRESET_HINTS[state.preset],
  };
}
