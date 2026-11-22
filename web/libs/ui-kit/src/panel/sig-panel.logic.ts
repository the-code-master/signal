// Module: web/libs/ui-kit/src/panel/sig-panel.logic.ts — Pure SigPanel View-Model Builder
// Invariants: Pure TypeScript; computes header visibility flags outside HTML template.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import type { SigPanelVm } from "./sig-panel.model";

export function buildSigPanelVm(title: string, subtitle: string, badge: string): SigPanelVm {
  const cleanBadge = badge.trim();
  return {
    heading: title.trim(),
    subheading: subtitle.trim(),
    hasBadge: cleanBadge.length > 0,
    badgeText: cleanBadge,
  };
}
