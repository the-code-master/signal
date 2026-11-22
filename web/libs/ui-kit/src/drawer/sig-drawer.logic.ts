// Module: web/libs/ui-kit/src/drawer/sig-drawer.logic.ts — Pure SigDrawer View-Model Builder
// Invariants: Pure TypeScript; zero DOM or Angular side effects.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigDrawerVm } from "./sig-drawer.model";

export function buildSigDrawerVm(open: boolean, title: string): SigDrawerVm {
  return { open, heading: title.trim() };
}
